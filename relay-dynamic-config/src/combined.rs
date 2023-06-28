use itertools::Itertools;
use relay_auth::PublicKey;
use relay_filter::FiltersConfig;
use relay_general::pii::{DataScrubbingConfig, PiiConfig};
use relay_general::store::{
    BreakdownsConfig, BuiltinMeasurementKey, MeasurementsConfig, SpanDescriptionRule,
    TransactionNameRule,
};
use relay_general::types::SpanAttribute;
use relay_quotas::Quota;
use relay_sampling::SamplingConfig;
use serde_json::Value;

use crate::{
    ErrorBoundary, Feature, ProjectConfig, SessionMetricsConfig, TaggingRule,
    TransactionMetricsConfig, DEFAULT_ALLOWED_DOMAINS,
};

pub struct DynamicConfig {
    global: ProjectConfig,
    organization: ProjectConfig,
    project: ProjectConfig,
    public_key: ProjectConfig,
}

impl DynamicConfig {
    pub fn allowed_domains(&self) -> impl Iterator<Item = &str> {
        // TODO: double-check that overwriting is the behavior that we want.
        let config = [&self.public_key, &self.project, &self.organization]
            .into_iter()
            .find(|slice| slice.allowed_domains.as_slice() != DEFAULT_ALLOWED_DOMAINS)
            .unwrap_or(&self.global);

        config.allowed_domains.iter().map(String::as_str)
    }

    pub fn trusted_relays(&self) -> impl Iterator<Item = &PublicKey> {
        self.iter().flat_map(|c| c.trusted_relays.iter()).unique()
    }

    pub fn pii_config(&self) -> Option<PiiConfig> {
        todo!()
    }

    /// The grouping configuration.
    pub fn grouping_config(&self) -> &Option<Value> {
        // Grouping config is opaque so we cannot merge it easily.
        // Assume that grouping will be per-project for the foreseeable future.
        // See https://github.com/getsentry/sentry/blob/254cfc0bd2f13dd794ea5bce43c0f77c217eecda/src/sentry/relay/config/__init__.py#L407-L409.
        &self.project.grouping_config
    }

    /// Configuration for filter rules.
    pub fn filter_settings(&self) -> &FiltersConfig {
        // To decide:
        // Do we want to define e.g.
        //   browser_extensions.is_enabled := any(scope.browser_extensions.is_enabled)
        // or make it a tri-state per scope and let lower levels override higher levels?
        todo!()
    }

    /// Configuration for data scrubbers.
    pub fn datascrubbing_settings(&self) -> &DataScrubbingConfig {
        todo!()
    }

    /// Maximum event retention for the organization.
    pub fn event_retention(&self) -> Option<u16> {
        self.ascend_from_local() // Use the most local value
            .filter_map(|c| c.event_retention)
            .next()
    }

    /// Usage quotas for this project.
    pub fn quotas(&self) -> impl Iterator<Item = &Quota> {
        // Order of quotas does not matter semantically.
        // Use the same order as sentry does for easier diffing.
        // See https://github.com/getsentry/sentry/blob/7ef1718552effcdf2de5f664882fcb395841ef9f/src/sentry/quotas/redis.py#L52-L112
        [
            &self.project,
            &self.organization,
            &self.public_key,
            &self.global, // should be empty
        ]
        .into_iter()
        .flat_map(|c| c.quotas.iter())
    }

    /// Configuration for sampling traces, if not present there will be no sampling.
    pub fn dynamic_sampling(&self) -> &Option<SamplingConfig> {
        todo!()
    }

    /// A list of measurements that are built-in and are not subject to custom measurement limits.
    pub fn builtin_measurements(&self) -> impl Iterator<Item = &BuiltinMeasurementKey> {
        // Combine builtin measurements from all scopes.
        let configs = self.iter().flat_map(|c| c.measurements.iter());
        configs.flat_map(|m| m.builtin_measurements.iter()).unique()
    }

    /// The maximum number of measurements allowed per event that are not known measurements.
    pub fn max_custom_measurements(&self) -> Option<usize> {
        // Take the most local setting.
        self.ascend_from_local()
            .find_map(|c| c.measurements.as_ref().map(|m| m.max_custom_measurements))
    }

    /// Configuration for operation breakdown. Will be emitted only if present.
    pub fn breakdowns(&self) -> &Option<BreakdownsConfig> {
        todo!()
    }

    /// Configuration for extracting metrics from sessions.
    pub fn session_metrics(&self) -> SessionMetricsConfig {
        // Session metrics struct is small enough to create a new instance on the fly.
        // Reconsider if this config grows.
        SessionMetricsConfig {
            // Enforce the highest version that is being required:
            version: self
                .iter()
                .map(|c| c.session_metrics.version)
                .max()
                .unwrap_or_default(),
            // If any
            drop: self.iter().any(|c| c.session_metrics.drop),
        }
    }

    /// Configuration for extracting metrics from transaction events.
    pub fn transaction_metrics(&self) -> &Option<ErrorBoundary<TransactionMetricsConfig>> {
        todo!()
    }

    /// The span attributes configuration.
    pub fn span_attributes(&self) -> impl Iterator<Item = &SpanAttribute> {
        // Combine span attributes from all slices.
        self.ascend_from_local()
            .flat_map(|c| c.span_attributes.iter())
            .unique()
    }

    /// Rules for applying metrics tags depending on the event's content.
    pub fn metric_conditional_tagging(&self) -> &Vec<TaggingRule> {
        // combine all, but think carefully about precedence.
        todo!()
    }

    /// Exposable features enabled for this project.
    pub fn features(&self) -> impl Iterator<Item = &Feature> {
        self.ascend_from_local()
            .flat_map(|c| c.features.0.iter())
            .unique()
    }

    /// Transaction renaming rules.
    pub fn tx_name_rules(&self) -> impl Iterator<Item = &TransactionNameRule> {
        // The first matching transaction name rule wins, so iterate from local to global
        // to give preference to project-specific rules:
        self.ascend_from_local()
            .flat_map(|c| c.tx_name_rules.iter())
    }

    /// Whether or not a project is ready to mark all URL transactions as "sanitized".
    pub fn tx_name_ready(&self) -> bool {
        // Deprecated feature flag, still serialized for external Relays.
        self.iter().any(|c| c.tx_name_ready)
    }

    /// Span description renaming rules.
    pub fn span_description_rules(&self) -> impl Iterator<Item = &SpanDescriptionRule> {
        // combine all, but think carefully about precedence.
        // The first matching transaction name rule wins, so iterate from local to global
        // to give preference to project-specific rules:
        self.ascend_from_local()
            .flat_map(|c| c.span_description_rules.iter().flatten())
    }

    /// Iterate from most local scope to most global scope.
    fn ascend_from_local(&self) -> std::array::IntoIter<&ProjectConfig, 4> {
        [
            &self.public_key,
            &self.project,
            &self.organization,
            &self.global,
        ]
        .into_iter()
    }

    /// Iterate from most global scope.
    fn descend_from_global(&self) -> std::array::IntoIter<&ProjectConfig, 4> {
        [
            &self.global,
            &self.organization,
            &self.project,
            &self.public_key,
        ]
        .into_iter()
    }

    /// Iterate over scopes in any order.
    ///
    /// This is a convenience wrapper to clarify intent on the caller side when the order of
    /// scopes does not matter.
    fn iter(&self) -> impl Iterator<Item = &ProjectConfig> {
        self.ascend_from_local()
    }
}

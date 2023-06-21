//! Dynamic configuration for metrics extraction from sessions and transactions.

use std::collections::BTreeSet;

use relay_sampling::RuleCondition;
use serde::{Deserialize, Serialize};

/// Rule defining when a target tag should be set on a metric.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(deepsize::DeepSizeOf)]
pub struct TaggingRule {
    // note: could add relay_sampling::RuleType here, but right now we only support transaction
    // events
    /// Condition that defines when to set the tag.
    pub condition: RuleCondition,
    /// Metrics on which the tag is set.
    pub target_metrics: BTreeSet<String>,
    /// Name of the tag that is set.
    pub target_tag: String,
    /// Value of the tag that is set.
    pub tag_value: String,
}

/// Current version of metrics extraction.
const SESSION_EXTRACT_VERSION: u16 = 3;
const EXTRACT_ABNORMAL_MECHANISM_VERSION: u16 = 2;

/// Configuration for metric extraction from sessions.
#[derive(Debug, Clone, Copy, Default, serde::Deserialize, serde::Serialize)]
#[serde(default, rename_all = "camelCase")]
pub struct SessionMetricsConfig {
    /// The revision of the extraction algorithm.
    ///
    /// Provided the revision is lower than or equal to the revision supported by this Relay,
    /// metrics are extracted. If the revision is higher than what this Relay supports, it does not
    /// extract metrics from sessions, and instead forwards them to the upstream.
    ///
    /// Version `0` (default) disables extraction.
    version: u16,

    /// Drop sessions after successfully extracting metrics.
    drop: bool,
}

impl SessionMetricsConfig {
    /// Returns `true` if session metrics is enabled and compatible.
    pub fn is_enabled(&self) -> bool {
        self.version > 0 && self.version <= SESSION_EXTRACT_VERSION
    }

    /// Returns `true` if Relay should not extract metrics from sessions.
    pub fn is_disabled(&self) -> bool {
        !self.is_enabled()
    }

    /// Whether or not the abnormal mechanism should be extracted as a tag.
    pub fn should_extract_abnormal_mechanism(&self) -> bool {
        self.version >= EXTRACT_ABNORMAL_MECHANISM_VERSION
    }

    /// Returns `true` if the session should be dropped after extracting metrics.
    pub fn should_drop(&self) -> bool {
        self.drop
    }
}

/// Configuration for extracting custom measurements from transaction payloads.
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct CustomMeasurementConfig {
    /// The maximum number of custom measurements to extract. Defaults to zero.
    limit: usize,
}

/// Maximum supported version of metrics extraction from transactions.
///
/// The version is an integer scalar, incremented by one on each new version.
const TRANSACTION_EXTRACT_VERSION: u16 = 1;

/// Defines whether URL transactions should be considered low cardinality.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum AcceptTransactionNames {
    /// For some SDKs, accept all transaction names, while for others, apply strict rules.
    ClientBased,

    /// Only accept transaction names with a low-cardinality source.
    /// Any value other than "clientBased" will be interpreted as "strict".
    #[serde(other)]
    Strict,
}

impl Default for AcceptTransactionNames {
    fn default() -> Self {
        Self::Strict
    }
}

/// Configuration for extracting metrics from transaction payloads.
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct TransactionMetricsConfig {
    /// The required version to extract transaction metrics.
    pub version: u16,
    /// Custom event tags that are transferred from the transaction to metrics.
    pub extract_custom_tags: BTreeSet<String>,
    /// Deprecated in favor of top-level config field. Still here to be forwarded to external relays.
    pub custom_measurements: CustomMeasurementConfig,
    /// Defines whether URL transactions should be considered low cardinality.
    pub accept_transaction_names: AcceptTransactionNames,
}

impl TransactionMetricsConfig {
    /// Creates an enabled configuration with empty defaults.
    pub fn new() -> Self {
        Self {
            version: 1,
            ..Self::default()
        }
    }

    /// Returns `true` if metrics extraction is enabled and compatible with this Relay.
    pub fn is_enabled(&self) -> bool {
        self.version > 0 && self.version <= TRANSACTION_EXTRACT_VERSION
    }
}

#[cfg(test)]
mod tests {
    use deepsize::DeepSizeOf;

    use super::*;

    #[test]
    fn taggingrulesize() {
        let rules = r#"[
            {
                "condition": {
                    "op": "and",
                    "inner": [
                        {
                            "op": "gt",
                            "name": "event.duration",
                            "value": 1200
                        }
                    ]
                },
                "targetMetrics": [
                    "s:transactions/user@none",
                    "d:transactions/duration@millisecond",
                    "d:transactions/measurements.lcp@millisecond"
                ],
                "targetTag": "satisfaction",
                "tagValue": "frustrated"
            },
            {
                "condition": {
                    "op": "and",
                    "inner": [
                        {
                            "op": "gt",
                            "name": "event.duration",
                            "value": 300
                        }
                    ]
                },
                "targetMetrics": [
                    "s:transactions/user@none",
                    "d:transactions/duration@millisecond",
                    "d:transactions/measurements.lcp@millisecond"
                ],
                "targetTag": "satisfaction",
                "tagValue": "tolerated"
            },
            {
                "condition": {
                    "op": "and",
                    "inner": []
                },
                "targetMetrics": [
                    "s:transactions/user@none",
                    "d:transactions/duration@millisecond",
                    "d:transactions/measurements.lcp@millisecond"
                ],
                "targetTag": "satisfaction",
                "tagValue": "satisfied"
            },
            {
                "condition": {
                    "op": "and",
                    "inner": [
                        {
                            "op": "eq",
                            "name": "event.contexts.trace.op",
                            "value": "pageload"
                        },
                        {
                            "op": "eq",
                            "name": "event.platform",
                            "value": "javascript"
                        },
                        {
                            "op": "gte",
                            "name": "event.duration",
                            "value": 16123.0
                        }
                    ]
                },
                "targetMetrics": [
                    "d:transactions/duration@millisecond"
                ],
                "targetTag": "histogram_outlier",
                "tagValue": "outlier"
            },
            {
                "condition": {
                    "op": "and",
                    "inner": [
                        {
                            "op": "eq",
                            "name": "event.contexts.trace.op",
                            "value": "pageload"
                        },
                        {
                            "op": "eq",
                            "name": "event.platform",
                            "value": "javascript"
                        },
                        {
                            "op": "gte",
                            "name": "event.duration",
                            "value": 7941.899538040161
                        }
                    ]
                },
                "targetMetrics": [
                    "d:transactions/measurements.lcp@millisecond"
                ],
                "targetTag": "histogram_outlier",
                "tagValue": "outlier"
            },
            {
                "condition": {
                    "op": "and",
                    "inner": [
                        {
                            "op": "eq",
                            "name": "event.contexts.trace.op",
                            "value": "pageload"
                        },
                        {
                            "op": "eq",
                            "name": "event.platform",
                            "value": "javascript"
                        },
                        {
                            "op": "gte",
                            "name": "event.duration",
                            "value": 5897.500002294778
                        }
                    ]
                },
                "targetMetrics": [
                    "d:transactions/measurements.fcp@millisecond"
                ],
                "targetTag": "histogram_outlier",
                "tagValue": "outlier"
            },
            {
                "condition": {
                    "op": "and",
                    "inner": [
                        {
                            "op": "eq",
                            "name": "event.contexts.trace.op",
                            "value": "navigation"
                        },
                        {
                            "op": "eq",
                            "name": "event.platform",
                            "value": "javascript"
                        },
                        {
                            "op": "gte",
                            "name": "event.duration",
                            "value": 4032.0
                        }
                    ]
                },
                "targetMetrics": [
                    "d:transactions/duration@millisecond"
                ],
                "targetTag": "histogram_outlier",
                "tagValue": "outlier"
            },
            {
                "condition": {
                    "op": "and",
                    "inner": [
                        {
                            "op": "eq",
                            "name": "event.contexts.trace.op",
                            "value": "http.server"
                        },
                        {
                            "op": "eq",
                            "name": "event.platform",
                            "value": "python"
                        },
                        {
                            "op": "gte",
                            "name": "event.duration",
                            "value": 383.0
                        }
                    ]
                },
                "targetMetrics": [
                    "d:transactions/duration@millisecond"
                ],
                "targetTag": "histogram_outlier",
                "tagValue": "outlier"
            },
            {
                "condition": {
                    "op": "and",
                    "inner": [
                        {
                            "op": "eq",
                            "name": "event.contexts.trace.op",
                            "value": "http.server"
                        },
                        {
                            "op": "eq",
                            "name": "event.platform",
                            "value": "node"
                        },
                        {
                            "op": "gte",
                            "name": "event.duration",
                            "value": 506.0
                        }
                    ]
                },
                "targetMetrics": [
                    "d:transactions/duration@millisecond"
                ],
                "targetTag": "histogram_outlier",
                "tagValue": "outlier"
            },
            {
                "condition": {
                    "op": "and",
                    "inner": [
                        {
                            "op": "eq",
                            "name": "event.contexts.trace.op",
                            "value": "http.server"
                        },
                        {
                            "op": "eq",
                            "name": "event.platform",
                            "value": "php"
                        },
                        {
                            "op": "gte",
                            "name": "event.duration",
                            "value": 891.0
                        }
                    ]
                },
                "targetMetrics": [
                    "d:transactions/duration@millisecond"
                ],
                "targetTag": "histogram_outlier",
                "tagValue": "outlier"
            },
            {
                "condition": {
                    "op": "and",
                    "inner": [
                        {
                            "op": "eq",
                            "name": "event.contexts.trace.op",
                            "value": "ui.load"
                        },
                        {
                            "op": "eq",
                            "name": "event.platform",
                            "value": "javascript"
                        },
                        {
                            "op": "gte",
                            "name": "event.duration",
                            "value": 199379.0
                        }
                    ]
                },
                "targetMetrics": [
                    "d:transactions/duration@millisecond"
                ],
                "targetTag": "histogram_outlier",
                "tagValue": "outlier"
            },
            {
                "condition": {
                    "op": "and",
                    "inner": [
                        {
                            "op": "eq",
                            "name": "event.contexts.trace.op",
                            "value": "celery.task"
                        },
                        {
                            "op": "eq",
                            "name": "event.platform",
                            "value": "python"
                        },
                        {
                            "op": "gte",
                            "name": "event.duration",
                            "value": 1516.0
                        }
                    ]
                },
                "targetMetrics": [
                    "d:transactions/duration@millisecond"
                ],
                "targetTag": "histogram_outlier",
                "tagValue": "outlier"
            },
            {
                "condition": {
                    "op": "and",
                    "inner": [
                        {
                            "op": "eq",
                            "name": "event.contexts.trace.op",
                            "value": "rails.request"
                        },
                        {
                            "op": "eq",
                            "name": "event.platform",
                            "value": "ruby"
                        },
                        {
                            "op": "gte",
                            "name": "event.duration",
                            "value": 407.0
                        }
                    ]
                },
                "targetMetrics": [
                    "d:transactions/duration@millisecond"
                ],
                "targetTag": "histogram_outlier",
                "tagValue": "outlier"
            },
            {
                "condition": {
                    "op": "and",
                    "inner": [
                        {
                            "op": "eq",
                            "name": "event.contexts.trace.op",
                            "value": "queue.task.celery"
                        },
                        {
                            "op": "eq",
                            "name": "event.platform",
                            "value": "python"
                        },
                        {
                            "op": "gte",
                            "name": "event.duration",
                            "value": 2637.0
                        }
                    ]
                },
                "targetMetrics": [
                    "d:transactions/duration@millisecond"
                ],
                "targetTag": "histogram_outlier",
                "tagValue": "outlier"
            },
            {
                "condition": {
                    "op": "and",
                    "inner": [
                        {
                            "op": "eq",
                            "name": "event.contexts.trace.op",
                            "value": "function.nextjs"
                        },
                        {
                            "op": "eq",
                            "name": "event.platform",
                            "value": "node"
                        },
                        {
                            "op": "gte",
                            "name": "event.duration",
                            "value": 505.0
                        }
                    ]
                },
                "targetMetrics": [
                    "d:transactions/duration@millisecond"
                ],
                "targetTag": "histogram_outlier",
                "tagValue": "outlier"
            },
            {
                "condition": {
                    "op": "and",
                    "inner": [
                        {
                            "op": "eq",
                            "name": "event.contexts.trace.op",
                            "value": "ui.load"
                        },
                        {
                            "op": "eq",
                            "name": "event.platform",
                            "value": "cocoa"
                        },
                        {
                            "op": "gte",
                            "name": "event.duration",
                            "value": 2387.0
                        }
                    ]
                },
                "targetMetrics": [
                    "d:transactions/duration@millisecond"
                ],
                "targetTag": "histogram_outlier",
                "tagValue": "outlier"
            },
            {
                "condition": {
                    "op": "and",
                    "inner": [
                        {
                            "op": "eq",
                            "name": "event.contexts.trace.op",
                            "value": "http.server"
                        },
                        {
                            "op": "eq",
                            "name": "event.platform",
                            "value": "csharp"
                        },
                        {
                            "op": "gte",
                            "name": "event.duration",
                            "value": 325.0
                        }
                    ]
                },
                "targetMetrics": [
                    "d:transactions/duration@millisecond"
                ],
                "targetTag": "histogram_outlier",
                "tagValue": "outlier"
            },
            {
                "condition": {
                    "op": "and",
                    "inner": [
                        {
                            "op": "eq",
                            "name": "event.contexts.trace.op",
                            "value": "http.server"
                        },
                        {
                            "op": "eq",
                            "name": "event.platform",
                            "value": "ruby"
                        },
                        {
                            "op": "gte",
                            "name": "event.duration",
                            "value": 347.0
                        }
                    ]
                },
                "targetMetrics": [
                    "d:transactions/duration@millisecond"
                ],
                "targetTag": "histogram_outlier",
                "tagValue": "outlier"
            },
            {
                "condition": {
                    "op": "and",
                    "inner": [
                        {
                            "op": "eq",
                            "name": "event.contexts.trace.op",
                            "value": "ui.load"
                        },
                        {
                            "op": "eq",
                            "name": "event.platform",
                            "value": "java"
                        },
                        {
                            "op": "gte",
                            "name": "event.duration",
                            "value": 2889.0
                        }
                    ]
                },
                "targetMetrics": [
                    "d:transactions/duration@millisecond"
                ],
                "targetTag": "histogram_outlier",
                "tagValue": "outlier"
            },
            {
                "condition": {
                    "op": "and",
                    "inner": [
                        {
                            "op": "eq",
                            "name": "event.contexts.trace.op",
                            "value": "http.server"
                        },
                        {
                            "op": "eq",
                            "name": "event.platform",
                            "value": "java"
                        },
                        {
                            "op": "gte",
                            "name": "event.duration",
                            "value": 246.0
                        }
                    ]
                },
                "targetMetrics": [
                    "d:transactions/duration@millisecond"
                ],
                "targetTag": "histogram_outlier",
                "tagValue": "outlier"
            },
            {
                "condition": {
                    "op": "and",
                    "inner": [
                        {
                            "op": "eq",
                            "name": "event.contexts.trace.op",
                            "value": "awslambda.handler"
                        },
                        {
                            "op": "eq",
                            "name": "event.platform",
                            "value": "node"
                        },
                        {
                            "op": "gte",
                            "name": "event.duration",
                            "value": 1747.0
                        }
                    ]
                },
                "targetMetrics": [
                    "d:transactions/duration@millisecond"
                ],
                "targetTag": "histogram_outlier",
                "tagValue": "outlier"
            },
            {
                "condition": {
                    "op": "and",
                    "inner": [
                        {
                            "op": "eq",
                            "name": "event.contexts.trace.op",
                            "value": "serverless.function"
                        },
                        {
                            "op": "eq",
                            "name": "event.platform",
                            "value": "python"
                        },
                        {
                            "op": "gte",
                            "name": "event.duration",
                            "value": 393.0
                        }
                    ]
                },
                "targetMetrics": [
                    "d:transactions/duration@millisecond"
                ],
                "targetTag": "histogram_outlier",
                "tagValue": "outlier"
            },
            {
                "condition": {
                    "op": "and",
                    "inner": [
                        {
                            "op": "eq",
                            "name": "event.contexts.trace.op",
                            "value": "function.aws.lambda"
                        },
                        {
                            "op": "eq",
                            "name": "event.platform",
                            "value": "node"
                        },
                        {
                            "op": "gte",
                            "name": "event.duration",
                            "value": 1633.0
                        }
                    ]
                },
                "targetMetrics": [
                    "d:transactions/duration@millisecond"
                ],
                "targetTag": "histogram_outlier",
                "tagValue": "outlier"
            },
            {
                "condition": {
                    "op": "and",
                    "inner": [
                        {
                            "op": "eq",
                            "name": "event.contexts.trace.op",
                            "value": "default"
                        },
                        {
                            "op": "eq",
                            "name": "event.platform",
                            "value": "javascript"
                        },
                        {
                            "op": "gte",
                            "name": "event.duration",
                            "value": 3216.0
                        }
                    ]
                },
                "targetMetrics": [
                    "d:transactions/duration@millisecond"
                ],
                "targetTag": "histogram_outlier",
                "tagValue": "outlier"
            },
            {
                "condition": {
                    "op": "and",
                    "inner": [
                        {
                            "op": "eq",
                            "name": "event.contexts.trace.op",
                            "value": "function.aws"
                        },
                        {
                            "op": "eq",
                            "name": "event.platform",
                            "value": "python"
                        },
                        {
                            "op": "gte",
                            "name": "event.duration",
                            "value": 1464.0
                        }
                    ]
                },
                "targetMetrics": [
                    "d:transactions/duration@millisecond"
                ],
                "targetTag": "histogram_outlier",
                "tagValue": "outlier"
            },
            {
                "condition": {
                    "op": "and",
                    "inner": [
                        {
                            "op": "eq",
                            "name": "event.contexts.trace.op",
                            "value": "active_job"
                        },
                        {
                            "op": "eq",
                            "name": "event.platform",
                            "value": "ruby"
                        },
                        {
                            "op": "gte",
                            "name": "event.duration",
                            "value": 1059.0
                        }
                    ]
                },
                "targetMetrics": [
                    "d:transactions/duration@millisecond"
                ],
                "targetTag": "histogram_outlier",
                "tagValue": "outlier"
            },
            {
                "condition": {
                    "op": "and",
                    "inner": [
                        {
                            "op": "eq",
                            "name": "event.contexts.trace.op",
                            "value": "navigation"
                        },
                        {
                            "op": "eq",
                            "name": "event.platform",
                            "value": "other"
                        },
                        {
                            "op": "gte",
                            "name": "event.duration",
                            "value": 8706.0
                        }
                    ]
                },
                "targetMetrics": [
                    "d:transactions/duration@millisecond"
                ],
                "targetTag": "histogram_outlier",
                "tagValue": "outlier"
            },
            {
                "condition": {
                    "op": "and",
                    "inner": [
                        {
                            "op": "eq",
                            "name": "event.contexts.trace.op",
                            "value": "queue.active_job"
                        },
                        {
                            "op": "eq",
                            "name": "event.platform",
                            "value": "ruby"
                        },
                        {
                            "op": "gte",
                            "name": "event.duration",
                            "value": 4789.0
                        }
                    ]
                },
                "targetMetrics": [
                    "d:transactions/duration@millisecond"
                ],
                "targetTag": "histogram_outlier",
                "tagValue": "outlier"
            },
            {
                "condition": {
                    "op": "and",
                    "inner": [
                        {
                            "op": "eq",
                            "name": "event.contexts.trace.op",
                            "value": "sidekiq"
                        },
                        {
                            "op": "eq",
                            "name": "event.platform",
                            "value": "ruby"
                        },
                        {
                            "op": "gte",
                            "name": "event.duration",
                            "value": 942.0
                        }
                    ]
                },
                "targetMetrics": [
                    "d:transactions/duration@millisecond"
                ],
                "targetTag": "histogram_outlier",
                "tagValue": "outlier"
            },
            {
                "condition": {
                    "op": "and",
                    "inner": [
                        {
                            "op": "eq",
                            "name": "event.contexts.trace.op",
                            "value": "pageload"
                        },
                        {
                            "op": "eq",
                            "name": "event.platform",
                            "value": "other"
                        },
                        {
                            "op": "gte",
                            "name": "event.duration",
                            "value": 3000.0
                        }
                    ]
                },
                "targetMetrics": [
                    "d:transactions/duration@millisecond"
                ],
                "targetTag": "histogram_outlier",
                "tagValue": "outlier"
            },
            {
                "condition": {
                    "op": "and",
                    "inner": [
                        {
                            "op": "eq",
                            "name": "event.contexts.trace.op",
                            "value": "pageload"
                        },
                        {
                            "op": "eq",
                            "name": "event.platform",
                            "value": "other"
                        },
                        {
                            "op": "gte",
                            "name": "event.duration",
                            "value": 4589.822045672948
                        }
                    ]
                },
                "targetMetrics": [
                    "d:transactions/measurements.lcp@millisecond"
                ],
                "targetTag": "histogram_outlier",
                "tagValue": "outlier"
            },
            {
                "condition": {
                    "op": "and",
                    "inner": [
                        {
                            "op": "eq",
                            "name": "event.contexts.trace.op",
                            "value": "pageload"
                        },
                        {
                            "op": "eq",
                            "name": "event.platform",
                            "value": "other"
                        },
                        {
                            "op": "gte",
                            "name": "event.duration",
                            "value": 3384.3555060724457
                        }
                    ]
                },
                "targetMetrics": [
                    "d:transactions/measurements.fcp@millisecond"
                ],
                "targetTag": "histogram_outlier",
                "tagValue": "outlier"
            },
            {
                "condition": {
                    "op": "and",
                    "inner": [
                        {
                            "op": "eq",
                            "name": "event.contexts.trace.op",
                            "value": "console.command"
                        },
                        {
                            "op": "eq",
                            "name": "event.platform",
                            "value": "php"
                        },
                        {
                            "op": "gte",
                            "name": "event.duration",
                            "value": 1485.0
                        }
                    ]
                },
                "targetMetrics": [
                    "d:transactions/duration@millisecond"
                ],
                "targetTag": "histogram_outlier",
                "tagValue": "outlier"
            },
            {
                "condition": {
                    "op": "and",
                    "inner": [
                        {
                            "op": "eq",
                            "name": "event.contexts.trace.op",
                            "value": "queue.sidekiq"
                        },
                        {
                            "op": "eq",
                            "name": "event.platform",
                            "value": "ruby"
                        },
                        {
                            "op": "gte",
                            "name": "event.duration",
                            "value": 2262.0
                        }
                    ]
                },
                "targetMetrics": [
                    "d:transactions/duration@millisecond"
                ],
                "targetTag": "histogram_outlier",
                "tagValue": "outlier"
            },
            {
                "condition": {
                    "op": "and",
                    "inner": [
                        {
                            "op": "eq",
                            "name": "event.contexts.trace.op",
                            "value": "transaction"
                        },
                        {
                            "op": "eq",
                            "name": "event.platform",
                            "value": "node"
                        },
                        {
                            "op": "gte",
                            "name": "event.duration",
                            "value": 333.0
                        }
                    ]
                },
                "targetMetrics": [
                    "d:transactions/duration@millisecond"
                ],
                "targetTag": "histogram_outlier",
                "tagValue": "outlier"
            },
            {
                "condition": {
                    "op": "and",
                    "inner": [
                        {
                            "op": "eq",
                            "name": "event.contexts.trace.op",
                            "value": "ui.action"
                        },
                        {
                            "op": "eq",
                            "name": "event.platform",
                            "value": "cocoa"
                        },
                        {
                            "op": "gte",
                            "name": "event.duration",
                            "value": 10400.0
                        }
                    ]
                },
                "targetMetrics": [
                    "d:transactions/duration@millisecond"
                ],
                "targetTag": "histogram_outlier",
                "tagValue": "outlier"
            },
            {
                "condition": {
                    "op": "and",
                    "inner": [
                        {
                            "op": "eq",
                            "name": "event.contexts.trace.op",
                            "value": "default"
                        },
                        {
                            "op": "eq",
                            "name": "event.platform",
                            "value": "node"
                        },
                        {
                            "op": "gte",
                            "name": "event.duration",
                            "value": 1686.0
                        }
                    ]
                },
                "targetMetrics": [
                    "d:transactions/duration@millisecond"
                ],
                "targetTag": "histogram_outlier",
                "tagValue": "outlier"
            },
            {
                "condition": {
                    "op": "and",
                    "inner": [
                        {
                            "op": "eq",
                            "name": "event.contexts.trace.op",
                            "value": "ui.action.click"
                        },
                        {
                            "op": "eq",
                            "name": "event.platform",
                            "value": "cocoa"
                        },
                        {
                            "op": "gte",
                            "name": "event.duration",
                            "value": 14519.0
                        }
                    ]
                },
                "targetMetrics": [
                    "d:transactions/duration@millisecond"
                ],
                "targetTag": "histogram_outlier",
                "tagValue": "outlier"
            },
            {
                "condition": {
                    "op": "and",
                    "inner": [
                        {
                            "op": "eq",
                            "name": "event.contexts.trace.op",
                            "value": "asgi.server"
                        },
                        {
                            "op": "eq",
                            "name": "event.platform",
                            "value": "python"
                        },
                        {
                            "op": "gte",
                            "name": "event.duration",
                            "value": 4690.0
                        }
                    ]
                },
                "targetMetrics": [
                    "d:transactions/duration@millisecond"
                ],
                "targetTag": "histogram_outlier",
                "tagValue": "outlier"
            },
            {
                "condition": {
                    "op": "and",
                    "inner": [
                        {
                            "op": "eq",
                            "name": "event.contexts.trace.op",
                            "value": "http.server"
                        },
                        {
                            "op": "eq",
                            "name": "event.platform",
                            "value": "go"
                        },
                        {
                            "op": "gte",
                            "name": "event.duration",
                            "value": 16.0
                        }
                    ]
                },
                "targetMetrics": [
                    "d:transactions/duration@millisecond"
                ],
                "targetTag": "histogram_outlier",
                "tagValue": "outlier"
            },
            {
                "condition": {
                    "op": "and",
                    "inner": [
                        {
                            "op": "eq",
                            "name": "event.contexts.trace.op",
                            "value": "sentry.test"
                        },
                        {
                            "op": "eq",
                            "name": "event.platform",
                            "value": "php"
                        },
                        {
                            "op": "gte",
                            "name": "event.duration",
                            "value": 4.0
                        }
                    ]
                },
                "targetMetrics": [
                    "d:transactions/duration@millisecond"
                ],
                "targetTag": "histogram_outlier",
                "tagValue": "outlier"
            },
            {
                "condition": {
                    "op": "and",
                    "inner": [
                        {
                            "op": "eq",
                            "name": "event.contexts.trace.op",
                            "value": "websocket.server"
                        },
                        {
                            "op": "eq",
                            "name": "event.platform",
                            "value": "ruby"
                        },
                        {
                            "op": "gte",
                            "name": "event.duration",
                            "value": 16.0
                        }
                    ]
                },
                "targetMetrics": [
                    "d:transactions/duration@millisecond"
                ],
                "targetTag": "histogram_outlier",
                "tagValue": "outlier"
            },
            {
                "condition": {
                    "op": "and",
                    "inner": [
                        {
                            "op": "eq",
                            "name": "event.contexts.trace.op",
                            "value": "ui.action.click"
                        },
                        {
                            "op": "eq",
                            "name": "event.platform",
                            "value": "java"
                        },
                        {
                            "op": "gte",
                            "name": "event.duration",
                            "value": 13211.0
                        }
                    ]
                },
                "targetMetrics": [
                    "d:transactions/duration@millisecond"
                ],
                "targetTag": "histogram_outlier",
                "tagValue": "outlier"
            },
            {
                "condition": {
                    "op": "and",
                    "inner": [
                        {
                            "op": "eq",
                            "name": "event.contexts.trace.op",
                            "value": "http.server"
                        },
                        {
                            "op": "eq",
                            "name": "event.platform",
                            "value": "other"
                        },
                        {
                            "op": "gte",
                            "name": "event.duration",
                            "value": 228.0
                        }
                    ]
                },
                "targetMetrics": [
                    "d:transactions/duration@millisecond"
                ],
                "targetTag": "histogram_outlier",
                "tagValue": "outlier"
            },
            {
                "condition": {
                    "op": "and",
                    "inner": [
                        {
                            "op": "eq",
                            "name": "event.contexts.trace.op",
                            "value": "test"
                        },
                        {
                            "op": "eq",
                            "name": "event.platform",
                            "value": "node"
                        },
                        {
                            "op": "gte",
                            "name": "event.duration",
                            "value": 4284.0
                        }
                    ]
                },
                "targetMetrics": [
                    "d:transactions/duration@millisecond"
                ],
                "targetTag": "histogram_outlier",
                "tagValue": "outlier"
            },
            {
                "condition": {
                    "op": "and",
                    "inner": [
                        {
                            "op": "eq",
                            "name": "event.contexts.trace.op",
                            "value": "gql"
                        },
                        {
                            "op": "eq",
                            "name": "event.platform",
                            "value": "node"
                        },
                        {
                            "op": "gte",
                            "name": "event.duration",
                            "value": 492.0
                        }
                    ]
                },
                "targetMetrics": [
                    "d:transactions/duration@millisecond"
                ],
                "targetTag": "histogram_outlier",
                "tagValue": "outlier"
            },
            {
                "condition": {
                    "op": "and",
                    "inner": [
                        {
                            "op": "eq",
                            "name": "event.contexts.trace.op",
                            "value": "default"
                        },
                        {
                            "op": "eq",
                            "name": "event.platform",
                            "value": "python"
                        },
                        {
                            "op": "gte",
                            "name": "event.duration",
                            "value": 253.0
                        }
                    ]
                },
                "targetMetrics": [
                    "d:transactions/duration@millisecond"
                ],
                "targetTag": "histogram_outlier",
                "tagValue": "outlier"
            },
            {
                "condition": {
                    "op": "and",
                    "inner": [
                        {
                            "op": "eq",
                            "name": "event.contexts.trace.op",
                            "value": "rails.action_cable"
                        },
                        {
                            "op": "eq",
                            "name": "event.platform",
                            "value": "ruby"
                        },
                        {
                            "op": "gte",
                            "name": "event.duration",
                            "value": 20.0
                        }
                    ]
                },
                "targetMetrics": [
                    "d:transactions/duration@millisecond"
                ],
                "targetTag": "histogram_outlier",
                "tagValue": "outlier"
            },
            {
                "condition": {
                    "op": "and",
                    "inner": [
                        {
                            "op": "eq",
                            "name": "event.contexts.trace.op",
                            "value": "queue.process"
                        },
                        {
                            "op": "eq",
                            "name": "event.platform",
                            "value": "php"
                        },
                        {
                            "op": "gte",
                            "name": "event.duration",
                            "value": 850.0
                        }
                    ]
                },
                "targetMetrics": [
                    "d:transactions/duration@millisecond"
                ],
                "targetTag": "histogram_outlier",
                "tagValue": "outlier"
            },
            {
                "condition": {
                    "op": "and",
                    "inner": [
                        {
                            "op": "eq",
                            "name": "event.contexts.trace.op",
                            "value": "websocket.server"
                        },
                        {
                            "op": "eq",
                            "name": "event.platform",
                            "value": "python"
                        },
                        {
                            "op": "gte",
                            "name": "event.duration",
                            "value": 24901.0
                        }
                    ]
                },
                "targetMetrics": [
                    "d:transactions/duration@millisecond"
                ],
                "targetTag": "histogram_outlier",
                "tagValue": "outlier"
            },
            {
                "condition": {
                    "op": "and",
                    "inner": [
                        {
                            "op": "eq",
                            "name": "event.contexts.trace.op",
                            "value": "rq.task"
                        },
                        {
                            "op": "eq",
                            "name": "event.platform",
                            "value": "python"
                        },
                        {
                            "op": "gte",
                            "name": "event.duration",
                            "value": 1435.0
                        }
                    ]
                },
                "targetMetrics": [
                    "d:transactions/duration@millisecond"
                ],
                "targetTag": "histogram_outlier",
                "tagValue": "outlier"
            },
            {
                "condition": {
                    "op": "and",
                    "inner": [
                        {
                            "op": "eq",
                            "name": "event.contexts.trace.op",
                            "value": "task"
                        },
                        {
                            "op": "eq",
                            "name": "event.platform",
                            "value": "python"
                        },
                        {
                            "op": "gte",
                            "name": "event.duration",
                            "value": 1317.0
                        }
                    ]
                },
                "targetMetrics": [
                    "d:transactions/duration@millisecond"
                ],
                "targetTag": "histogram_outlier",
                "tagValue": "outlier"
            },
            {
                "condition": {
                    "op": "and",
                    "inner": [
                        {
                            "op": "eq",
                            "name": "event.contexts.trace.op",
                            "value": "ui.action.swipe"
                        },
                        {
                            "op": "eq",
                            "name": "event.platform",
                            "value": "java"
                        },
                        {
                            "op": "gte",
                            "name": "event.duration",
                            "value": 18818.0
                        }
                    ]
                },
                "targetMetrics": [
                    "d:transactions/duration@millisecond"
                ],
                "targetTag": "histogram_outlier",
                "tagValue": "outlier"
            },
            {
                "condition": {
                    "op": "and",
                    "inner": [
                        {
                            "op": "eq",
                            "name": "event.contexts.trace.op",
                            "value": "queue.task.rq"
                        },
                        {
                            "op": "eq",
                            "name": "event.platform",
                            "value": "python"
                        },
                        {
                            "op": "gte",
                            "name": "event.duration",
                            "value": 3313.0
                        }
                    ]
                },
                "targetMetrics": [
                    "d:transactions/duration@millisecond"
                ],
                "targetTag": "histogram_outlier",
                "tagValue": "outlier"
            },
            {
                "condition": {
                    "op": "and",
                    "inner": [
                        {
                            "op": "eq",
                            "name": "event.contexts.trace.op",
                            "value": "navigation"
                        },
                        {
                            "op": "eq",
                            "name": "event.platform",
                            "value": "java"
                        },
                        {
                            "op": "gte",
                            "name": "event.duration",
                            "value": 9647.0
                        }
                    ]
                },
                "targetMetrics": [
                    "d:transactions/duration@millisecond"
                ],
                "targetTag": "histogram_outlier",
                "tagValue": "outlier"
            },
            {
                "condition": {
                    "op": "and",
                    "inner": [
                        {
                            "op": "eq",
                            "name": "event.contexts.trace.op",
                            "value": "ui.action.scroll"
                        },
                        {
                            "op": "eq",
                            "name": "event.platform",
                            "value": "java"
                        },
                        {
                            "op": "gte",
                            "name": "event.duration",
                            "value": 7432.0
                        }
                    ]
                },
                "targetMetrics": [
                    "d:transactions/duration@millisecond"
                ],
                "targetTag": "histogram_outlier",
                "tagValue": "outlier"
            },
            {
                "condition": {
                    "op": "and",
                    "inner": [
                        {
                            "op": "gte",
                            "name": "event.duration",
                            "value": 0
                        }
                    ]
                },
                "targetMetrics": [
                    "d:transactions/duration@millisecond",
                    "d:transactions/measurements.lcp@millisecond",
                    "d:transactions/measurements.fcp@millisecond"
                ],
                "targetTag": "histogram_outlier",
                "tagValue": "inlier"
            },
            {
                "condition": {
                    "op": "and",
                    "inner": []
                },
                "targetMetrics": [
                    "d:transactions/duration@millisecond",
                    "d:transactions/measurements.lcp@millisecond",
                    "d:transactions/measurements.fcp@millisecond"
                ],
                "targetTag": "histogram_outlier",
                "tagValue": "outlier"
            }
        ]"#;
        let rules: Vec<TaggingRule> = serde_json::from_str(rules).unwrap();
        dbg!(rules.into_iter().fold(0, |x, a| a.deep_size_of() + x));
    }
}

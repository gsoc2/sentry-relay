local gocdtasks = import 'github.com/getsentry/gocd-jsonnet/v1.0.0/gocdtasks.libsonnet';

{
  Pipeline(region, config):: {
    environment_variables: {
      GCP_PROJECT: config.gcp_project,
      GKE_CLUSTER: config.gke_cluster,
      GKE_REGION: config.gke_region,
      GKE_CLUSTER_ZONE: config.gke_cluster_zone,
      GKE_BASTION_ZONE: config.gke_bastion_zone,
    },
    group: 'relay',
    lock_behavior: 'unlockWhenFinished',
    materials: {
      relay_repo: {
        git: 'git@github.com:getsentry/relay.git',
        shallow_clone: true,
        branch: 'master',
        destination: 'relay',
      },
    },
    stages: [

      {
        checks: {
          approval: {
            type: 'manual',
          },
          fetch_materials: true,
          jobs: {
            checks: {
              environment_variables: {
                GITHUB_TOKEN: '{{SECRET:[devinfra-github][token]}}',
              },
              timeout: 1800,
              elastic_profile_id: 'relay',
              tasks: [
                gocdtasks.script(importstr '../bash/github-check-runs.sh'),
              ],
            },
          },
        },
      },

      {
        'deploy-production': {
          approval: {
            type: 'success',
            allow_only_on_success: true,
          },
          fetch_materials: true,
          jobs: {
            create_sentry_release: {
              environment_variables: {
                SENTRY_ORG: 'sentry',
                SENTRY_PROJECT: 'relay',
                SENTRY_URL: 'https://sentry.my.sentry.io/',
                // Temporary; self-service encrypted secrets aren't implemented yet.
                // This should really be rotated to an internal integration token.
                SENTRY_AUTH_TOKEN: '{{SECRET:[devinfra-temp][relay_sentry_auth_token]}}',
              },
              timeout: 1200,
              elastic_profile_id: 'relay',
              tasks: [
                gocdtasks.script(importstr '../bash/create-sentry-release.sh'),
              ],
            },
            deploy: {
              timeout: 1200,
              elastic_profile_id: 'relay',
              tasks: [
                gocdtasks.script(importstr '../bash/deploy-relay.sh'),
              ],
            },
          },
        },
      },
    ],
  },
}

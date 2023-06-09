local pipedream = import './libs/pipedream.libsonnet';
local relay = import './libs/relay-pipeline.libsonnet';

local pipedream_config = {
    name: 'relay',
    auto_deploy: false,
    materials: {
        relay_repo: {
            git: 'git@github.com:getsentry/relay.git',
            shallow_clone: true,
            branch: 'master',
            destination: 'relay',
        },
    },
};

local region_configs = {
    s4s: {
        gcp_project: 's4s-gcp',
        gke_cluster: '',
        gke_region: '',
        gke_cluster_zone: '',
        gke_bastion_zone: '',

    },
    nam: {
        gcp_project: 's4s-nam',
        gke_cluster: '',
        gke_region: '',
        gke_cluster_zone: '',
        gke_bastion_zone: '',
    },
    eu: {
        gcp_project: 's4s-eu',
        gke_cluster: '',
        gke_region: '',
        gke_cluster_zone: '',
        gke_bastion_zone: '',
    },
};

pipedream.Render(pipedream_config, region_configs, relay.Pipeline)

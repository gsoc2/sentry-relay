local pipedream = import './libs/pipedream.libsonnet';
local relay = import './libs/relay-pipeline.libsonnet';

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

pipedream.Render("relay", region_configs, relay.Pipeline)

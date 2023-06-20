local pipedream = import 'github.com/getsentry/gocd-jsonnet/v1.0.0/pipedream.libsonnet';
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

pipedream.render(pipedream_config, relay.Pipeline)

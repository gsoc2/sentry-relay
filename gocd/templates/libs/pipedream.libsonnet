local DEPLOYMENT_ORDER = ['s4s', 'nam', 'eu'];
local CONTINUE_STAGE_NAME = 'continue-pipedream';

{
    Render(pipedream_config, region_configs, pipeline_fn)::
    local name = pipedream_config.name;
    local init_pipeline_name = 'deploy-' + name;
    local approval = if !pipedream_config.auto_deploy then
            {
                type: 'manual',
            }
        else
            {};
    local trigger_pipeline = {
        [name+'.yaml']: {
            format_version: 10,
            pipelines: {
                [init_pipeline_name]: {
                    group: name,
                    lock_behavior: "unlockWhenFinished",
                    materials: pipedream_config.materials,
                    stages: [
                        {
                            [CONTINUE_STAGE_NAME]: {
                                approval: approval,
                                jobs: {
                                    start: {
                                        tasks: [
                                            {
                                                exec: {
                                                    command: true,
                                                },
                                            },
                                        ],
                                    },
                                },
                            },
                        },
                    ],
                },
            },
        },
    };
    local supported_regions = [x for x in DEPLOYMENT_ORDER if std.get(region_configs, x) != null];

    local generate_pipeline(name, region, config, pipeline_fn) =
        local pipeline_name = init_pipeline_name+'-'+region;
        local complete_pipeline = {
            format_version: 10,
            pipelines: {
                [pipeline_name]: pipeline_fn(
                    region,
                    config,
                ),
            },
        };
        local index = std.find(region, supported_regions)[0];
        local upstream_pipeline = if index == 0 then
            init_pipeline_name
        else
            init_pipeline_name+'-'+supported_regions[index-1];
        local extra_stage = if index < std.length(supported_regions) - 1 then
            [
                {
                    [CONTINUE_STAGE_NAME]: {
                        approval: {
                            type: 'success',
                            allow_only_on_success: true,
                        },
                        jobs: {
                            continue: {
                                tasks: [
                                    {
                                        exec: {
                                            command: true,
                                        },
                                    },
                                ],
                            },
                        },
                    },
                },
            ]
            else
                [];

        complete_pipeline + {
            pipelines+: {
                [pipeline_name]+: {
                    materials+: {
                        upstream_pipeline: {
                            pipeline: upstream_pipeline,
                            stage: CONTINUE_STAGE_NAME,
                        },
                    },
                    stages+: extra_stage,
                },
            },
        };

    local service_pipelines = {
        [name+'-'+region+'.yaml']: generate_pipeline(name, region, region_configs[region], pipeline_fn)
        for region in supported_regions
    };
    trigger_pipeline + service_pipelines,
}

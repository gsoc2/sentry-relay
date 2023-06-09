local DEPLOYMENT_ORDER = ['s4s', 'nam', 'eu'];
local INITIAL_STAGE_NAME = 'start-pipedream';
local CONTINUE_STAGE_NAME = 'continue-pipedream';

{
    Render(name, region_configs, pipeline_fn)::
    local trigger_pipeline = {[name+'.yaml']: {
        format_version: 10,
        pipelines: {
            ['deploy-' + name]: {
                group: name,
                lock_behavior: "unlockWhenFinished",
                materials: {},
                stages: [
                    {
                        [INITIAL_STAGE_NAME]: {
                            approval: {
                                type: 'manual',
                                allow_only_on_success: true,
                            },
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
    }};
    local supported_regions = [x for x in DEPLOYMENT_ORDER if std.get(region_configs, x) != null];

    local generate_pipeline(name, region, config, pipeline_fn) =
        local pipeline_name = 'deploy-'+name+'-'+region;
        local service_pipeline = pipeline_fn(
            pipeline_name,
            region,
            config,
        );
        local index = std.find(region, supported_regions)[0];
        local upstream_pipeline = if index == 0 then
            'deploy-'+name
        else
            'deploy-'+name+'-'+supported_regions[index-1];
        local upstream_stage = if index == 0 then
            INITIAL_STAGE_NAME
        else
            CONTINUE_STAGE_NAME;
        local extra_stages = if index < std.length(supported_regions) - 1 then
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
        service_pipeline + {
            pipelines+: {
                [pipeline_name]+: {
                    materials+: {
                        upstream_pipeline: {
                            pipeline: upstream_pipeline,
                            stage: upstream_stage,
                        },
                    },
                    stages+: extra_stages,
                },
            },
        };

    local service_pipelines = {
        [name+'-'+region+'.yaml']: generate_pipeline(name, region, region_configs[region], pipeline_fn)
        for region in std.objectFields(region_configs)
    };
    trigger_pipeline + service_pipelines,
}

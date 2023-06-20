#!/bin/bash

/devinfra/scripts/k8s/k8stunnel && \
/devinfra/scripts/k8s/k8s-deploy.py \
    --label-selector="service=relay,deploy_if_production=true" \
    --image="us.gcr.io/sentryio/relay:${GO_REVISION_RELAY_REPO}" \
    --container-name="relay"

#!/usr/bin/env bash

set -euo pipefail

CONTAINER_NAME=zkTEE
DOCKER_IMAGE=bl4ck5un/rust-sgx-sdk:v2.0.0-preview

if docker container inspect $CONTAINER_NAME > /dev/null 2>&1; then
  docker start -ai $CONTAINER_NAME
else
  docker run \
    --platform linux/amd64 \
    -v $PWD:/root/sgx \
    -ti \
    --hostname $CONTAINER_NAME \
    --name $CONTAINER_NAME \
    -e SGX_MODE=SW \
    $DOCKER_IMAGE
fi

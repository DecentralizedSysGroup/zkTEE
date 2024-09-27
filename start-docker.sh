#!/usr/bin/env bash

set -euo pipefail

CONTAINER_NAME=sgx-starter
DOCKER_IMAGE=sgx-sdk-20.04

if docker container inspect $CONTAINER_NAME > /dev/null 2>&1; then
  docker start -ai $CONTAINER_NAME
else
  docker run \
    -v $PWD:/root/sgx \
    -ti \
    --hostname $CONTAINER_NAME \
    --name $CONTAINER_NAME \
    -e SGX_MODE=SW \
    $DOCKER_IMAGE
fi

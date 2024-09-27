#!/usr/bin/env bash

set -euo pipefail

CONTAINER_NAME=sgx-starter-hw
DOCKER_IMAGE=bl4ck5un/rust-sgx-sdk:v2.0.0-preview

if docker container inspect $CONTAINER_NAME > /dev/null 2>&1; then
  docker start -ai $CONTAINER_NAME
else
  docker run \
    -v $PWD:/root/sgx \
    -ti \
    --hostname $CONTAINER_NAME \
    --name $CONTAINER_NAME \
    -e SGX_MODE=HW \
    --device /dev/sgx/enclave --device /dev/sgx/provision \
    $DOCKER_IMAGE
fi

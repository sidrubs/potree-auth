#!/usr/bin/env bash

# Builds the current codebase into a docker image `potree-auth:latest`

set -x
set -eo pipefail

if ! [ -x "$(command -v docker)" ]; then
  echo >&2 "Error: docker is not installed."
  exit 1
fi

SCRIPT_DIR=$(dirname "$0")
DOCKERFILE_DIR=${SCRIPT_DIR}/..

docker build -t potree-auth:latest ${DOCKERFILE_DIR}
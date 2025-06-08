#!/usr/bin/env bash

# Runs the `potree-auth:latest` image using docker compose.

set -x
set -eo pipefail

if ! [ -x "$(command -v docker)" ]; then
  echo >&2 "Error: docker is not installed."
  exit 1
fi

SCRIPT_DIR=$(dirname "$0")
DOCKER_COMPOSE_DIR=${SCRIPT_DIR}/../docker-compose.yml

docker compose -f ${DOCKER_COMPOSE_DIR} up
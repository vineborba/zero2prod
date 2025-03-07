#!/usr/bin/env bash
set -x
set -eo pipefail

# if a valkey container is running, print instructions to kill it and exit
RUNNING_CONTAINER=$(docker ps --filter 'name=valkey' --format '{{.ID}}')
if [[ -n $RUNNING_CONTAINER ]]; then
  echo >&2 "there is a valkey container already running, kill it with"
  echo >&2 "    docker kill ${RUNNING_CONTAINER}"
  exit 1
fi

# Launch Valkey using Docker
docker run \
  -p 6379:6379 \
  -d \
  --name "valkey_$(date '+%s')" \
  -e ALLOW_EMPTY_PASSWORD=yes \
  bitnami/valkey:8.0

>&2 echo "Valkey is ready to go!"
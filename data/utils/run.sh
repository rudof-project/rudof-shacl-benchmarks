#!/usr/bin/env bash

set -euo pipefail

printf "[+] Running rudof/$1:latest with the conformant data file\n\n"

docker run \
  --rm \
  -v ./test-core-shapes.ttl:/shapes.ttl:ro \
  -v ./data-conforms.ttl:/data.ttl:ro \
  rudof/$1:latest \
  /data.ttl \
  turtle \
  /shapes.ttl \turtle \
  /test.csv \
  /dev/stdout \
  1 \
  0

printf "\n\n\n[+] Running rudof/$1:latest with the non-conformant data file\n\n"

docker run \
  --rm \
  -v ./test-core-shapes.ttl:/shapes.ttl:ro \
  -v ./data-no-conforms.ttl:/data.ttl:ro \
  rudof/$1:latest \
  /data.ttl \
  turtle \
  /shapes.ttl \turtle \
  /test.csv \
  /dev/stdout \
  1 \
  0

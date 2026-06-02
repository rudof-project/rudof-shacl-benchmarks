#!/usr/bin/env bash

set -e

mkdir -p /app/dist/icdd

IDX=(1 2 3 4)
TYPES=("binary" "directed1ton" "directedbinary")

for t in "${TYPES[@]}"; do
  cp "/app/icdd/sourcecode/src/main/resources/benchmark/rulesets/predicates_$t.ttl" "/app/dist/icdd/shapes-$t.ttl"
  for i in "${IDX[@]}"; do
    cp "/app/icdd/sourcecode/src/main/resources/benchmark/datasets/$i/dataset-$t.ttl" "/app/dist/icdd/data-$t-$i.ttl"
  done
done

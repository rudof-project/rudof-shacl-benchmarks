#!/usr/bin/env bash

set -e

cd /app/era && \
  bash /app/era/get_data.sh
cd /app

mkdir -p /app/dist/era

mv /app/era/data/ERA.ttl /app/dist/era/era-data.ttl
mv /app/era/data/ES.ttl  /app/dist/era/es-data.ttl
mv /app/era/data/FR.ttl  /app/dist/era/fr-data.ttl

cp /app/era/shapes/core_shapes.ttl /app/dist/era/core-shapes.ttl
cp /app/era/shapes/era_shapes.ttl  /app/dist/era/era-shapes.ttl
cp /app/era/shapes/tds_shapes.ttl  /app/dist/era/tds-shapes.ttl

trap 'code=$?; rm -f /app/2025-01-05-rinf-xml-combined.nt; exit $code' EXIT

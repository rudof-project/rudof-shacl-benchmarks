#!/usr/bin/env bash

set -e

mkdir -p /app/dist/lubm

SIZES=(5 10 50 100 500)

cd /app/lubm && \
  ant -f build.xml
cd /app

for s in ${SIZES[@]}; do
  cd /app/lubm && \
    bash run.sh "$s" "data-$s.nt.gz" && \
    gzip -d data-$s.nt.gz && \
    perl -pi -e 's/<(?!(file|http):\/\/)([^>]+)>/<file:\/\/\/\2>/g' data-*.nt && \
    perl -pi -e "s|file://$PWD|file://|g" data-*.nt
done
cd /app

mv /app/lubm/data-*.nt /app/dist/lubm/
# TODO - Auto generate with shexer if https://github.com/weso/shexer/issues/209 is solved
cp /app/utils/lubm-shapes.ttl /app/dist/lubm/shapes.ttl

trap 'code=$?; cd /app/lubm && rm -f data-*.nt && rm -f data-*.nt.gz && rm -f University*.owl; exit $code' EXIT

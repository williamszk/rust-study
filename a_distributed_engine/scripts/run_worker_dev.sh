#! /bin/bash

docker build -t dustr-worker-dev -f scripts/Dockerfile.worker.dev .

cp ./dustr/target/debug/worker ./worker

docker run \
    -it \
    -d \
    --rm \
    dustr-worker-dev

rm worker
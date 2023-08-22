#! /bin/bash

cp ./dustr/target/debug/worker ./worker

docker build -t dustr-worker-dev -f scripts/Dockerfile.worker.dev .

rm worker

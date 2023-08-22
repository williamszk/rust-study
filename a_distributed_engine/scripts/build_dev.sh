#! /bin/bash

# compile the executables
cd dustr
cargo build
cd -

# create images
cp ./dustr/target/debug/worker ./worker
docker build -t dustr-worker-dev -f scripts/Dockerfile.worker.dev .
rm worker

cp ./dustr/target/debug/manager ./manager
docker build -t dustr-manager-dev -f scripts/Dockerfile.manager.dev .
rm manager
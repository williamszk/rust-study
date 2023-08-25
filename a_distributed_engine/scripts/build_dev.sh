#! /bin/bash

# compile the executables ------------------------------------------------------
cd dustr
cargo build
cd -

# create images ----------------------------------------------------------------
docker build -t dustr-worker-dev -f scripts/Dockerfile.worker.dev .
docker build -t dustr-manager-dev -f scripts/Dockerfile.manager.dev .
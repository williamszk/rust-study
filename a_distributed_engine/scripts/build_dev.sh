#! /bin/bash

# compile the executables ------------------------------------------------------
cargo build

# create images ----------------------------------------------------------------
docker build -t dustr-worker-dev -f scripts/Dockerfile.worker.dev .
docker build -t dustr-manager-dev -f scripts/Dockerfile.manager.dev .
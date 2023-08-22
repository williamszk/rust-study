#! /bin/bash

cp ./dustr/target/debug/manager ./manager
docker build -t dustr-manager-dev -f scripts/Dockerfile.manager.dev .
rm manager

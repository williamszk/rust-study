#! /bin/bash

docker build -t dustr-build-dev -f scripts/Dockerfile.build.dev .

docker run \
    -it \
    -d \
    --name dustr-builder \
    -v ./dustr/target:/usr/dustr/target \
    dustr-build-dev

# [ inside the container ]

docker exec -it dustr-builder cargo build 

# time cargo build

# exit

docker kill dustr-builder
docker rm dustr-builder

sudo chown -R $USER:$USER ./dustr/target

# in case we want to do a build from scratch
# sudo rm -rf ./dustr/target
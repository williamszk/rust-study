

cargo new a_distributed_engine

cargo run --bin main
cargo run --bin worker
cargo run --bin repl


docker run -it -d -p 8081:8081 --name dustr-worker williamszk/a-distributed-engine-worker 
docker run -it --name dustr-main williamszk/a-distributed-engine-main 

docker kill dustr-worker
docker rm dustr-worker

docker kill dustr-main
docker rm dustr-main

docker compose run

# Testing Workder 01 ==========================================================
docker run -it -d \
    -p 8081:8081 \
    --name dustr-worker-01 \
    williamszk/a-distributed-engine-worker \
    ./worker 8081

docker kill dustr-worker-01
docker rm dustr-worker-01


# Testing Workder 02 ==========================================================
docker run -it -d \
    -p 8082:8082 \
    --name dustr-worker-02 \
    williamszk/a-distributed-engine-worker \
    ./worker 8082

docker kill dustr-worker-02
docker rm dustr-worker-02

docker logs dustr-worker-02 

# ----------------------------------------------
docker run --rm -it williamszk/a-distributed-engine-base-image bash

# ----------------------------------------------
# Docker build dev container
docker build -t test-image -f docker/main/Dockerfile.dev .
docker run -it \
    -v ./a_distributed_engine/target:/usr/a_distributed_engine/target \
    --rm test-image

# inside the container
time cargo build 
time cargo build --release

# to delete the target dir, and remove the cache
# in the host machine
sudo rm -rf ./a_distributed_engine/target
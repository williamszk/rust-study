

cargo new dustr

cargo run --bin manager
cargo run --bin worker
cargo run --bin dustr


docker run -it -d -p 8081:8081 --name dustr-worker williamszk/a-distributed-engine-worker 
docker run -it --name dustr-manager williamszk/a-distributed-engine-manager 

docker kill dustr-worker
docker rm dustr-worker

docker kill dustr-manager
docker rm dustr-manager

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
docker build -t dustr-worker-dev -f docker/worker/Dockerfile.dev .
docker compose down 
docker compose up
docker run -it \
    -v ./dustr/target:/usr/dustr/target \
    --rm dustr-worker-dev

# inside the container
cd dustr
time cargo build 
time cargo build --release

# to delete the target dir, and remove the cache
# in the host machine
sudo rm -rf ./dustr/target


# I'm having problem trying to make things work with docker-compose
# to remove all containers
docker rm -f $(docker ps -a -q)
docker kill $(docker ps -a -q)

docker compose down 
docker compose up --build
docker compose run -it  dustr-worker bash
docker compose ls
docker compose logs a_distributed_engine-dustr-worker-1
docker inspect a_distributed_engine-dustr-worker-1

# ----------------------------------------------
chmod +x ./scripts/run_worker_dev.sh
./scripts/run_worker_dev.sh


cp ./dustr/target/debug/worker ./

sudo chown -R $USER:$USER ./dustr/target

# ----------------------------------------------
docker compose run --rm -it  dustr-manager bash

openssl s_client -connect dummyjson.com:443

apt-get update && apt-get install openssl

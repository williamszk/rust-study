
docker build \
    -f docker/main/Dockerfile \
    . \
    -t williamszk/a-distributed-engine-main


docker build \
    -f docker/worker/Dockerfile \
    . \
    -t williamszk/a-distributed-engine-worker
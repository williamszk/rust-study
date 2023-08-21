
docker build \
    -f docker/manager/Dockerfile \
    . \
    -t williamszk/a-distributed-engine-manager


docker build \
    -f docker/worker/Dockerfile \
    . \
    -t williamszk/a-distributed-engine-worker
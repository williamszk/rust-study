# https://docs.docker.com/build/building/multi-stage/#use-an-external-image-as-a-stage
FROM fedora

WORKDIR /usr/a_distributed_engine
COPY ./a_distributed_engine/target/debug/main ./main

CMD ./main banana


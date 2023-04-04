

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

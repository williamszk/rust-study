
# Objectives

First of all, this is just an experiment for me to learn stuff.
I don't know what I'll discover.

This is a place to write some code for a distributed engine.

- [x] Start by setting up a server with actix-web.
- [x] Make master request something to a worker node.
- [x] Build two containers running inside a same docker-compose file.
- [ ] Build two worker nodes and make the main node send requests to both of them.
- .
- .
- .
- [ ] Build a vector and sum a value in all elements of the vector and send the computation to two worker nodes.
- .
- .
- .
- [ ] Use grpc for communicating between nodes not http.
- [ ] Spin up ec2 instances with as the nodes.
- [ ] Use Terraform to manage the cluster.
- [ ] Use Kubernetes to deploy the distributed engine cluster.

We'll build a cluster of containers.
There is the main node and the worker nodes.
In the main node we should be able to send commands.
The main node will not execute those commands, it will pass them to the worker nodes.

I have two alternatives when creating a dustr cluster.

1) Create a standard cluster. We will directly configure the nodes on the cluster.
2) Use on top of Kubernetes, or any other container orchestrator. It could be also docker swarm.

For this we'll need to make the core logic agnostic to the way in which the workload
is distributed to the nodes.
In Spark-k8s there is the "Kubernetes Scheduler Backend" which is responsible for
taking the core logic and making it run inside the k8s cluster.

## Sources of instruction:

- https://rust-cli.github.io/book/tutorial/cli-args.html
- https://www.usenix.org/legacy/event/hotcloud10/tech/full_papers/Zaharia.pdf   Spark: Cluster Computing with Working Sets
- https://www.usenix.org/system/files/conference/nsdi12/nsdi12-final138.pdf  Resilient Distributed Datasets: A Fault-Tolerant Abstraction for
In-Memory Cluster Computing



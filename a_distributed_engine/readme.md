
# Objectives

First of all, this is just an experiment for me to learn stuff.
I don't know what I'll discover.

This is a place to write some code for a distributed engine.

- [x] Start by setting up a server with actix-web.
- [x] Make master request something to a worker node.
- [x] Build two containers running inside a same docker-compose file.
- [x] Make worker executable receive command line argument for port.
- [x] Make default port be 8081 for the worker and 8080 for the master node.
- [ ] Inside of the worker node, there is no problem to use por 8080. But when exposing the port the container should know to export it to another port. Maybe we don't need to have a command line argument to set the port internally. To handle the conflicting ports we can just use the `docker run` and pass the appropriate options for port `-p`.
- [ ] Build two worker nodes and make the main node send requests to both of them.
- .
- [ ] Build a CLI tool to communicate to the main node. It is like kubectl, so that we can send commands to the main node.
- .
- [ ] There should be a way to make the main node know the existence of the worker
nodes. One possibility is to make the worker node know the address of the main node
and then it can make a request to the main node so that they are listed as available for work.
- .
- [ ] The main node should have a way to do health checks on the worker nodes, before trying to send requests.
- .
- [ ] Build a vector and sum a value in all elements of the vector and send the computation to two worker nodes.
- .
- [ ] Use grpc for communicating between nodes not http.
- [ ] Spin up ec2 instances with as the nodes.
- [ ] Use Terraform to manage the cluster.
- [ ] Use Kubernetes to deploy the distributed engine cluster.
- .
- [ ] We'll build a cluster of containers. There is the main node and the worker nodes. In the main node we should be able to send commands. The main node will not execute those commands, it will pass them to the worker nodes.
- [ ] I have two alternatives when creating a dustr cluster. 1) Create a standard cluster. We will directly configure the nodes on the cluster. 2) Use on top of Kubernetes, or any other container orchestrator. It could be also docker swarm.

For this we'll need to make the core logic agnostic to the way in which the workload
is distributed to the nodes.
In Spark-k8s there is the "Kubernetes Scheduler Backend" which is responsible for
taking the core logic and making it run inside the k8s cluster.

## Sources of instruction:

- https://rust-cli.github.io/book/tutorial/cli-args.html
- https://www.usenix.org/legacy/event/hotcloud10/tech/full_papers/Zaharia.pdf   Spark: Cluster Computing with Working Sets
- https://www.usenix.org/system/files/conference/nsdi12/nsdi12-final138.pdf  Resilient Distributed Datasets: A Fault-Tolerant Abstraction for
In-Memory Cluster Computing



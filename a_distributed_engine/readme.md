
# Objectives

This is a place to write some code for a distributed engine.

- [x] Start by setting up a server with actix-web.
- [x] Make master request something to a worker node.
- [ ] Build two container running inside a same docker-compose file.
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

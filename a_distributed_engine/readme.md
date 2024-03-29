# Objectives

First of all, this is just an experiment for me to learn stuff.
I don't know what I'll discover.

This is a place to write some code for a distributed engine.

## To-do:

- [ ] Try making the manager send an array to one worker asking the worker to
      double the values inside. This is one little step.
- [ ] Experiment with those notes: https://github.com/LukeMathWalker/ndarray-koans
- [ ] With more time take a look at this package: https://docs.rs/ndarray/latest/ndarray/
- [ ] Study the book and see if anything will help build the project.
- [ ] These also seems to be a good material to explore:
      https://www.lpalmieri.com/posts/2019-02-23-scientific-computing-a-rust-adventure-part-0-vectors/
- [ ] There is this package called linfa: https://github.com/rust-ml/linfa;
      This is a scikit-learn like project.
- [ ] We can try to run a simple linear model with linfa and then run it with
      scikit-learn and then compare their performances. We can try to do a a
      performance comparison like in:
      https://www.lpalmieri.com/posts/2019-12-01-taking-ml-to-production-with-rust-a-25x-speedup/
      We can try to use this package inside our project.
- [ ] Try to import a python trained model into rust and then create a rust package
      with bin crate that can use it in its functionality.
      https://docs.rs/serde-pickle/latest/serde_pickle/
      `ml_models/python`.
- [ ] It's being hard to find a way to import a pickle object with a trained model
      into Rust. But a good alternative is to train and serve the mode inside Rust
      itself. For a data scientist the only difference (check that) would be training.
      But with the training dataframe (the dataframe used for training the mode)
      we can train a model inside Rust and serve it. Also the training in Rust could
      be considerably faster. Also the serving could be much faster.
- [ ] An experiment: create a Rust-Go integration. We can call go code through Rust.
      Try using json string for passing information around.
      We could try to do the same with C and C++, use json string and parse internally.
      Instead of trying to use the native interfaces for communication and recreating
      the signature of functions all the time. This takes lots of time.
- [ ] Try to read a csv locally and do something and write it back. Try to do it
      with the manager first.
      Then we can try to make the worker do it. We use the manager to ask the
      worker to do it.
      Use one worker first. Then we can ask two workers to do it.
- [ ] Let's try to do a map operation of double in an array inside the manager.
      Try to distribute this operation between the workers.
      First, we can try to make the manager send the data from the manager to
      the workers.
      In larger data sets we could try to make the workers read directly from the source.
      This would require us to read predefined chunks of the data.
      We could try to read apache arrow files.
- [ ] Inside of the worker node, there is no problem to use por 8080. But when
      exposing the port the container should know to export it to another port.
      Maybe we don't need to have a command line argument to set the port
      internally. To handle the conflicting ports we can just use the `docker run`
      and pass the appropriate options for port `-p`.
- [ ] Polars is made with rust I can try to build something that works with Python
      like pandas and polars and make the transition to a distributed engine
      be seem less.
- [x] Build two worker nodes and make the manager node send requests to both of them.
- [ ] Build a CLI tool to communicate to the manager node. It is like kubectl, so
      that we can send commands to the manager node.
- [ ] There should be a way to make the manager node know about the existence of the worker
      nodes. One possibility is to make the worker node know the address of the manager node
      and then it can make a request to the manager node so that they are listed
      as available for work.
      Question: should the worker make a request to the master and ask to be
      part of the cluster? Or should the master
      make a request to the worker so that it participates on the cluster?
- [ ] HTTP and REST is just one way to make the communication between worker and
      manager nodes. Another way would be to do web-sockets or using gRPC.
- [ ] Use gRPC for communicating between nodes instead of HTTP.
- .
- [ ] The manager node should have a way to do health checks on the worker
      nodes, before trying to send requests.
- .
- [ ] Build a vector and sum a value in all elements of the vector and send the
      computation to two worker nodes.
- .
- [ ] Try using Terraform to manage the cluster.
- [ ] Try using Kubernetes to deploy the distributed engine cluster.
- .
- [ ] We'll build a cluster of containers. There is the manager node and the worker
      nodes. In the manager node we should be able to send commands. The manager node
      will not execute those commands, it will pass them to the worker nodes.
      One of the jobs of the manager is to plan the processing. By planing we mean
      first divide the dataset into the appropriate sizes.
- [ ] I have two alternatives when creating a dustr cluster.
      Create a standard cluster. We will directly configure the nodes on the
      cluster.
      Use on top of Kubernetes, or any other container orchestrator.
      It could be also Docker Swarm.
- [ ] I'm assuming a client-server relationship in the cluster. But we could
      explore other forms of relationships between the components of the cluster.
      For example a message-broker architecture.
      Are there any other forms of architecture that are interesting for this problem?
- [x] We can try to build and run the rust program inside a container if we
      use volumes for the `Cargo.lock` and `target` files.
      This works but it needs internet. Which is a good assumption for someone
      working on this project.
- [x] currently the build happens inside the host machine, ideally we should
      write the code in such a way that the build happens inside a container;
      we could use a base image which is responsible for building all the
      executables; and just copy the executables to the appropriate docker images;
      This is not a good idea, because it will make the build compile from scratch;
      It takes much more time, a simple test showed that with caching the
      compilation in the machine took 0.20s and from scratch
      (including download of libs) it took 103s;
- [x] Start by setting up a server with actix-web.
- [x] Make master request something to a worker node.
- [x] Build two containers running inside a same docker-compose file.
- [x] Make worker executable receive command line argument for port.
- [x] Make default port be 8081 for the worker and 8080 for the master node.
- [ ] We'll need to make the core logic agnostic to the way in which the workload
      is distributed to the nodes.
      In Spark-k8s there is the "Kubernetes Scheduler Backend" which is responsible for
      taking the core logic and making it run inside the k8s cluster.

## Sources of instruction:

- https://rust-cli.github.io/book/tutorial/cli-args.html
- https://www.usenix.org/legacy/event/hotcloud10/tech/full_papers/Zaharia.pdf Spark: Cluster Computing with Working Sets
- https://www.usenix.org/system/files/conference/nsdi12/nsdi12-final138.pdf Resilient Distributed Datasets: A Fault-Tolerant Abstraction for
  In-Memory Cluster Computing

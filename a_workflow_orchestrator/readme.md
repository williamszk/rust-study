# Objective

Just an experiment for building a workflow orchestrator using Rust.

Immediate tasks:

- [] Build a copy of cronjob

## Some considerations for later

- Should this orchestrator live inside a cluster?
- What are the types of nodes that we should have?
- It is not the orchestrator's job to process or store the data.
- This should be a general purpose orchestrator.
- It could be used for simple workload like running bash scripts.
- It should have an optional web ui for monitoring complex workflows.
- The web ui should be dark mode by default.
- It shouldn't only use DAGs. Watch this video about Prefect: https://youtu.be/XL4wgLUp-VA

# 🦀 Infe.rs

This is a WIP repo that aims to implement a rusty task queuer for machine learning model inference, running in dockerised microservices. This came out of wanting to learn about (i) more robust inference of machine learning models - particulary in Rust - and (ii) using docker for deployment of (micro)services.

This project uses three microservices. One is a [standard postgres image](https://hub.docker.com/_/postgres) from Docker. The other two I am in the process are writing, and they are a task queuer and a model inference runner.

### Task Queuer

The Task Queuer is currently in progress. It uses a postgres database as the task queue, which it communicates with using [tokio_postgres](https://crates.io/crates/tokio-postgres) and [deadpool_postgres](https://crates.io/crates/deadpool-postgres). It exposes an API using [actix](https://actix.rs/). So far it can:
- check the connection to the database,
- add jobs to the queue, and
- check the jobs on the queue.

### Model Inference Runner

To be implemented with [Candle](https://github.com/huggingface/candle)!

## Pre-requisites

- You will need to [install Rust](https://www.rust-lang.org/learn/get-started) and [Docker](https://docs.docker.com/engine/install/).
- Docker will install any other pre-requisites when spinning up the containers, but for your piece of mind you may wish to verify the build with:
```
cargo build
```

- If developing, please install pre-commit checks:
```
pre-commit install
```

## Usage

- Start the microservices by running:
```
docker compose up --build
```
- Run `curl https://localhost:8000/api/healthcheck` to verify all is well.
- Add some tasks and view them with `scripts/test_api.sh`.


## To Do

- Task queuer (add task to 'tasks' db, update with doing/done).
- Model inference.

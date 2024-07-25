# 🦀 Infe.rs

This is a WIP repo that aims to implement a rusty task queuer for machine learning model inference, running in dockerised microservices. This came out of wanting to learn more about (i) robust inference of machine learning models - particulary in Rust - and (ii) using Docker for deployment of (micro)services.

This project uses three microservices. One is a [standard postgres image](https://hub.docker.com/_/postgres) from Docker. The other two I'm in the process of writing, and will (hopefully!) be a task queuer and a model inference runner.

### 📋 Task Queuer

The Task Queuer is currently in progress. It uses a postgres database as the task queue, which it communicates with using [tokio_postgres](https://crates.io/crates/tokio-postgres) and [deadpool_postgres](https://crates.io/crates/deadpool-postgres). It exposes an API using [actix](https://actix.rs/). So far it can:
- check the connection to the database,
- add jobs to the queue, and
- check the jobs on the queue.

### ⚡️ Model Inference Runner

To be implemented with [Candle](https://github.com/huggingface/candle)! Next up:
- poll the database for the oldest inference task that hasn't been picked up.
- run inference on it!
- update the task queue with the output.

## ⚙️ Pre-requisites

- You will need to install [Rust](https://www.rust-lang.org/learn/get-started) and [Docker](https://docs.docker.com/engine/install/).
- Docker will install any other pre-requisites when spinning up the containers, but you may wish to verify it builds locally with `cargo build`.

- If developing, please install pre-commit checks:
```
pre-commit install
```

## 🧑‍💻 Usage

- Start the microservices by running:
```
docker compose up --build
```
- Run `curl http://localhost:8000/api/healthcheck` to verify all is well.
- Add some tasks and view them with `scripts/test_api.sh`.

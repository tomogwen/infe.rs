# 🔍 Infe.rs

This is a WIP repo implementing a simple model inference stack. It should provide a simple web UI to interact with an ML model. When finished, it will consist of the following microservices:
- a front-end, written in React,
- a back-end, written in Rust,
- a Postgres DB, to store previous queries, and
- a model runner, specifically vLLM.

## ⚙️ Pre-requisites

- To build and run the project you will need [Docker](https://docs.docker.com/engine/install/) and [Docker Compose](https://docs.docker.com/compose/install/).
- To develop you will also need [Rust](https://www.rust-lang.org/learn/get-started) and [React](https://react.dev/learn/installation).
- If developing, please install [pre-commit](https://pre-commit.com/#install) hooks:
```
pre-commit install
```

## 🧑‍💻 Usage

- Build and run with:
```
docker compose up --build
```
- Run `curl http://localhost:8000/api/healthcheck` to check the backend is running.
- Verify some of the backend/database functionality with `scripts/test_api.sh`.

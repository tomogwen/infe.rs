# Infe.rs

To be rusty dockerised inference of ML models.

## Installation

- `pre-commit install`

## To Do

- Task queuer (add task to 'tasks' db, update with doing/done).
- Model inference.

## Building and running your application

- Start the application by running:
`docker compose up --build`.
- Run `curl localhost:8000` to get a sample!

### Hard reset

- `docker compose down`
- `docker rm -f $(docker ps -a -q)`
- `docker volume rm $(docker volume ls -q)`

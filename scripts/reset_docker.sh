docker compose down
if [ "$(docker ps -a -q)" ]; then
    docker rm -f $(docker ps -a -q)
fi
if [ "$(docker volume ls -q)" ]; then
    docker volume rm $(docker volume ls -q)
fi
docker compose up --build

docker build -f reverse-proxy/Dockerfile -t todo-proxy .
cd server
docker-compose up -d --wait
pause
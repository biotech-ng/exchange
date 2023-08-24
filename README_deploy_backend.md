### Deploy and run backend in container

Run database:
```
cd backend/
docker compose up
```

From the root project directoty execute:
```
docker build -t backend_exchange -f backend_dockerfile .
docker run -e DATABASE_URL=postgres://postgres:postgres@postgres:5432  --network container:postgres  backend_exchange
```

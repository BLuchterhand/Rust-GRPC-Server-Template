# Restful-Backend
#### Run postgres server:
docker compose up

#### Run server:
* cargo run --bin server
* server becomes available at localhost:40130

#### Creating migration files (for use in external services, not necessary here):
* migrate create -ext sql -dir migrations <name>

#### Running migrations (not currently implemented, saving for future use)
* migrate -path migrations -database postgres://localhost:5432/postgres up

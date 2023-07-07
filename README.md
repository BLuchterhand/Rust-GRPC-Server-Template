# Restful-Backend
Rune postgres server:
docker compose up

Run server:
cargo run --bin server

Creating migration files (for use in external services, not necessary here):
migrate create -ext sql -dir migrations <name>

Running migrations (not currently implemented, saving)
migrate -path migrations -database postgres://localhost:5432/postgres up

To generate Swift pb and grpc filers:
protoc --swift_out=/Users/ben/Documents/GitHub/Restful-Frontend --swift-grpc_out=Client=true:/Users/ben/Documents/GitHub/Restful-Frontend api/account/account_service.proto

Had to manually install binary from releases, had to rename bin from:
protoc-gen-grpc-swift 
to 
protoc-gen-swift-grpc
Could not make:
https://github.com/grpc/grpc-swift/releases
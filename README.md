# SQLX Example

Small Rust example for using SQLX, Postgres and Tokio

### Start Postgres
```
docker-compose up -d
```

### Run Example
Each time you run the application it will
- Insert a new record
- Count and Print how many records there now are
- Print all of the records
```
cargo run --release
```

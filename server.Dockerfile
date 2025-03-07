FROM rust:1.84.1-bookworm AS builder
WORKDIR /app
COPY ./domain /app/domain
COPY ./infrastructure /app/infrastructure
COPY ./app_server /app/app_server
COPY ./mission_resetter /app/mission_resetter
COPY Cargo.toml /app/Cargo.toml

RUN cargo build -p app_server --release
RUN cargo install --root /app/sqlx_bin sqlx-cli

FROM debian:bookworm
RUN apt-get update && \
    apt-get install -y make libssl-dev

WORKDIR /app

COPY --from=builder /app/target/release/app_server /app/target/release/app_server 
COPY --from=builder /app/sqlx_bin/bin/sqlx /usr/local/bin/sqlx

COPY .env /app/.env
COPY exp_table.csv /app/exp_table.csv
COPY jwt_key.txt /app/jwt_key.txt
COPY ./migrations /app/migrations
COPY Makefile /app/Makefile

EXPOSE 8080

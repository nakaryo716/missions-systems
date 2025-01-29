FROM rust:1.84-bookworm

WORKDIR /app
COPY ./app_server /app/app_server
COPY ./mission_resetter  /app/mission_resetter 
COPY ./domain /app/domain
COPY ./infrastructure /app/infrastructure
COPY ./migrations /app/migrations
COPY Cargo.toml /app/Cargo.toml
COPY .env /app/.env
COPY exp_table.csv /app/exp_table.csv
COPY jwt_key.txt /app/jwt_key.txt
COPY Makefile /app/Makefile

RUN cargo install sqlx-cli
RUN cargo build --release

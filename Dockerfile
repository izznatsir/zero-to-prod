FROM rust:1.79-alpine3.20 AS base

RUN apk add clang lld

WORKDIR /app

FROM base AS chef

RUN cargo install cargo-chef --locked

FROM chef AS planner

COPY . .

RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder

COPY --from=planner /app/recipe.json recipe.json

RUN cargo chef cook --release --recipe-path recipe.json

COPY . .

ENV SQLX_OFFLINE=true

RUN cargo build --release --bin ztp

FROM base AS sqlx

RUN cargo install sqlx-cli@^0.7 --locked --no-default-features --features rustls,postgres

FROM alpine:3.20 AS runtime

WORKDIR /app

COPY --from=sqlx /usr/local/cargo/bin/sqlx sqlx

COPY --from=builder /app/target/release/ztp ztp

COPY configuration configuration

COPY migrations migrations

ENV APP_ENVIRONMENT=production

CMD ["./ztp"]

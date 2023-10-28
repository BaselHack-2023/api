FROM rust:1.73.0 AS builder
WORKDIR /usr/src/api
COPY . .
RUN cargo install --path .

FROM debian:bullseye
RUN apt-get update && apt-get install -y libssl-dev pkg-config libpq-dev && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/api /usr/local/bin/api
CMD ["api"]
# syntax=docker/dockerfile:1.4
FROM rust:buster AS base

ENV USER=root

WORKDIR /code
RUN cargo init
COPY Cargo.toml /code/Cargo.toml
RUN cargo fetch
COPY . /code

FROM base AS development

EXPOSE 8080

CMD [ "cargo", "run", "--offline" ]

FROM base AS dev-envs

EXPOSE 8000
RUN <<EOF
apt-get update
apt-get install -y --no-install-recommends git libpq-dev && rm -rf /var/lib/apt/lists/*
EOF

RUN <<EOF
useradd -s /bin/bash -m vscode
groupadd docker
usermod -aG docker vscode
EOF
# install Docker tools (cli, buildx, compose)
COPY --from=gloursdocker/docker / /
CMD [ "cargo", "run", "--offline" ]

FROM base AS builder

RUN cargo build --release --offline

FROM debian:buster-slim

RUN apt-get update && apt-get install -y --no-install-recommends libpq-dev && rm -rf /var/lib/apt/lists/*
EXPOSE 8080

COPY --from=builder /code/target/release/api /api

CMD [ "/api" ]

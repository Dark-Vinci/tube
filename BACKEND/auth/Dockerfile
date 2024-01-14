#!/bin/sh
FROM rust:latest

RUN cargo install cargo-watch

ARG name=auth
ARG config=123
ARG service_token=0000
ARG port=3000
ARG environment=development

ENV CONFIG=${config}
ENV SERVICE_TOKEN=${service_token}
ENV NAME=${name}
ENV PORT=${port}
ENV ENVIRONMENT=${environment}

COPY sdk /app/backend/sdk

WORKDIR /app/backend/auth

COPY auth/Cargo.toml /app/backend/auth/Cargo.toml
COPY auth/Cargo.lock /app/backend/auth/Cargo.lock

RUN apt-get update && \
    apt-get install -y protobuf-compiler

RUN cargo build

COPY auth /app/backend/auth

CMD ["cargo", "watch", "-c", "-w", "src", "-x", "run"]

EXPOSE $PORT

#!/bin/sh
FROM rust:latest

RUN cargo install cargo-watch

ARG name=gate
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

WORKDIR /app/backend/gate

COPY gate/Cargo.toml /app/backend/gate/Cargo.toml
COPY gate/Cargo.lock /app/backend/gate/Cargo.lock

RUN apt-get update && \
    apt-get install -y protobuf-compiler

RUN cargo build

COPY gate /app/backend/gate

CMD ["cargo", "watch", "-c", "-w", "src", "-x", "run"]

EXPOSE $PORT

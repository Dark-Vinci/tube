#!/bin/sh
FROM rust:latest

ARG name=posts
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

WORKDIR /app/backend/posts

RUN apt-get update && \
    apt-get install -y protobuf-compiler

COPY posts /app/backend/posts

RUN cargo build

CMD if [ "${ENVIRONMENT}" = production ]; then \
    cargo run --release; \
    [./app/backend/posts/target/release/posts]; \
else \
    cargo install cargo-watch; \
    cargo watch -c -w src -x run; \
fi

EXPOSE $PORT

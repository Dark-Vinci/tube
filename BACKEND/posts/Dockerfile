#!/bin/sh

FROM rust:latest as sdk_build

WORKDIR /

COPY ./sdk .

RUN cargo build

FROM rust:latest

WORKDIR /app/backend/posts

ARG name=posts
ARG config=123
ARG service_token=0000
ARG port=3000
ARG env=development

WORKDIR /app/backend/posts

COPY ./posts/Cargo.toml /app/backend/posts
COPY ./posts/Cargo.lock /app/backend/posts

COPY --from=sdk_build /target/release/sdk /app/backend/sdk

COPY ./posts /app/backend/posts

ENV CONFIG=${config}
ENV SERVICE_TOKEN=${service_token}
ENV NAME=${name}
ENV PORT=${port}
ENV ENV=${env}

CMD if [ "${ENV}" = production ]; then \
    cargo run --release; \
    [./app/backend/posts/target/release/posts]; \
else \
    cargo install cargo-watch; \
    cargo watch -c -w src -x run; \
fi

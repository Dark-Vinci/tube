#!/bin/sh

FROM rust:latest as sdk_build

WORKDIR /

COPY ./SDK .

RUN cargo build

FROM rust:latest

WORKDIR /app/backend/auth

ARG name=auth
ARG config=123
ARG service_token=0000
ARG port=3000
ARG env=development

WORKDIR /app/backend/auth

COPY ./auth/Cargo.toml /app/backend/auth
COPY ./auth/Cargo.lock /app/backend/auth

COPY --from=sdk_build /target/release/sdk /app/backend/sdk

COPY ./auth /app/backend/auth

ENV CONFIG=${config}
ENV SERVICE_TOKEN=${service_token}
ENV NAME=${name}
ENV PORT=${port}
ENV ENV=${env}

CMD if [ "${ENV}" = production ]; then \
    cargo run --release; \
    [./app/backend/auth/target/release/auth]; \
else \
    cargo install cargo-watch; \
    cargo watch -c -w src -x run; \
fi

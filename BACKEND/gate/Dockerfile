FROM rust:1.67 as builder

WORKDIR /app/user/sdk

COPY ./sdk /app/user/sdk

RUN cargo run build

FROM rust:1.67

WORKDIR /app/user/gate

ARG port=3001
ARG environment=development

COPY ./auth/cargo.toml /app/user/gate
COPY ./cargo.lock ./

RUN cargo build

COPY --from=build_sdk / /app/user/sdk

COPY ./auth /app/user/auth

ENV PORT=${port}
ENV ENVIRONMENT=${environment}

CMD if [ enviroment == production ]; then \
     cargo build --release && ./app/user/auth/target/release/auth; \
else \
     cargo run --watch;
fi

EXPOSE $PORT
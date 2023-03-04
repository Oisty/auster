ARG RUST_VERSION=1.67
FROM node:19-slim as frontend-builder

RUN apt-get update && \
    DEBIAN_FRONTEND=noninteractive apt-get install -y --no-install-recommends \
        build-essential \
        ca-certificates \
        curl \
        git \
        ssh \
        libssl-dev \
        pkg-config && \
    apt-get clean && \
    rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY web /app
RUN npm ci

FROM rustlang/rust:nightly-buster-slim

ENV SQLX_OFFLINE true
ENV APP_ENVIRONMENT production

RUN apt-get update && \
    DEBIAN_FRONTEND=noninteractive apt-get install -y --no-install-recommends \
        pkg-config \
        libssl-dev && \
    apt-get clean && \
    rm -rf /var/lib/apt/lists/*

COPY ./ ./
COPY --from=builder /app/dist /web/dist

RUN cargo build --release

ENTRYPOINT ["./target/release/auster"]

EXPOSE 8000

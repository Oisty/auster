FROM rustlang/rust:nightly-buster-slim

ENV SQLX_OFFLINE true
ENV APP_ENVIRONMENT production
ENV BASE_URL localhost

RUN apt-get update && \
    DEBIAN_FRONTEND=noninteractive apt-get install -y --no-install-recommends \
        pkg-config \
        libssl-dev && \
    apt-get clean && \
    rm -rf /var/lib/apt/lists/*

COPY ./ ./

RUN cargo build --release

ENTRYPOINT ["./target/release/auster"]

EXPOSE 8000

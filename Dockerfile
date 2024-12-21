FROM rust:latest AS builder
WORKDIR /app

COPY Cargo.toml Cargo.lock ./
COPY src ./src

RUN cargo build --release

RUN ls

FROM rust:latest
WORKDIR /app

RUN apt-get update && apt-get install -y && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/dualwrites /usr/local/bin/

CMD ["dualwrites"]

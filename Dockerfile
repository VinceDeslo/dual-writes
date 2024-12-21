FROM rust:latest AS builder
WORKDIR /app

COPY . .

RUN apt-get update && apt-get install -y protobuf-compiler curl unzip && rm -rf /var/lib/apt/lists/*

RUN curl -LO https://github.com/protocolbuffers/protobuf/releases/download/v29.2/protoc-29.2-linux-x86_64.zip \
    && unzip protoc-29.2-linux-x86_64.zip -d /usr/local \
    && rm protoc-29.2-linux-x86_64.zip

ENV PATH="/usr/local/bin:${PATH}"

RUN echo 'protoc version: $(protoc --version)'

RUN cargo build --release

FROM rust:latest
WORKDIR /app

COPY --from=builder /app/target/release/dualwrites /usr/local/bin/

EXPOSE 50051

CMD ["dualwrites"]

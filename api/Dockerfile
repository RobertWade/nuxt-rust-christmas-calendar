# Dockerfile
FROM rust:1.90.0-slim

WORKDIR /usr/src/app

COPY Cargo.toml Cargo.lock ./

RUN mkdir src && echo "fn main() {}" > src/main.rs

RUN cargo build --release && rm -rf src

COPY . .

EXPOSE 8080

CMD ["cargo", "run", "--release"]
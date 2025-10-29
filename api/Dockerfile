# Dockerfile
FROM rust:latest

WORKDIR /usr/src/app

COPY Cargo.toml Cargo.lock ./

RUN mkdir src && echo "fn main() {}" > src/main.rs

RUN cargo build --release && rm -rf src

COPY . .

EXPOSE 8080

CMD ["cargo", "run", "--release"]
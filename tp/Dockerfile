FROM rustlang/rust:nightly-slim

WORKDIR /app
COPY Cargo.toml .
COPY src/ src/

RUN USER=root cargo build 
CMD ["./target/debug/tp"]
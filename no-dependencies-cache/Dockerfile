FROM rust:1.73 as builder
WORKDIR /app
COPY . /app
RUN cargo build --release
CMD ["./target/release/no-dependencies-cache"]

FROM rust:1.73
WORKDIR /app
RUN cargo init
RUN cargo build --release
CMD ["./target/release/app"]
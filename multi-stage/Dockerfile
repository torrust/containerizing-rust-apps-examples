# This is the first stage. This image is used to compile the Rust application.
FROM rust:1.73 as builder
WORKDIR /app
RUN cargo init
# Install the package in the current directory
RUN cargo install --path .
CMD ["./target/release/app"]

# This is the production stage.
# The slim image does not contain the common packages contained in the default tag and only contains the minimal packages needed to run rust.
FROM debian:bookworm-slim
RUN apt-get update && apt-get install && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/app /usr/local/bin/app
CMD ["app"]
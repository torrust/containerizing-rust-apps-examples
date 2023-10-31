FROM rust:1.73
WORKDIR /app
RUN curl -L --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash
RUN cargo binstall --no-confirm cargo-chef cargo-nextest
RUN cargo chef --version && cargo nextest --version
RUN cargo init
RUN cargo build --release
CMD ["./target/release/app"]
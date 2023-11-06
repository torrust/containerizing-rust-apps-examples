FROM rust:1.73

WORKDIR /app

RUN cargo init
RUN cargo build --release

RUN groupadd -r appuser
RUN useradd -r -u 1001 -g appuser appuser

USER appuser

CMD ["./target/release/app"]
FROM rust:1.73 as chef

WORKDIR /app

# Install cargo-chef
RUN cargo install cargo-chef --locked

# Examines your project and builds a recipe that captures the set of information required to build your dependencies
FROM chef AS planner

COPY . .

RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder 

COPY --from=planner /app/recipe.json recipe.json

# Build dependencies - this is the caching Docker layer!
RUN cargo chef cook --release --recipe-path recipe.json

COPY . .

# Build the application for release. Since dependencies are cached, this will only build your code
RUN cargo build --release

CMD ["./target/release/dependencies-cache-with-cargo-chef"]

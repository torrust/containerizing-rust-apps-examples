FROM rust:1.73 as nextest
RUN cargo install cargo-nextest --locked

FROM nextest AS builder
WORKDIR /build/src
COPY . /build/src
RUN cargo build
RUN cargo nextest archive --tests --benches --examples --workspace --all-targets --all-features --archive-file /build/archiving-and-reusing-builds.tar.zst
CMD ["/build/src/target/debug/archiving-and-reusing-builds"]

FROM nextest AS tester
WORKDIR /test
COPY . /test/src/
COPY --from=builder \
    /build/archiving-and-reusing-builds.tar.zst \
    /test/archiving-and-reusing-builds.tar.zst
RUN cargo nextest run --workspace-remap /test/src/ --extract-to /test/src/ --no-run --archive-file /test/archiving-and-reusing-builds.tar.zst
RUN cargo nextest run --workspace-remap /test/src/ --target-dir-remap /test/src/target/ --cargo-metadata /test/src/target/nextest/cargo-metadata.json --binaries-metadata /test/src/target/nextest/binaries-metadata.json
# TODO: why do we need to build again?
RUN cd ./src && cargo build 
RUN mkdir -p /app/bin/; cp -l /test/src/target/debug/archiving-and-reusing-builds /app/bin/archiving-and-reusing-builds
CMD ["/app/bin/archiving-and-reusing-builds"]

FROM nextest AS runtime
WORKDIR /app
COPY --from=tester /app/bin/archiving-and-reusing-builds /app/
CMD ["/app/archiving-and-reusing-builds"]

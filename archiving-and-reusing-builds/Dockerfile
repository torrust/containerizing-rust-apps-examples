## Fisrt stage to isntall the nextest tool
FROM rust:1.73 as nextest
RUN cargo install cargo-nextest --locked

## Second stage to build the applciation and package it with nextest
FROM nextest AS builder
WORKDIR /build/src
COPY . /build/src
RUN cargo build
# We archive the build artifacts to reuse them in the next stage
# NOTICE: the application must contains at least one integration test in order
# include the binary in the archive.
# See: https://github.com/nextest-rs/nextest/issues/423
RUN cargo nextest archive --tests --benches --examples --workspace --all-targets --all-features --archive-file /build/archiving-and-reusing-builds.tar.zst
CMD ["/build/src/target/debug/archiving-and-reusing-builds"]

## Third stage to test the application
FROM nextest AS tester
WORKDIR /test
COPY . /test/src/
COPY --from=builder \
    /build/archiving-and-reusing-builds.tar.zst \
    /test/archiving-and-reusing-builds.tar.zst
# We extract the build artifacts from the archive
RUN cargo nextest run --workspace-remap /test/src/ --extract-to /test/src/ --no-run --archive-file /test/archiving-and-reusing-builds.tar.zst
# We run the tests. We override the default target-dir to use the application binary,
# otherwise it would be created in a temporary directory and we wouln't able the copy it
# in the next stage.
RUN cargo nextest run --workspace-remap /test/src/ --target-dir-remap /test/src/target/ --cargo-metadata /test/src/target/nextest/cargo-metadata.json --binaries-metadata /test/src/target/nextest/binaries-metadata.json
RUN mkdir -p /app/bin/; cp -l /test/src/target/debug/archiving-and-reusing-builds /app/bin/archiving-and-reusing-builds
CMD ["/app/bin/archiving-and-reusing-builds"]

## Fourth stage to run the application in production
FROM nextest AS runtime
WORKDIR /app
# We take the application binary from the tester stage to ensure the binary we 
# use has passed the tests.
COPY --from=tester /app/bin/archiving-and-reusing-builds /app/
CMD ["/app/archiving-and-reusing-builds"]

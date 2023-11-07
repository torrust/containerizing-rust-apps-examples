# syntax=docker/dockerfile:latest

## Base Builder Image
FROM rust:bookworm as chef
WORKDIR /tmp
RUN curl -L --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash
RUN cargo binstall --no-confirm cargo-chef cargo-nextest

## Tester Image
FROM rust:slim-bookworm as tester
WORKDIR /tmp
RUN apt-get update; apt-get install -y curl; apt-get autoclean
RUN curl -L --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash
RUN cargo binstall --no-confirm cargo-nextest

## Su Exe Compile
FROM docker.io/library/gcc:bookworm as gcc
COPY ./contrib/dev-tools/su-exec/ /usr/local/src/su-exec/
RUN cc -Wall -Werror -g /usr/local/src/su-exec/su-exec.c -o /usr/local/bin/su-exec; chmod +x /usr/local/bin/su-exec

## Chef Prepare (look at project and see wat we need)
FROM chef AS recipe
WORKDIR /build/src
COPY . /build/src
RUN cargo chef prepare --recipe-path /build/recipe.json

## Cook (release)
FROM chef AS dependencies
WORKDIR /build/src
COPY --from=recipe /build/recipe.json /build/recipe.json
RUN cargo chef cook --tests --benches --examples --workspace --all-targets --all-features --recipe-path /build/recipe.json --release
RUN cargo nextest archive --tests --benches --examples --workspace --all-targets --all-features --archive-file /build/temp.tar.zst --release  ; rm -f /build/temp.tar.zst

## Build Archive (release)
FROM dependencies AS build
WORKDIR /build/src
COPY . /build/src
RUN cargo nextest archive --tests --benches --examples --workspace --all-targets --all-features --archive-file /build/full-example.tar.zst --release

## Extract and Test (release)
FROM tester as test
WORKDIR /test
COPY . /test/src
COPY --from=build \
  /build/full-example.tar.zst \
  /test/full-example.tar.zst
RUN cargo nextest run --workspace-remap /test/src/ --extract-to /test/src/ --no-run --archive-file /test/full-example.tar.zst
RUN cargo nextest run --workspace-remap /test/src/ --target-dir-remap /test/src/target/ --cargo-metadata /test/src/target/nextest/cargo-metadata.json --binaries-metadata /test/src/target/nextest/binaries-metadata.json

RUN mkdir -p /app/bin/; cp -l /test/src/target/release/full-example /app/bin/full-example
RUN chown -R root:root /app; chmod -R u=rw,go=r,a+X /app; chmod -R a+x /app/bin

## Runtime
FROM gcr.io/distroless/cc-debian12:debug as runtime
RUN ["/busybox/cp", "-sp", "/busybox/sh","/busybox/cat","/busybox/ls","/busybox/env", "/bin/"]
COPY --from=gcc --chmod=0555 /usr/local/bin/su-exec /bin/su-exec
ARG USER_ID=1000
COPY --chmod=0555 ./share/container/entry_script_sh /usr/local/bin/entry.sh
ENTRYPOINT ["/usr/local/bin/entry.sh"]

## Release Runtime
FROM runtime as release
COPY --from=test /app/ /usr/
CMD ["/usr/bin/full-example"]

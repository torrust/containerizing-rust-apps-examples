## Compile su-exec
FROM docker.io/library/gcc:bookworm as gcc
COPY ./contrib/dev-tools/su-exec/ /usr/local/src/su-exec/
RUN cc -Wall -Werror -g /usr/local/src/su-exec/su-exec.c -o /usr/local/bin/su-exec; chmod +x /usr/local/bin/su-exec

## Application
FROM rust:1.73 as builder
WORKDIR /app
RUN cargo init
RUN cargo build --release
CMD ["./target/release/app"]

## Runtime
FROM gcr.io/distroless/cc-debian12:debug as runtime
RUN ["/busybox/cp", "-sp", "/busybox/sh","/busybox/cat","/busybox/ls","/busybox/env", "/bin/"]

COPY --from=builder /app/target/release/app /usr/local/bin/app
COPY --from=gcc --chmod=0555 /usr/local/bin/su-exec /bin/su-exec
COPY --chmod=0555 ./share/container/entry_script_sh /usr/local/bin/entry.sh

ENTRYPOINT ["/usr/local/bin/entry.sh"]
CMD ["/usr/local/bin/app"]
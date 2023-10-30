# Multi Stage

In this example, we will build a Rust application using a multi-stage Dockerfile.
First, we will build the application using the Rust toolchain, then we will copy
the binary into a new image that only contains the binary and its dependencies.

Build the docker image:

```console
docker build -t docker-rust-app-multi-stage .
```

Run the docker image:

```console
docker run --rm -it docker-rust-app-multi-stage
```

Output:

```console
Hello, world!
```

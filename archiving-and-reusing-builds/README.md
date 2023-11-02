# Archiving And Reusing Builds

In this example, `cargo-nextest` is used to pass the build application from one docker stage to another.

[`cargo nextest archive`](https://nexte.st/book/reusing-builds.html) allows archiving
builds on one machine (or docker stage), and then extracting the archive to run tests on another machine.

> NOTICE: in order to pass the application binary with `cargo nextest archive`, the application must contain an integration test. Read this issue <https://github.com/nextest-rs/nextest/issues/423> for more info.

Build the docker image:

```console
docker build -t docker-rust-app-archiving-and-reusing-builds .
```

Run the docker image:

```console
docker run --rm -it docker-rust-app-archiving-and-reusing-builds
```

Example output:

```console
Hello World!
```

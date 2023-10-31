# Dependencies Cache With Cargo Chef

In this example, the application has some dependencies that are cached by using docker layers.

It's done by using [cargo chef](https://github.com/LukeMathWalker/cargo-chef).

Build the docker image without cache:

```console
time docker build --no-cache -t docker-rust-app-dependencies-cache-with-cargo-chef .
```

It takes 0m20.639s.

Build the docker image again caching the dependencies:

```console

```console
time docker build -t docker-rust-app-dependencies-cache-with-cargo-chef .
```

It should use the cache for the first build and it should not download and build dependencies on the second build.

It takes 0m1.414s.

Run the docker image:

```console
docker run --rm -it docker-rust-app-dependencies-cache-with-cargo-chef
```

Example output:

```console
Great to see you, Garsansan!
```

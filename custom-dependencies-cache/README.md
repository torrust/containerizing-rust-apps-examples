# Custom dependencies cache (without cargo chef)

In this example, the application has some dependencies that are not cached by using docker layers.

It's done the way it was done before having [cargo chef](https://github.com/LukeMathWalker/cargo-chef).

See this article [Plumbing the Deps of the Crate: Caching Rust Docker Builds](https://dev.to/mgattozzi/plumbing-the-deps-of-the-crate-caching-rust-docker-builds-2e48).

Build the docker image without cache:

```console
time docker build --no-cache -t docker-rust-app-custom-dependencies-cache .
```

It takes 0m7.932s.

Build the docker image again caching the dependencies:

```console

```console
time docker build -t docker-rust-app-custom-dependencies-cache .
```

It should use the cache for the first build and it should not download and build dependencies on the second build. 

It takes 0m1.598s.

Run the docker image:

```console
docker run --rm -it docker-rust-app-custom-dependencies-cache
```

Example output:

```console
Great to see you, Shehalnes!
```

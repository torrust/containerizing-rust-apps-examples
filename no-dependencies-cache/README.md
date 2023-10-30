# No dependencies cache

In this example, the application has some dependencies that are not cached with docker layers.

If you change something in the application but do not change the dependencies,
the dependencies will be downloaded and compiled again.

Build the docker image:

```console
docker build -t docker-rust-app-no-dependencies-cache .
```

Run the docker image:

```console
docker run --rm -it docker-rust-app-no-dependencies-cache
```

Example output:

```console
Hail, Selaelhaltherthus! Well met!
```

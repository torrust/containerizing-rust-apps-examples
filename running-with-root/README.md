# Running With Root

Build the docker image:

```console
docker build -t docker-rust-app-running-with-root .
```

Run the docker image:

```console
docker run --rm -it docker-rust-app-running-with-root whoami
```

Output:

```console
root
```

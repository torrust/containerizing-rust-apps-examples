# Full Example

This example uses all the patterns defined in the other examples.

It's a simplified version of the [`Containerfile`](https://github.com/torrust/torrust-tracker/blob/develop/Containerfile)
used by the [Torrust Tracker](https://github.com/torrust/torrust-tracker).

Since you can build Rust apps in debug and release mode, the official one has two
flows of building: one for `debug` and one for `release`.

The `debug` building flow has been removed to keep the example simpler and they
are mostly the same.

Build the docker image:

```console
docker build -t docker-rust-app-full-example .
```

Run the docker image:

```console
docker run --rm -it -e USER_ID=1001 docker-rust-app-full-example
```

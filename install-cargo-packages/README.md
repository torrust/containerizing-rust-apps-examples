# Install Cargo Packages

Sometimes you need to install Rust packages in your `Dockerfile`. Instead of
using `cargo install` or download the binaries or follow the steps provided by each
package, you can use `cargo binstall`.

Repo: <https://github.com/cargo-bins/cargo-binstall>.

Build the docker image:

```console
docker build -t docker-rust-app-install-cargo-packages .
```

Run the docker image:

```console
docker run --rm -it docker-rust-app-install-cargo-packages
```

Output:

```console
Hello, world!
```

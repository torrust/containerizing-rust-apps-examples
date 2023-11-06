# Running With Regular User

You can create custom users in your Dockerfile. This is useful for security reasons.
If your application is compromised, the attacker will not have root access to your system.

But be careful, if you create a user with a UID that already exists in your system, the
user will have the same permissions as the existing user.

Build the docker image:

```console
docker build -t docker-rust-app-running-with-regular-user .
```

Run the docker image:

```console
docker run --rm -it docker-rust-app-running-with-regular-user whoami
```

Run the docker image overwriting the user:

```console
docker run --user 1000 --rm -it docker-rust-app-running-with-regular-user
```

Output:

```console
appuser
```

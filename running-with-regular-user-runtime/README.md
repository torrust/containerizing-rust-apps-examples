# Running With Regular User Created At Runtime

You can create custom users in your Dockerfile. This is useful for security reasons.
If your application is compromised, the attacker will not have root access to your system.

But be careful, if you create a user with a UID that already exists in your system, the
user will have the same permissions as the existing user.

In this example, the user is created dynamically at runtime. The UID is passed 
as an environment variable.

Build the docker image:

```console
docker build -t docker-rust-app-running-with-regular-user-runtime .
```

Run the docker image:

```console
$ docker run --rm -it \
    -e USER_ID=1001 \
    docker-rust-app-running-with-regular-user-runtime
+ '[' -z 1001 ]
+ adduser --disabled-password --shell /bin/sh --uid 1001 appuser
+ cd /home/appuser
+ exec /bin/su-exec appuser /usr/local/bin/app
Hello, world!
```

If you want to confirm that the docker container is running as the user with ID `1001`,
you can overwrite the command. This is a docker common pattern, to use `ENTRYPOINT`
for the executable and `CMD` for the arguments to that executable because `CMD`
can be easily overridden.

This command will run the container and sleep for 30 seconds.

```console
docker run --rm -it \
    -e USER_ID=1001 \
    docker-rust-app-running-with-regular-user-runtime sleep 30
```

In the meantime, you can check that the container is running as the user with ID `1001`.

```console
ps -aux | grep sleep
1001      154754  0.0  0.0   1300   256 pts/0    Ss+  18:05   0:00 sleep 30
```

FROM rust:1.73 as builder

WORKDIR /app

# Copy over the manifest files
COPY Cargo.toml Cargo.lock /app/

# Create a dummy main.rs to build dependencies
RUN mkdir src && echo "fn main() { println!(\"if you see this, the build broke\"); }" > src/main.rs

# This build step will cache the dependencies as they're not changed
RUN cargo build --release

# Now, remove the dummy main.rs and copy your source code
COPY . /app

# You'll need to update the last modified of the main.rs file to inform cargo to rebuild it
RUN touch -a -m ./src/main.rs

# Build the application for release. Since dependencies are cached, this will only build your code
RUN cargo build --release

CMD ["./target/release/custom-dependencies-cache"]

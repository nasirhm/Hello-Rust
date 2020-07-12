# Use a minimal image
FROM rustlang/rust:nightly-slim AS build

# Where we will build the program
WORKDIR /src/Rocket

# Copy source code into the container
COPY . .

# Build the program in release mode
RUN cargo build --release

# Create the runtime image
FROM ubuntu:18.04

# Copy the compiled service binary
COPY --from=build /src/Rocket/target/release/Rocket /usr/local/bin/Rocket

# Start the helloworld service on container boot
CMD ["usr/local/bin/Rocket"]

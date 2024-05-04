# Use a Rust base image
FROM rust:latest

# Set up the working directory
WORKDIR /usr/src/app

# Install any additional dependencies needed for your build process
# For example, you may need to install protoc, cbindgen, and ld linker
RUN apt-get update && \
    apt-get install -y protobuf-compiler && \
    apt-get install -y binutils && \
    apt-get install -y lld && \
    cargo install cbindgen

# Copy the source code into the container
COPY . .

RUN make build

# Run the make test command inside the container
RUN make test

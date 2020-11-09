# Version of Rust I want to use
FROM rust:1.47

# Working directory within the container
WORKDIR /usr/src/warp_example

# Copy the files from my host to the working directory
COPY . .

# Expose the post that the server is running on
EXPOSE 3030

# Build project and add built executable to your PATH
RUN cargo install --path .

# Run the executable
CMD ["warp_post_example"]

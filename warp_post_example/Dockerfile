# Version of Rust I want to use
FROM rust:1.47

# Working directory within the container
WORKDIR /usr/src/warp_post_example

# Copy the files from my host to the working directory
COPY . .

# Expose the post that the server is running on
EXPOSE 3030

# Build project and add built executable to your PATH
RUN cargo install --path .

ENV RUST_LOG=info

# Run the executable with log level set to Info
# Map stdout and stderr to "log_file"
CMD ["warp_post_example"]

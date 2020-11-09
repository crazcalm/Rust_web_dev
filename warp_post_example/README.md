# Warp Post Example

## Container steps
### Build container
The below comamand will build the container using the instructions in the Dockerfile

```
podman build -t warp_post_example .
```

### Run container

```
podman run -it --rm -p 8080:3030 --name warp warp_post_example
```


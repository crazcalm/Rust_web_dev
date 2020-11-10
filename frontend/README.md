# Frontend

## Develop Environment
### Run Docker
- `:ro` is for read-only

**Note**: Change the volume host machine path as needed

```
podman run -p 8080:80 --name some-nginx -v /home/crazcalm/Documents/Github/Rust_web_dev/frontend/:/usr/share/nginx/html:ro -d nginx
```

### Test Build
#### Build Docker Container
- `-t` name of image

```
podman build -t some-content-nginx .
```
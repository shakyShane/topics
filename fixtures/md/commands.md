# Command: Cargo Test

```toml
DOCKER_CLIENT_ = { valuesFrom = "global-vars", path = "images.client" }
```

```shell
docker build -t ${DOCKER_CLIENT_tag} ${DOCKER_CLIENT_context}
cargo test
```
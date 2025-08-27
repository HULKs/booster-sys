#! /bin/sh
podman run -it --rm \
  --network=host \
  -v .:/booster-dev/:Z \
  -v booster-dev-persistent:/booster-dev-persistent/:z \
  localhost/booster-sys-container:latest

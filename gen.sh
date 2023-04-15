# !/bin/bash

docker run --rm -v "${PWD}:/local" --user $(id -u):$(id -g)  openapitools/openapi-generator-cli:v6.1.0 generate \
    --skip-validate-spec \
    -i $1 \
    -g rust \
    -o /local/lib \
    --additional-properties=packageName=gitea-rs \
    --additional-properties=projectVersion=1.19.0

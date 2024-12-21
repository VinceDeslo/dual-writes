__default:
    just --list

docker-build:
    docker build -t dualwrites:latest .

infra-up:
    cd ./infra && ./kube-up.sh

infra-down:
    cd ./infra && ./kube-down.sh

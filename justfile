__default:
    just --list

docker-build:
    docker build -t dualwrites:latest .

infra-up:
    cd ./infra && ./kube-up.sh

infra-down:
    cd ./infra && ./kube-down.sh

gen-proto-descriptor:
    protoc --proto_path=proto --descriptor_set_out=proto/analytics-descriptor.pb --include_imports proto/analytics.proto

grpc-list:
    ./scripts/grpc-list.sh

grpc-request:
    ./scripts/grpc-request.sh

k8s-request:
    ./scripts/k8s-request.sh

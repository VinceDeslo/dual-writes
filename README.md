# Dualwrites

A small test playground to mess around with NATS event propagation on DB transactions.

### Design

```mermaid
flowchart LR
    A[Client] -->|Process rpc| B(Analytics Service)
    B --> C{Tx}
    C -->|Store| D[DB]
    C -->|Event| E[NATS]
```

### Dev shell

```shell
nix develop
```

### Commands

```shell
just up # Setup local deployment
just down # Tear down local deployment
just docker-build # Build source and local image
just infra-up # spin up a namespace and deployment
just infra-down # teardown namespace and deployment
just grpc-list # list service operations locally
just grpc-request # test a request locally
just k8s-request # test a request on the local k8s pod
```

### Local testing

```
cargo run
just grpc-request
```

### k8s testing
```
just up
just k8s-request
just down
```

### nats pod testing
```
just up
nats context add localhost --description "Localhost"

k get pods
k port-forward pod/<pod-name> 4222:4222
nats pub events.test "testing pod"
```

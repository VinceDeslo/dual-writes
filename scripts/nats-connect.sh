#!/bin/bash

LOCAL_PORT=4222
K8S_PORT=4222
SERVICE_NAME="nats-service"

echo "Port-forwarding $SERVICE_NAME"

kubectl -n localdev port-forward "svc/$SERVICE_NAME" $LOCAL_PORT:$K8S_PORT &
PF_PID=$!

echo "Port-forward PID: $PF_PID"
trap "kill $PF_PID 2>/dev/null || true" EXIT
sleep 2

if ! nats context list | grep -q "localhost"; then
    echo "Adding localhost NATS context..."
    nats context add localhost --server "nats://localhost:4222" --description "Localhost K8s NATS"
fi

echo "NATS connection ready! You can now use 'nats' commands with '--context localhost'"
echo "Example: nats pub --context localhost events.test \"hello\""

echo "Press Ctrl+C to exit..."
wait $PF_PID
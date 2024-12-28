#!/bin/bash

LOCAL_PORT=50051
K8S_PORT=50051
SERVICE_NAME="dualwrites-service"
PAYLOAD_FILE="scripts/payload.json"

echo "Port-forwarding $SERVICE_NAME"

kubectl -n localdev port-forward "svc/$SERVICE_NAME" $LOCAL_PORT:$K8S_PORT > /dev/null 2>&1 &
PF_PID=$!

echo "Port-forward PID: $PF_PID"
trap "kill $PF_PID 2>/dev/null || true" EXIT
sleep 2

echo "Hitting grpc endpoint with $PAYLOAD_FILE"
grpcurl -plaintext -d @ localhost:50051 analytics.Analytics/Process < $PAYLOAD_FILE

echo "Press Ctrl+C to exit..."
wait $PF_PID
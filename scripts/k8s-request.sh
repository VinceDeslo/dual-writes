#!/bin/bash

LOCAL_PORT=50051
K8S_PORT=50051
SERVICE_NAME="dualwrites-service"

echo "Port-forwarding $SERVICE_NAME"

kubectl -n localdev port-forward "svc/$SERVICE_NAME" $LOCAL_PORT:$K8S_PORT > /dev/null 2>&1 &
BACKGROUND_PID=$!

echo "Port-forward PID: $BACKGROUND_PID"
sleep 1s

grpcurl -plaintext -d @ localhost:50051 analytics.Analytics/Process < scripts/payload.json

kill $BACKGROUND_PID
echo "Port-forward terminated."

#!/bin/bash

# Server
kubectl delete -f service.yaml
kubectl delete -f deployment.yaml

# NATS
kubectl delete -f nats-service.yaml
kubectl delete -f nats-deployment.yaml
kubectl delete -f nats-config.yaml

kubectl delete -f namespace.yaml

#!/bin/bash

kubectl apply -f namespace.yaml

# NATS
kubectl apply -f nats-config.yaml
kubectl apply -f nats-deployment.yaml
kubectl apply -f nats-service.yaml

# Server
kubectl apply -f deployment.yaml
kubectl apply -f service.yaml

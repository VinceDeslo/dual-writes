#!/bin/bash

kubectl delete -f service.yaml
kubectl delete -f deployment.yaml
kubectl delete -f nats-deployment.yaml
kubectl delete -f namespace.yaml

#!/bin/bash

grpcurl -plaintext -d @ localhost:50051 analytics.Analytics/Process < scripts/payload.json

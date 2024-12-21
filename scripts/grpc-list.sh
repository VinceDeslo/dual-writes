#!/bin/bash

grpcurl -plaintext localhost:50051 list
grpcurl -plaintext localhost:50051 describe analytics.Analytics
grpcurl -plaintext localhost:50051 describe analytics.AnalyticsRequest
grpcurl -plaintext localhost:50051 describe analytics.AnalyticsResponse
grpcurl -plaintext localhost:50051 describe analytics.AnalyticsData

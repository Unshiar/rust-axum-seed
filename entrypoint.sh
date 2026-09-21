#!/bin/sh
set -e

health_check() {
    echo "Running health check"
    if curl -f --silent "http://localhost:${SERVER_PORT}/health" > /dev/null; then
        echo "Health check succeeded"
        exit 0
    else
        echo "Health check failed"
        exit 1
    fi
}

if [ "$1" = "healthcheck" ]; then
    health_check
fi

exec "/usr/local/axum-app"

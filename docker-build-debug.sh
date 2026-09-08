#! /bin/sh
echo "(debug)Cleaning..."
cargo clean
echo ""

echo "(debug)Building docker image..."
docker build -t app:latest . -f Dockerfile.debug
echo ""

echo "(debug)Done."

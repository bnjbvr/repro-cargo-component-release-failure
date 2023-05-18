#!/bin/bash

echo "Building with "cargo-component build" works"
(cd ./modules && \
    cargo component build --target wasm32-unknown-unknown)

echo "Building with "cargo-component build --release" fails"
(cd ./modules && \
    cargo component build --release --target wasm32-unknown-unknown)

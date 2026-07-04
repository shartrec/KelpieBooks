#!/bin/bash
# build_prod.sh

# 1. Clean and compile the optimized Wasm frontend
cd frontend
trunk build --release

# 2. Compile the backend binary
cd ../backend
cargo build --release

# 3. Create a clean, production-ready bundle workspace
cd ..
mkdir -p dist/static
mkdir -p dist/templates

# Copy the binary and the generated frontend static output next to each other
cp target/release/kelpie-books dist/
cp -r backend/static/* dist/static/
cp -r backend/templates/* dist/templates/
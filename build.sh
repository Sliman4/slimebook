#!/bin/sh
# Set up
rustup target add wasm32-unknown-unknown
ROOT_DIR=$PWD
PROJECT_NAME="slimebook"

# Build mdbook
cd $ROOT_DIR/book
rm -rf book
mdbook build
cd $ROOT_DIR

# Build web4 contract
cd $ROOT_DIR/web4
cargo build --target wasm32-unknown-unknown --release
cd $ROOT_DIR

#!/bin/sh
# Build WASM package for office-print
# Requires: wasm-pack (cargo install wasm-pack)
# Output: crates/office-print/pkg/

wasm-pack build crates/office-print --target web --features wasm --out-dir pkg
echo "WASM package built → crates/office-print/pkg/"

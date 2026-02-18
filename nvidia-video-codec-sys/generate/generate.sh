#!/usr/bin/env bash
#
# generate.sh - Generate Rust FFI bindings from nv-codec-headers.
#
# Usage:
#   ./generate.sh <tag> <output_dir>
#
# Example:
#   ./generate.sh n13.0.19.0 src/bindings/v13_0
#
# This script:
#   1. Clones the specified tag from FFmpeg/nv-codec-headers (shallow)
#   2. Installs the headers into a temporary prefix
#   3. Runs bindgen to produce a single platform-independent binding file
#   4. Writes the output to <output_dir>/bindings.rs

set -euo pipefail

TAG="${1:?Usage: $0 <tag> <output_dir>}"
OUTPUT_DIR="${2:?Usage: $0 <tag> <output_dir>}"

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
WRAPPER_H="${SCRIPT_DIR}/wrapper.h"

# Temporary working directory
WORK_DIR="$(mktemp -d)"
trap 'rm -rf "${WORK_DIR}"' EXIT

echo "==> Cloning nv-codec-headers @ ${TAG}..."
git clone --depth 1 --branch "${TAG}" \
    https://github.com/FFmpeg/nv-codec-headers.git \
    "${WORK_DIR}/nv-codec-headers"

# Install headers into a local prefix so bindgen can find them
PREFIX="${WORK_DIR}/prefix"
make -C "${WORK_DIR}/nv-codec-headers" install PREFIX="${PREFIX}"

INCLUDE_DIR="${PREFIX}/include"

# Ensure output directory exists
mkdir -p "${OUTPUT_DIR}"

echo "==> Generating bindings..."
bindgen "${WRAPPER_H}" \
    --no-layout-tests \
    --no-doc-comments \
    --default-enum-style rust_non_exhaustive \
    --with-derive-default \
    --with-derive-hash \
    --with-derive-eq \
    --blocklist-item "FFNV_.*" \
    -- \
    -I "${INCLUDE_DIR}" \
    > "${OUTPUT_DIR}/bindings.rs"

echo "==> Done! Bindings written to ${OUTPUT_DIR}/bindings.rs"
ls -la "${OUTPUT_DIR}/"

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
#   3. Runs bindgen for Linux x86_64 and Windows x86_64 targets
#   4. Writes the output to <output_dir>/linux_x86_64.rs and windows_x86_64.rs

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

# Common bindgen flags
COMMON_FLAGS=(
    --no-layout-tests
    --no-doc-comments
    --default-enum-style rust_non_exhaustive
    --with-derive-default
    --with-derive-hash
    --with-derive-eq
    --blocklist-item "FFNV_.*"
    --blocklist-item "ffnv_.*"
    --raw-line "#![allow(non_upper_case_globals)]"
    --raw-line "#![allow(non_camel_case_types)]"
    --raw-line "#![allow(non_snake_case)]"
    --raw-line "#![allow(dead_code)]"
)

echo "==> Generating Linux x86_64 bindings..."
bindgen "${WRAPPER_H}" \
    "${COMMON_FLAGS[@]}" \
    -- \
    -I "${INCLUDE_DIR}" \
    -target x86_64-unknown-linux-gnu \
    > "${OUTPUT_DIR}/linux_x86_64.rs"

echo "==> Generating Windows x86_64 bindings..."
bindgen "${WRAPPER_H}" \
    "${COMMON_FLAGS[@]}" \
    -- \
    -I "${INCLUDE_DIR}" \
    -target x86_64-pc-windows-msvc \
    -D_WIN32 \
    -D_MSC_VER=1929 \
    -fms-extensions \
    > "${OUTPUT_DIR}/windows_x86_64.rs"

echo "==> Done! Bindings written to ${OUTPUT_DIR}/"
ls -la "${OUTPUT_DIR}/"

#!/usr/bin/env bash
set -euo pipefail

cargo +nightly install bootimage --version 0.10.6 2>/dev/null || true
cargo +nightly bootimage --manifest-path kernel/Cargo.toml

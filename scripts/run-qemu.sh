#!/usr/bin/env bash
set -euo pipefail

IMG=target/x86_64-blog_os/debug/bootimage-syclone_kernel.bin
if [[ ! -f "$IMG" ]]; then
  echo "Kernel image not found. Run ./build.sh first."
  exit 1
fi

qemu-system-x86_64 -drive format=raw,file="$IMG" -serial stdio -display none

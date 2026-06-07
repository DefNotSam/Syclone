# Syclone OS Prototype

An experimental VM-focused operating system prototype with a compatibility-aware file handling design.

## Goals
- Boot a minimal OS kernel in QEMU
- Provide a simple shell / status output
- Add support for downloading arbitrary files
- Add archive handling for `.zip`
- Provide a pluggable file-type runtime/compatibility layer for `.exe`, `.app`, and more

## Getting started

Requirements:
- Rust nightly toolchain
- `cargo bootimage`
- `qemu-system-x86_64`

Build and run:

```bash
./build.sh
./scripts/run-qemu.sh
```

## Project layout

- `kernel/` — Rust kernel crate with minimal boot code
- `docs/` — architecture notes and implementation plan
- `x86_64-blog_os.json` — custom target specification for the OS image

## Next steps

1. Boot the kernel in QEMU
2. Add a minimal shell and file system support
3. Add networking and downloader support
4. Add archive and file-type handler modules

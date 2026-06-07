# Syclone OS Architecture

## Overview
Syclone is an experimental OS prototype designed to run in a VM/emulator and handle diverse file types from multiple operating systems.

## Initial implementation goals
- Minimal Rust-based kernel booting in QEMU
- Basic screen output from the kernel
- Project structure for future file system, network, and compatibility modules

## Planned subsystems

### Boot / kernel
- `bootloader`-based Rust boot image
- `no_std`, `no_main` kernel entry
- Console output through VGA text buffer

### File handling
- Download manager for HTTP/HTTPS
- File catalog and persistence on virtual disk
- Pluggable file handlers by extension

### Archive support
- `.zip` unpacking and listing
- `.app` package metadata support
- Placeholder compatibility runtime for `.exe`

### Compatibility layer
- Detect file types by headers and extensions
- Provide sandboxed launchers for supported formats
- Add new handlers without changing kernel core

## Next milestones
1. Boot the kernel in QEMU
2. Add serial or VGA text console driver
3. Initialize a basic file catalog
4. Add network download support
5. Support `.zip` unpacking and package inspection

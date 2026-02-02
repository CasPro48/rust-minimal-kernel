# Rust Minimal Kernel

A minimalistic OS kernel written entirely in Rust.

## Features

- **No standard library** (`#![no_std]`) - runs on bare metal
- **VGA text mode** - 80x25 character display with colors
- **Panic handler** - custom panic handling for kernel panics
- **x86_64 architecture** - targets modern 64-bit processors

## Project Structure

```
rust-minimal-kernel/
├── Cargo.toml              # Package manifest
├── rust-toolchain.toml     # Nightly toolchain config
├── .cargo/
│   └── config.toml         # Build configuration
├── src/
│   ├── main.rs             # Kernel entry point
│   └── vga_buffer.rs       # VGA text buffer driver
└── linker.ld               # Linker script
```

## Prerequisites

- Rust nightly toolchain
- `rust-src` component
- QEMU (for running the kernel)

## Building

```bash
# Install required components (first time only)
rustup override set nightly
rustup component add rust-src llvm-tools-preview

# Build the kernel
cargo build

# Build in release mode
cargo build --release
```

## Running with QEMU

```bash
# Install QEMU if not present
sudo apt install qemu-system-x86

# Run the kernel
qemu-system-x86_64 -kernel target/x86_64-unknown-none/debug/rust-minimal-kernel
```

## Architecture

### VGA Text Buffer

The kernel writes directly to the VGA text buffer at memory address `0xB8000`. Each character is represented by 2 bytes:
- Byte 0: ASCII character code
- Byte 1: Color attribute (foreground + background)

### Memory Layout

- Kernel loaded at 1MB
- VGA buffer at 0xB8000
- Stack grows downward from high memory

## License

MIT License - see LICENSE file for details.

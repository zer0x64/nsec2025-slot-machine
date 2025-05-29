# Wonderlight Slot Machine

This repository contains the code and resources for the Wonderlight Slot Machine, developed for NorthSec 2025 CTF. The project simulates a casino slot machine with vulnerable Random Number Generators (RNGs) that could be exploited to win large sums of money.

## Overview

The Wonderlight Slot Machine is a Rust-based project that includes:
- A slot machine application
- A slot machine simulator to calculate win odds
- Custom cryptographic components for obfuscation
- GPIO support for physical hardware
- Solve code for the challenges.
- Various tools for maintenance and operation

## How to build

### Prerequisites

- Rust toolchain (latest stable version recommended)
- FUSE (required for AppImages)
- NPM
- Protocol Buffer Compiler (protoc)
- Specific version of tauri-cli. This can be installed using `cargo install tauri-cli --git https://github.com/tauri-apps/tauri.git --rev 72211beec3efcb02472e580e827c0e2a2d69f2cc`.
- ansible if you want to use the production build playbook
- Run `npm install` in the client/ directory.

### Building from development and testing
```
cargo tauri dev
```

For production builds. Note that you'll need to modify inventory.yaml to point to your own build container/virtual machines.
```
cd prod_build
ansible-playbook build.yaml -i inventory.yml
```

## Usage

### Controls

- SpaceBar - Lever + select button
- Up Arrow - Up button
- Down Arrow - Down button
- B - Insert badge
- Shift+B - Toggle badge "insert/remove" badge

### Troubleshooting

Depending on your setup, the following environment variables could help:
- `__NV_DISABLE_EXPLICIT_SYNC=1` (for Nvidia GPUs)
- `LIBGL_ALWAYS_SOFTWARE=1`
- `WEBKIT_DISABLE_COMPOSITING_MODE=1`
- `WEBKIT_DISABLE_DMABUF_RENDERER=1`
- `WAYLAND_DISPLAY=` (delete the variable, forces use of Xorg instead of Wayland)

If nothing works, try running it on a new Ubuntu VM.

## Project Structure

- `src/` - Main application code
- `slot-machine-crypto/` - Cryptographic functionality for obfuscation of 2nd reversing flag
- `slot-machine-procmacro/` - Procedural macros used for obfuscation of 2nd reversing flag
- `client/` - Frontend of the application
- `simulator/` - Slot machine simulator, used to test the win odds according to multiple bet strategy
- `capabilities/` - Tauri capability definitions
- `schemas/` - Protobuf data schemas
- `solve/` - Exploit code for all the flags

## Contributors

This track was created by [zer0x64](https://github.com/zer0x64), [OliPro007](https://github.com/OliPro007), [junior-n30](https://github.com/junior-n30) and [lle](https://github.com/lle).

## License

This project is dual-licensed under either:

- MIT License ([LICENSE-MIT](LICENSE-MIT))
- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))

at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you shall be dual licensed as above, without any additional terms or conditions.

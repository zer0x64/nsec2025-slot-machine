# Wonderlight Website

The `wonderlight-website` module provides the official website for the Wonderlight Slot Machine project.

## Overview

This component serves as the public-facing website for the Wonderlight Slot Machine, providing information, documentation, and potentially administrative interfaces for the slot machine system. It is built as a Rust web application.

## Features

- Official documentation for the Wonderlight Slot Machine
- Marketing and promotional content
- User guides and instructions
- Administrative interfaces (if applicable)
- Integration with the slot machine ecosystem

## Installation

### Prerequisites

- Rust toolchain (latest stable version recommended)
- Dependencies listed in the Cargo.toml file

### Building

```bash
# Navigate to the wonderlight-website directory
cd wonderlight-website

# Build the website
cargo build
```

## Usage

To run the website locally:

```bash
cargo run
```

By default, the website will be accessible at http://localhost:8080 (or another configured port).

## Project Structure

- `src/` - Source code for the website application
- `Cargo.toml` - Rust dependencies and project configuration

## Integration

This website integrates with the broader Wonderlight Slot Machine ecosystem by:

- Providing documentation for users and administrators
- Potentially offering interfaces for remote management
- Serving as a central information hub for the project
- Possibly connecting to slot machine instances for status monitoring

## Deployment

For production deployment:

```bash
cargo build --release
```

## License

This module is dual-licensed under either:

- MIT License ([LICENSE-MIT](../LICENSE-MIT))
- Apache License, Version 2.0 ([LICENSE-APACHE](../LICENSE-APACHE))

at your option.
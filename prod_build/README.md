# Wonderlight Slot Machine Production Build

The `prod_build` directory contains configuration and scripts for building production-ready versions of the Wonderlight Slot Machine software.

## Overview

This directory manages the production build process, providing consistent and reproducible builds for deployment. It includes configuration files that define build parameters, inventory management, and deployment specifications.

## Contents

- `build.yaml` - Build configuration and parameters
- `inventory.yaml` - Inventory of components and assets for production builds

## Purpose

The production build system serves several important functions:

- Creates optimized, release-ready builds
- Manages hardware-specific configuration
- Ensures consistent build environments
- Handles asset packaging and bundling
- Prepares deployable artifacts

## Usage

To create a production build:

```bash
# Navigate to the project root
cd nsec2025-slot-machine

# Run the production build process
cargo build --features prod
```

## Build Configuration

The `build.yaml` file defines various aspects of the build process, including:

- Target architectures
- Optimization levels
- Feature flags
- Asset inclusion
- Output formats

## Inventory Management

The `inventory.yaml` file maintains a record of:

- Required components
- Asset files
- Dependencies
- Version information

## Integration

The production build system integrates with:

- The main application code
- Asset management
- Dependency resolution
- Tauri configuration for application packaging

## License

This module is dual-licensed under either:

- MIT License ([LICENSE-MIT](../LICENSE-MIT))
- Apache License, Version 2.0 ([LICENSE-APACHE](../LICENSE-APACHE))

at your option.
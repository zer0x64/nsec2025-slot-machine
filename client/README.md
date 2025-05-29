# Wonderlight Slot Machine Client

The client module provides the web-based frontend interface for the Wonderlight Slot Machine project.

## Overview

This client application allows users to interact with the slot machine via a web interface. It's built with modern web technologies and communicates with the Rust-based backend.

## Technologies Used

- Svelte framework
- Vite build tool
- PostCSS for styling
- JavaScript/TypeScript

## Project Structure

- `src/` - Source code for the client application
- `static/` - Static assets (images, fonts, etc.)
- `package.json` - Dependencies and scripts
- `vite.config.js` - Vite configuration
- `svelte.config.js` - Svelte configuration

## Installation

```bash
# Navigate to the client directory
cd client

# Install dependencies
npm install
```

## Development

To run the development server:

```bash
npm run dev
```

This will start a local development server, typically at http://localhost:5173.

## Building for Production

```bash
npm run build
```

The built files will be output to the `dist` directory.

You can also simply build the full project and this frontend will be built as part of the main build process.

## License

This module is dual-licensed under either:

- MIT License ([LICENSE-MIT](../LICENSE-MIT))
- Apache License, Version 2.0 ([LICENSE-APACHE](../LICENSE-APACHE))

at your option.

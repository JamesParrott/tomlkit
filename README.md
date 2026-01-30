# tomlkit 🦆

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

**High-performance TOML validation, powered by Rust.**

`tomlkit` is a VS Code extension designed for developers who want instant, reliable TOML validation. By using the official Rust `toml` parser compiled to WebAssembly (WASM), we provide the same level of accuracy as `cargo` itself, right in your editor.

![tomlkit icon](icon.png)

## Features

- **Blazing Fast**: Incremental validation using an optimized `LineIndex` algorithm.
- **Rust-Powered**: Built on top of the world-class `toml-rs` parser. Real-time feedback as you type.
- **Accurate**: Detects syntax errors, unclosed strings, and malformed tables with precise line and column information.
- **Lightweight**: Low memory footprint and minimal impact on VS Code performance.

## Installation

1. Open **VS Code**.
2. Go to **Extensions** (`Ctrl+Shift+X`).
3. Search for `tomlkit`.
4. Click **Install**.

## License

MIT © [nachinsec](https://github.com/nachinsec)

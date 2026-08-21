# CodeToMd

A fast, lightweight, and portable native Windows desktop application built in Rust. It bundles your source code files into a single, well-structured Markdown file for use as AI context (LLMs).

## Features

- **Portable & Standalone**: No installation required.
- **Fast & Native**: Built with Rust and `egui`.
- **Smart Formatting**: Preserves folder structure and uses dynamic code fences with correct language mapping.

## How to use

1. Open a project folder (or drag and drop it into the app).
2. Right-click or use the button to add files to the export list.
3. Choose an output `.md` file path.
4. Click **Generate** (to replace) or **Append** (to add to an existing file).

## Building from source

```bash
cargo build --release
```

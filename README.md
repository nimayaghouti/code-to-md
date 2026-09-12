# CodeToMd

A fast, lightweight, and portable native Windows desktop application built in Rust. It bundles your source code files into a single, well-structured Markdown file for use as AI context (LLMs).

## Features

- **Portable & Fast**: No installation required. Fully native UI built with Rust and `egui`.
- **Live Sync & Git Integration**: Automatically detects external file changes without locking files. View Git status badges and export only `Diff` patches to drastically save LLM tokens.
- **Smart Workflow**: Supports `.gitignore`, real-time search, bulk-selection via checkboxes, and syntax-highlighted code previews.
- **Smart Formatting**: Preserves folder structure and uses dynamic code fences with correct language mapping.

## How to use

1. Open a project folder (or drag and drop it into the app).
2. Search, filter, and add files to your export list.
3. _(Optional)_ Toggle `[Diff]` mode for modified files to export only the changes.
4. Click **Save** or **Append** and Choose an output `.md` file path.

## Building from source

```bash
cargo build --release
```

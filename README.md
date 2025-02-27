# Renk 🎨

[![License](https://img.shields.io/badge/license-MIT-blue?style=flat-square)](LICENSE)

A tool for grabbing color palettes from various sources and converting them into different formats.

<p align=center>🚧 Under Construction 🚧</p>

_The main goal of this project is to dive into Rust and GitHub Copilot Edits. Note that the tool may not be of incredible use, at least yet._

## Features ✨

- Download color palettes from various sources.
- Convert color palettes to different formats.
- Export color palettes to standard output.
- Export color palettes to specific applications _(⚠️ Unstable)_

## Installation 🛠️

To install the project, you need to have Rust installed. You can install Rust from [rust-lang.org](https://www.rust-lang.org/).

Clone the repository and build the project:

```bash
git clone https://github.com/TehoorMarjan/renk.git
cd renk
cargo build --release
install -Dm755 "target/release/renk" "/usr/bin/renk"
```

## Usage 🚀

You can use the `renk` command-line tool to list sources, list destinations, and get palettes.

### List Sources

```bash
renk list sources
```

### List Destinations

```bash
renk list destinations
```

### Get Palette

```bash
renk get <destination> <source>
```

Example:

```bash
renk get gpl nord
```

## Contributing 🤝

Contributions are welcome! Please open an issue or submit a pull request on GitHub.

For local development, you may create a `.env` file:

```ini
CONFIG_URL=http://127.0.0.1:8000/config/renk.json
```

and use `python -m http.server` to serve a possibly locally modified file instead of the official one.

## Troubleshooting 🤯

The tool has only been tested on Linux. If you run into issues on Windows or Mac, please report them on GitHub.

## Acknowledgements 🙏

This project uses the following crates:

- [`palette`](https://crates.io/crates/palette)
- [`clap`](https://crates.io/crates/clap)
- [`regex`](https://crates.io/crates/regex)
- [`dirs`](https://crates.io/crates/dirs)

## License 📄

This project is licensed under the MIT License.

## Contact Information 📧

If you have any questions or feedback, please open an issue on GitHub.

GitHub: [TehoorMarjan](https://github.com/TehoorMarjan)

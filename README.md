
# Ziplyn

Ziplyn is a fast and lightweight file compression and extraction tool built in Rust. It supports various compression formats, offering high performance and a simple command-line interface.

## Features

- **High Performance**: Built with Rust, Ziplyn offers high performance and low memory usage.
- **Multiple Compression Formats**: Supports gzip, zip, and more.
- **Easy to Use**: Simple command-line interface for quick compression and extraction.
- **Cross-Platform**: Works on Windows, macOS, and Linux.

## Installation

To install Ziplyn, you can download the pre-built binaries from the [releases page](https://github.com/Vaibhavee89/ziplyn/releases).

### Building from Source

1. Ensure you have Rust installed. If not, you can install it from [rustup.rs](https://rustup.rs/).
2. Clone the repository:
   ```sh
   git clone https://github.com/Vaibhavee89/ziplyn.git
   cd ziplyn
   ```
3. Build the project:
   ```sh
   cargo build --release
   ```
4. The binary will be located in the `target/release` directory.

## Usage

### Compressing Files

To compress a file:
```sh
ziplyn compress <source> <destination>
```

**Example**:
```sh
ziplyn compress file.txt file.txt.gz
```

### Decompressing Files

To decompress a file:
```sh
ziplyn decompress <source> <destination>
```

**Example**:
```sh
ziplyn decompress file.txt.gz file.txt
```

## Why Ziplyn?

Ziplyn stands out from other compression tools due to its:

- High performance, leveraging Rust's efficiency.
- Support for multiple compression formats.
- Lightweight design and low memory usage.
- Easy-to-use, intuitive command-line interface.
- Cross-platform compatibility.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Contributing

Contributions are welcome! If you'd like to contribute, please fork the repository and create a pull request.

## Issues

If you encounter any issues or have feature requests, please open an issue in the [GitHub Issues](https://github.com/Vaibhavee89/ziplyn/issues) section.

## Acknowledgments

Special thanks to the Rust community for their support and guidance.
```
flate2
zip
```
For more information, visit the documentation.








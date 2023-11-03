# Package a Python Script into a Command-Line Tool

## User Guide for Rust-ETL-Demo

### Installation

Before you can use this package, you need to have Rust installed on your machine. If you haven't installed Rust yet, you can do so by following the instructions on the [official Rust website](https://www.rust-lang.org/tools/install).

Once you have Rust installed, you can install this package by adding it to your project's `Cargo.toml` file.

```toml
[dependencies]
csv = "1.3.0"
libc = "0.2.149"
procfs = "0.16.0"
rand = "0.8.5"
reqwest = { version = "0.11.22", features = ["blocking"] }
rusqlite = "0.29.0"
sysinfo = "0.29.10"
```

Then, run the following command in your terminal to download and compile the package:

```bash
cargo build
```

### Usage

Here's a basic example of how to use this package:

```rust
// Import the package
use rust_etl;

fn main() {
    // Use the package
    rust_etl::your_function();
}
```

Replace `your_function` with the actual function you want to call from your package.
`extract`, `transform_load`, or `query`


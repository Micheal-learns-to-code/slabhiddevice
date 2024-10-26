# slabhiddevice

`slabhiddevice` is a Rust library designed to support USB to UART bridge controllers. This library offers a set of functionalities to efficiently interact with and manage USB HID devices.

The library is based on the API specification detailed in the [HID Library API Specification](https://www.silabs.com/documents/public/application-notes/AN532.pdf).

## Installation

Add the following to your `Cargo.toml`:

```toml
[dependencies]
slabhiddevice = "0.1.0"
```

## Usage

Import the `slabhiddevice`. The starting point for all the functionality is to create a HidDevice object. With a HidDevice object, you can get the number of Hid devices connected to the system, open the devices, retrieve more information from them.

```rust
use slabhiddevice::hiddevice;

fn main() {
    // Initialize the slabhiddevice library

    let hiddev = hiddevice::HidDevice::new().unwrap();
    // Perform operations with the devices
    // ...
}
```
For more usage examples, please take a look at the `examples/` directory 
## Documentation

For detailed documentation, please visit [docs.rs/slabhiddevice](https://docs.rs/slabhiddevice).

## Contributing

Contributions are welcome! Please submit a pull request or open an issue to discuss your ideas.
If you have a feature request or find a bug, please follow these steps:

### Requesting a Feature

1. **Open an Issue**: Go to the Issues section of our GitHub repository and open a new issue.
2. **Describe the Feature**: Provide a detailed description of the feature you would like to see, including any relevant use cases and examples.
3. **Label the Issue**: Add the `enhancement` label to your issue.

### Reporting a Bug

1. **Open an Issue**: Go to the Issues section of our GitHub repository and open a new issue.
2. **Describe the Bug**: Provide a detailed description of the bug, including steps to reproduce it, the expected behavior, and the actual behavior.
3. **Provide Context**: Include any relevant information such as your operating system, Rust version, and any error messages.
4. **Label the Issue**: Add the `bug` label to your issue.

### Submitting a Pull Request

1. **Fork the Repository**: Fork the repository to your own GitHub account.
2. **Create a Branch**: Create a new branch for your feature or bug fix.
3. **Make Your Changes**: Implement your feature or bug fix.
4. **Test Your Changes**: Ensure that your changes do not break existing functionality.
5. **Submit a Pull Request**: Open a pull request to merge your changes into the main repository. Provide a detailed description of your changes and reference any related issues.

We appreciate your contributions and look forward to collaborating with you!

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
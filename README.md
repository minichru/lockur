# lockur

**lockur** is a simple command-line interface (CLI) tool designed to make file encryption and decryption a breeze. With a focus on ease of use and security, lockur offers a straightforward way to secure your files with strong encryption standards.

## Features

- **Easy-to-use**: Simple commands to encrypt and decrypt files directly from your terminal.
- **Secure**: Uses AES-256 GCM for encryption, ensuring your files are securely encrypted.
- **Customizable**: Allows setting a custom salt for key derivation, enhancing the security of your encrypted files.

## Installation

Before installing `lockur`, ensure you have Rust's toolchain installed on your system. If not, you can install it from [here](https://rustup.rs/).

Once Rust is installed, you can install `lockur` directly from crates.io by running:

```bash
cargo install lockur
```

## Usage

### Encrypting a File

To encrypt a file, use the `encrypt` command with the `--password` flag to specify your encryption password. The encrypted file will be saved with the same name as the original file but with an `.enc` extension.

```bash
lockur encrypt --password yourpassword input.txt 
```

### Decrypting a File

To decrypt a file, use the `decrypt` command with the `--password` flag to specify the decryption password. The decrypted file will overwrite the original encrypted file.

```bash
lockur decrypt --password yourpassword input.txt.enc 
```

### Setting a Custom Salt

You can set a custom salt for the key derivation process using the `set` command. This is an optional step but can enhance the security of your encrypted files.

```bash
lockur set --salt yoursalt
```

## Contributing

Contributions are welcome! Please feel free to submit pull requests, open issues, or suggest improvements to the project.

## License

`lockur` is licensed under the MIT License. See the LICENSE file for more details.

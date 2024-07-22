# 🔐 PASETO CLI

Welcome to PASETO CLI - Your Swiss Army Knife for PASETO v4.local Tokens!

![PASETO CLI Logo](https://via.placeholder.com/150x150.png?text=PASETO+CLI)

[![Build Status](https://img.shields.io/travis/Govcraft/paseto-cli/master.svg?style=flat-square)](https://travis-ci.org/Govcraft/paseto-cli)
[![License](https://img.shields.io/badge/license-GPL--3.0-blue.svg?style=flat-square)](https://github.com/GovCraft/paseto_cli/blob/main/LICENSE)
[![Crates.io](https://img.shields.io/crates/v/paseto_cli.svg?style=flat-square)](https://crates.io/crates/paseto_cli)

## 🚀 Quick Start

### Installation

```bash
cargo install paseto_cli
```

### Generate a Token

```bash
echo "your-32-byte-key-in-base64" | paseto_cli generate --subject "user123" --expiration "2h"
```

### Validate a Token

```bash
echo "your-32-byte-key-in-base64" | paseto_cli validate --token "v4.local.your-token-here" --subject "user123"
```

## 🎭 What is PASETO CLI?

PASETO CLI is a powerful command-line tool that simplifies working with PASETO (Platform-Agnostic SEcurity TOkens) v4.local tokens. Whether you're a developer, system administrator, or security enthusiast, PASETO CLI has got you covered!

### ✨ Key Features

- 🛠 Generate PASETO v4.local tokens with custom claims
- ✅ Validate existing tokens and verify their claims
- 🎨 Multiple output formats: plain, pretty, and JSON
- ⏱ Support for relative time expressions (e.g., "2h", "1d")
- 🔒 Secure by default, adhering to PASETO best practices

## 📚 Table of Contents

- [Installation](#-installation)
- [Usage](#-usage)
- [Examples](#-examples)
- [Use Cases](#-use-cases)
- [Security Considerations](#-security-considerations)
- [Contributing](#-contributing)
- [License](#-license)

## 📥 Installation

### Using Cargo

```bash
cargo install paseto_cli
```

### From Source

```bash
git clone https://github.com/Govcraft/paseto-cli.git
cd paseto-cli
cargo build --release
```

The binary will be available at `target/release/paseto_cli`.

## 🛠 Usage

PASETO CLI provides two main commands: `generate` and `validate`.

### Generate a Token

```bash
echo "your-32-byte-key-in-base64" | paseto_cli generate [OPTIONS]
```

Options:
- `-s, --subject <SUBJECT>`: Set the subject claim
- `-i, --issuer <ISSUER>`: Set the issuer claim
- `-a, --audience <AUDIENCE>`: Set the audience claim
- `--jti <JTI>`: Set the token identifier claim
- `--expiration <EXPIRATION>`: Set the expiration time
- `--not-before <NOT_BEFORE>`: Set the not-before time
- `--issued-at <ISSUED_AT>`: Set the issued-at time
- `-c, --custom <KEY=VALUE>`: Add custom claims

### Validate a Token

```bash
echo "your-32-byte-key-in-base64" | paseto_cli validate --token <TOKEN> [OPTIONS]
```

Options:
- `-t, --token <TOKEN>`: The PASETO token to validate (required)
- `-s, --subject <SUBJECT>`: Expected subject claim
- `-i, --issuer <ISSUER>`: Expected issuer claim
- `-a, --audience <AUDIENCE>`: Expected audience claim
- `--jti <JTI>`: Expected token identifier claim
- `--expiration <EXPIRATION>`: Expected expiration time
- `--not-before <NOT_BEFORE>`: Expected not-before time
- `--issued-at <ISSUED_AT>`: Expected issued-at time
- `-c, --custom <KEY=VALUE>`: Expected custom claims

### Global Options

- `-f, --format <FORMAT>`: Set the output format (plain, pretty, json)
- `-h, --help`: Print help information
- `-V, --version`: Print version information

## 🌟 Examples

### Generate a Token with Custom Claims

```bash
echo "your-32-byte-key-in-base64" | paseto_cli generate --subject "user123" --expiration "2h" --custom role=admin
```

### Validate a Token with Pretty Output

```bash
echo "your-32-byte-key-in-base64" | paseto_cli --format pretty validate --token "v4.local.your-token-here" --subject "user123"
```

### Generate a Token with JSON Output

```bash
echo "your-32-byte-key-in-base64" | paseto_cli --format json generate --subject "user123" --expiration "2h"
```

## 🎯 Use Cases

1. 🔐 **User Authentication**: Generate tokens for secure user login systems.
2. 🚦 **API Authorization**: Include role-based access control in tokens.
3. 🔀 **Single Sign-On (SSO)**: Create tokens for seamless multi-service authentication.
4. 🔑 **API Key Management**: Generate and validate long-lived API keys.
5. 💻 **Session Management**: Create short-lived tokens for web application sessions.
6. 🌐 **Microservices Communication**: Secure inter-service data exchange.
7. 🏠 **IoT Device Authentication**: Authenticate IoT devices with central servers.
8. 📝 **Audit Logging**: Track user actions with custom claims for comprehensive logs.
9. 🧪 **Testing and Development**: Generate specific tokens for auth flow testing.
10. 🔄 **Token Rotation**: Implement secure token rotation strategies.

## 🛡 Security Considerations

1. 🔐 **Key Management**: Keep your PASETO v4 key secure and confidential.
2. ⏳ **Token Lifetime**: Use appropriate expiration times based on security needs.
3. ✅ **Claim Validation**: Always validate all relevant claims when verifying tokens.
4. 🔒 Encrypted Payload: Remember that while v4.local tokens encrypt their payload, protecting the confidentiality of claims, the encryption key must still be kept secure.
5. 🔄 **Key Rotation**: Implement a strategy to periodically update your PASETO v4 key.

## 🤝 Contributing

We welcome contributions! Please see our [Contributing Guidelines](CONTRIBUTING.md) for more details.

## 📄 License

PASETO CLI is licensed under the GNU General Public License v3.0. See the [LICENSE](LICENSE) file for details.

---

Built with ❤️ by [Govcraft](https://www.github.com/Govcraft). For issues, feature requests, or questions, please [open an issue](https://github.com/Govcraft/paseto-cli/issues).

Happy token generating and validating! 🎉

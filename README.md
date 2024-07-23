# PASETO CLI

PASETO CLI generates and validates PASETO v4.local tokens.

[![Build Status](https://img.shields.io/travis/Govcraft/paseto-cli/master.svg?style=flat-square)](https://travis-ci.org/Govcraft/paseto-cli)
[![License](https://img.shields.io/badge/license-GPL--3.0-blue.svg?style=flat-square)](https://github.com/GovCraft/paseto_cli/blob/main/LICENSE)
[![Crates.io](https://img.shields.io/crates/v/paseto_cli.svg?style=flat-square)](https://crates.io/crates/paseto_cli)

## Quick Start

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

## Features

- Generates PASETO v4.local tokens with custom claims
- Validates existing tokens and verifies their claims
- Supports multiple output formats: plain, pretty, and JSON
- Handles relative time expressions (e.g., "2h", "1d")
- Adheres to PASETO best practices

## Table of Contents

- [Installation](#installation)
- [Usage](#usage)
- [Examples](#examples)
- [Use Cases](#use-cases)
- [Security Considerations](#security-considerations)
- [Contributing](#contributing)
- [License](#license)
- [About the Author](#about-the-author)

## Installation

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

The binary is located at `target/release/paseto_cli`.

## Usage

PASETO CLI provides two main commands: `generate` and `validate`.

### Generate a Token

```bash
echo "your-32-byte-key-in-base64" | paseto_cli generate [OPTIONS]
```

Options:
- `-s, --subject <SUBJECT>`: Sets the subject claim
- `-i, --issuer <ISSUER>`: Sets the issuer claim
- `-a, --audience <AUDIENCE>`: Sets the audience claim
- `--jti <JTI>`: Sets the token identifier claim
- `--expiration <EXPIRATION>`: Sets the expiration time
- `--not-before <NOT_BEFORE>`: Sets the not-before time
- `--issued-at <ISSUED_AT>`: Sets the issued-at time
- `-c, --custom <KEY=VALUE>`: Adds custom claims

### Validate a Token

```bash
echo "your-32-byte-key-in-base64" | paseto_cli validate --token <TOKEN> [OPTIONS]
```

Options:
- `-t, --token <TOKEN>`: Specifies the PASETO token to validate (required)
- `-s, --subject <SUBJECT>`: Specifies the expected subject claim
- `-i, --issuer <ISSUER>`: Specifies the expected issuer claim
- `-a, --audience <AUDIENCE>`: Specifies the expected audience claim
- `--jti <JTI>`: Specifies the expected token identifier claim
- `--expiration <EXPIRATION>`: Specifies the expected expiration time
- `--not-before <NOT_BEFORE>`: Specifies the expected not-before time
- `--issued-at <ISSUED_AT>`: Specifies the expected issued-at time
- `-c, --custom <KEY=VALUE>`: Specifies expected custom claims

### Global Options

- `-f, --format <FORMAT>`: Sets the output format (plain, pretty, json)
- `-h, --help`: Prints help information
- `-V, --version`: Prints version information

## Examples

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

## Use Cases

1. User Authentication: Generates tokens for user login systems.
2. API Authorization: Includes role-based access control in tokens.
3. Single Sign-On (SSO): Creates tokens for multi-service authentication.
4. API Key Management: Generates and validates long-lived API keys.
5. Session Management: Creates short-lived tokens for web application sessions.
6. Microservices Communication: Secures inter-service data exchange.
7. IoT Device Authentication: Authenticates IoT devices with central servers.
8. Audit Logging: Tracks user actions with custom claims for logs.
9. Testing and Development: Generates specific tokens for auth flow testing.
10. Token Rotation: Implements token rotation strategies.

## Security Considerations

1. Key Management: Store the PASETO v4 key securely.
2. Token Lifetime: Set appropriate expiration times based on security requirements.
3. Claim Validation: Validate all relevant claims when verifying tokens.
4. Encrypted Payload: v4.local tokens encrypt their payload, protecting the confidentiality of claims. The encryption key must be kept secure.
5. Key Rotation: Implement a strategy to periodically update the PASETO v4 key.

## Contributing

Contributions are welcome. See [Contributing Guidelines](CONTRIBUTING.md) for more details.

## License

PASETO CLI is licensed under the GNU General Public License v3.0. See the [LICENSE](LICENSE) file for details.

## About the Author

I'm @rrrodzilla, a technologist with 30 years of industry experience. I'm a former SOA and cloud architect, and former Principal Technical Product Manager at AWS for the Rust Programming Language. Currently, I'm the owner and operator of Govcraft, building and consulting on Rust and AI solutions.

For more information, visit https://www.govcraft.ai

---

For issues, feature requests, or questions, open an issue at https://github.com/Govcraft/paseto-cli/issues.

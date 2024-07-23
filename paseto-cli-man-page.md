# PASETO_CLI(1) User Commands

## NAME
paseto_cli - A command-line tool for generating and validating PASETO v4.local tokens

## SYNOPSIS
`paseto_cli [OPTIONS] <COMMAND>`

## DESCRIPTION
PASETO_CLI is a powerful and flexible command-line interface for working with Platform-Agnostic SEcurity TOkens (PASETO) v4.local tokens. It provides functionality for both generating new tokens and validating existing ones, making it an essential tool for developers and system administrators working with secure authentication and authorization systems.

## OPTIONS
`-f, --format <FORMAT>`
    Specify the output format for the results. Available options are:
    - `plain` (default): Simple text output
    - `pretty`: Colorized and formatted output for better readability
    - `json`: JSON-formatted output for easy parsing and integration with other tools

`-h, --help`
    Print help information

`-V, --version`
    Print version information

## COMMANDS
### generate
Generate a new PASETO token using a v4 key provided via stdin.

Usage: `cat V4_key_file | paseto_cli generate [OPTIONS]`
       `echo wubbalubbadubdubwubbalubbadubdub | paseto_cli generate [OPTIONS]`

Options:
* `-s, --subject <SUBJECT>`
    Set the subject claim
* `-i, --issuer <ISSUER>`
    Set the issuer claim
* `-a, --audience <AUDIENCE>`
    Set the audience claim
* `--jti <JTI>`
    Set the token identifier claim
* `--expiration <EXPIRATION>`
    Set the expiration time (ISO 8601 or relative time, e.g., '2h', '1d')
* `--not-before <NOT_BEFORE>`
    Set the not-before time (ISO 8601 or relative time)
* `--issued-at <ISSUED_AT>`
    Set the issued-at time (ISO 8601 or relative time)
* `-c, --custom <KEY=VALUE>`
    Add custom claims in the format KEY=VALUE

### validate
Validate an existing PASETO token using a v4 key provided via stdin.

Usage: `cat V4_key_file | paseto_cli validate [OPTIONS] --token <TOKEN>`
       `echo wubbalubbadubdubwubbalubbadubdub | paseto_cli validate [OPTIONS] --token <TOKEN>`

Options:
* `-t, --token <TOKEN>`
    The PASETO token to validate (required)
* `-s, --subject <SUBJECT>`
    Expected subject claim
* `-i, --issuer <ISSUER>`
    Expected issuer claim
* `-a, --audience <AUDIENCE>`
    Expected audience claim
* `--jti <JTI>`
    Expected token identifier claim
* `--expiration <EXPIRATION>`
    Expected expiration time (ISO 8601 or relative time)
* `--not-before <NOT_BEFORE>`
    Expected not-before time (ISO 8601 or relative time)
* `--issued-at <ISSUED_AT>`
    Expected issued-at time (ISO 8601 or relative time)
* `-c, --custom <KEY=VALUE>`
    Expected custom claims in the format KEY=VALUE

## INPUT
PASETO_CLI expects a v4 PASETO key to be provided via stdin for all operations. This key should be a 32-byte key encoded in base64 format.

## OUTPUT
The output format is determined by the `--format` option:
* `plain`: Simple text output
* `pretty`: Colorized and formatted output for better readability
* `json`: JSON-formatted output

## EXIT STATUS
* 0: Successful operation
* 1: Error occurred (with error message printed to stderr)

## EXAMPLES
1. Generate a basic token:
   ```
   echo "your-32-byte-key-in-base64" | paseto_cli generate
   ```

2. Generate a token with custom claims and expiration:
   ```
   echo "your-32-byte-key-in-base64" | paseto_cli generate --subject "user123" --expiration "2h" --custom role=admin
   ```

3. Generate an expired token (for testing purposes):
   ```
   echo "your-32-byte-key-in-base64" | paseto_cli generate --expiration "-2h"
   ```

4. Validate a token:
   ```
   echo "your-32-byte-key-in-base64" | paseto_cli validate --token "v4.local.payload" --subject "user123"
   ```

5. Use pretty output format:
   ```
   echo "your-32-byte-key-in-base64" | paseto_cli --format pretty generate --subject "user123" --expiration "2h"
   ```

6. Use JSON output format:
   ```
   echo "your-32-byte-key-in-base64" | paseto_cli --format json generate --subject "user123" --expiration "2h"
   ```

## USE CASES
1. Authentication Systems: Generate tokens for user authentication in web applications or APIs.
2. Authorization: Include role-based access control information in tokens for fine-grained permissions.
3. Single Sign-On (SSO): Create tokens that can be used across multiple services within an organization.
4. API Key Management: Generate and validate long-lived API keys with custom claims.
5. Session Management: Create short-lived tokens for managing user sessions in web applications.
6. Microservices Communication: Secure inter-service communication using PASETO tokens.
7. IoT Device Authentication: Generate and validate tokens for IoT devices to securely communicate with central servers.
8. Audit Logging: Include custom claims in tokens to track user actions and facilitate audit logging.
9. Testing and Development: Generate tokens with specific claims and expiration times for testing authentication and authorization flows.
10. Token Rotation: Validate existing tokens and generate new ones as part of a token rotation strategy.

## SECURITY CONSIDERATIONS
1. Key Management: Ensure that the PASETO v4 key is kept secure and not exposed in logs or version control systems.
2. Token Lifetime: Use appropriate expiration times for tokens based on their intended use and security requirements.
3. Claim Validation: Always validate all relevant claims when verifying tokens, including expiration, issuer, and audience.
4. Custom Claims: Be cautious when adding sensitive information as custom claims, as the payload of local tokens is encrypted but not signed.
5. Key Rotation: Implement a key rotation strategy to periodically update the PASETO v4 key used for token generation and validation.

## LIMITATIONS
1. PASETO_CLI only supports v4.local tokens. It does not support other PASETO versions or public tokens.
2. The tool does not provide key generation functionality. Users must provide their own properly formatted PASETO v4 keys.
3. PASETO_CLI does not integrate directly with databases or user management systems. It focuses solely on token generation and validation.

## SEE ALSO
* PASETO Specification: https://github.com/paseto-standard/paseto-spec
* rusty_paseto Documentation: https://docs.rs/rusty_paseto/

## AUTHOR
Written by Roland Rodriguez <roland@govcraft.ai>

## REPORTING BUGS
Report bugs to the GitHub issue tracker: https://github.com/yourusername/paseto_cli/issues

## COPYRIGHT
Copyright © 2024 Roland Rodriguez. License GPLv3+: GNU GPL version 3 or later <https://gnu.org/licenses/gpl.html>.
This is free software: you are free to change and redistribute it. There is NO WARRANTY, to the extent permitted by law.

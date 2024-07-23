use std::convert::TryFrom;
use std::io::{self, Read, Write};

use clap::{crate_authors, crate_description, crate_name, crate_version, Parser, Subcommand, ValueEnum};
use rusty_paseto::core::V4;
use rusty_paseto::prelude::*;
use serde_json::Value;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};
use terminal_size::{terminal_size, Width};
use textwrap::{Options, WordSplitter};
use thiserror::Error;
use time::{Duration, OffsetDateTime};

// after_help = "EXAMPLES:
//     Generate a token:
//     echo \"your-32-byte-key-in-base64\" | pasto generate\"
//
//     Generate a token:
//     echo \"your-32-byte-key-in-base64\" | pasto generate --subject \"user123\" --expiration \"2h\"
//
//     Generate an expired token:
//     echo \"your-32-byte-key-in-base64\" | pasto generate --expiration \"-2h\"
//
//     Validate a token:
//     echo \"your-32-byte-key-in-base64\" | pasto validate --token \"v4.local.payload\" --subject \"user123\"
//
//     Use pretty output:
//     echo \"your-32-byte-key-in-base64\" | pasto --format pretty generate --subject \"user123\" --expiration \"2h\"
//
//     Use json output:
//     echo \"your-32-byte-key-in-base64\" | pasto --format json generate --subject \"user123\" --expiration \"2h\"
// "

#[derive(Parser)]
#[command(
    name = crate_name!(),
    version = crate_version!(),
    about = crate_description!(),
    author = crate_authors!("\n"),
    long_about = "Pasto is a CLI tool for generating and validating PASETO (Platform-Agnostic SEcurity TOkens) v4.local tokens. It supports custom claims and various output formats. A v4 PASETO key must be provided via stdin for all operations. For more information, run `man paseto_cli`",
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// Specify the output format for the results
    #[arg(
        short,
        long,
        value_enum,
        default_value_t = OutputFormat::Plain,
        help = "Set the output format"
    )]    format: OutputFormat,
}

#[derive(Clone, ValueEnum)]
enum OutputFormat {
    Plain,
    Pretty,
    Json,
}

#[derive(Subcommand)]
enum Commands {
    /// Generate a new PASETO token
    #[command(
        about = "Generate a new PASETO token using a v4 key provided via stdin",
        override_usage = "cat V4_key_file | paseto_cli.exe generate [OPTIONS]\n       echo wubbalubbadubdubwubbalubbadubdub | paseto_cli.exe generate [OPTIONS]"
    )]
    Generate {
        #[arg(short, long, help = "Set the subject claim")]
        subject: Option<String>,

        #[arg(short, long, help = "Set the issuer claim")]
        issuer: Option<String>,

        #[arg(short, long, help = "Set the audience claim")]
        audience: Option<String>,

        #[arg(long, help = "Set the token identifier claim")]
        jti: Option<String>,

        #[arg(
            long,
            value_name = "EXPIRATION",
            allow_hyphen_values = true,
            help = "Set the expiration time (ISO 8601 or relative time, e.g., '2h', '1d')"
        )]
        expiration: Option<String>,

        #[arg(
            long,
            value_name = "NOT_BEFORE",
            allow_hyphen_values = true,
            help = "Set the not-before time (ISO 8601 or relative time)"
        )]
        not_before: Option<String>,

        #[arg(
            long,
            value_name = "ISSUED_AT",
            allow_hyphen_values = true,
            help = "Set the issued-at time (ISO 8601 or relative time)"
        )]
        issued_at: Option<String>,

        #[arg(
            short,
            long,
            num_args = 0..,
            value_parser = parse_key_val,
            help = "Add custom claims in the format KEY=VALUE"
        )]
        custom: Vec<(String, String)>,
    },

    /// Validate an existing PASETO token
    #[command(
        about = "Validate an existing PASETO token using a v4 key provided via stdin",
        override_usage = "cat V4_key_file | paseto_cli.exe validate [OPTIONS] --token <TOKEN>\n       echo wubbalubbadubdubwubbalubbadubdub | paseto_cli.exe validate [OPTIONS] --token <TOKEN>"
    )]
    Validate {
        #[arg(short, long, required = true, help = "The PASETO token to validate")]
        token: String,

        #[arg(short, long, help = "Expected subject claim")]
        subject: Option<String>,

        #[arg(short, long, help = "Expected issuer claim")]
        issuer: Option<String>,

        #[arg(short, long, help = "Expected audience claim")]
        audience: Option<String>,

        #[arg(long, help = "Expected token identifier claim")]
        jti: Option<String>,

        #[arg(
            long,
            value_name = "EXPIRATION",
            allow_hyphen_values = true,
            help = "Expected expiration time (ISO 8601 or relative time)"
        )]
        expiration: Option<String>,

        #[arg(
            long,
            value_name = "NOT_BEFORE",
            allow_hyphen_values = true,
            help = "Expected not-before time (ISO 8601 or relative time)"
        )]
        not_before: Option<String>,

        #[arg(
            long,
            value_name = "ISSUED_AT",
            allow_hyphen_values = true,
            help = "Expected issued-at time (ISO 8601 or relative time)"
        )]
        issued_at: Option<String>,

        #[arg(
            short,
            long,
            num_args = 0..,
            value_parser = parse_key_val,
            help = "Expected custom claims in the format KEY=VALUE"
        )]
        custom: Vec<(String, String)>,
    },
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    // Read the key from stdin
    let key = read_key_from_stdin()?;

    match &cli.command {
        Commands::Generate { subject, issuer, audience, jti: identifier, expiration, not_before, issued_at, custom } => {
            let claims = PasetoClaims {
                subject,
                issuer,
                audience,
                identifier,
                expiration,
                not_before,
                issued_at,
            };
            let token = generate_token(&key, claims, custom)?;
            output_result(&cli.format, Ok(token))?;
        }
        Commands::Validate { token, subject, issuer, audience, jti: identifier, expiration, not_before, issued_at, custom } => {
            let claims = PasetoClaims {
                subject,
                issuer,
                audience,
                identifier,
                expiration,
                not_before,
                issued_at,
            };
            let result = validate_token(&key, token, claims, custom);
            output_result(&cli.format, result)?;
        }
    }

    Ok(())
}

#[derive(Error, Debug)]
enum CustomError {
    #[error("{0}")]
    ClaimValidationError(#[from] PasetoClaimError),
    #[error("{0}")]
    ParserError(#[from] GenericParserError),
    #[error(transparent)]
    PasetoError(#[from] PasetoError),
    #[error(transparent)]
    AnyhowError(#[from] anyhow::Error),
    #[error(transparent)]
    FormatError(#[from] time::error::Format),
    #[error(transparent)]
    SerdeJsonError(#[from] serde_json::Error),
}
fn output_result(format: &OutputFormat, result: Result<String, CustomError>) -> anyhow::Result<()> {
    match result {
        Ok(output) => {
            match format {
                OutputFormat::Json => {
                    println!("{{\"success\": true, \"output\": {}}}", serde_json::to_string(&output)?);
                }
                OutputFormat::Pretty => {
                    let mut stdout = StandardStream::stdout(ColorChoice::Auto);
                    stdout.set_color(ColorSpec::new().set_fg(Some(Color::Green)))?;
                    // write!(stdout, "> Success!")?;
                    stdout.reset()?;
                    // Check if the output is a PASETO token
                    if output.starts_with("v4.local.") {
                        color_print_token(&mut stdout, &output)?;
                    } else {

                        // If it's not a token, just wrap and print it
                        // Try to parse as JSON, if it fails, treat it as a plain string
                        match serde_json::from_str::<Value>(&output) {
                            Ok(Value::Object(map)) => {
                                // Find the length of the longest key
                                let max_key_len = map.keys().map(|key| key.len()).max().unwrap_or(0);
                                stdout.set_color(ColorSpec::new().set_fg(Some(Color::Rgb(214, 64, 159))))?;
                                writeln!(stdout, "{:>width$}CLAIM VALUES", " ".repeat(max_key_len + 2), width = max_key_len)?;
                                stdout.reset()?;
                                for (key, value) in map {
                                    stdout.set_color(ColorSpec::new().set_fg(Some(Color::Rgb(214, 64, 159))))?;
                                    write!(stdout, "{:>width$}:", key, width = max_key_len)?;
                                    stdout.reset()?;
                                    // Match on the value to print it without quotes if it's a string
                                    match value {
                                        Value::String(s) => writeln!(stdout, " {}", s)?,
                                        _ => writeln!(stdout, " {:?}", value)?,
                                    }
                                }
                            }
                            _ => {
                                let wrapped_output = wrap_text(&output);
                                writeln!(stdout, "{}", wrapped_output)?;
                            }
                        }
                    }
                }
                OutputFormat::Plain => {
                    println!("{}", output);
                }
            }
        }
        Err(e) => {
            let error_message = e.to_string();

            match format {
                OutputFormat::Json => {
                    println!("{{\"success\": false, \"error\": {}}}", serde_json::to_string(&error_message)?);
                }
                OutputFormat::Pretty => {
                    let mut stderr = StandardStream::stderr(ColorChoice::Auto);
                    stderr.set_color(ColorSpec::new().set_fg(Some(Color::Red)))?;
                    write!(stderr, "✗ ")?;
                    stderr.reset()?;
                    writeln!(stderr, "{}", error_message)?;
                }
                OutputFormat::Plain => {
                    eprintln!("Error: {}", error_message);
                }
            }
            std::process::exit(1);
        }
    }
    Ok(())
}
fn color_print_token(stdout: &mut StandardStream, token: &str) -> anyhow::Result<()> {
    let parts: Vec<&str> = token.split('.').collect();
    if parts.len() != 3 {
        return Err(anyhow::anyhow!("Invalid token format"));
    }

    // Print "v4" in blue
    stdout.set_color(ColorSpec::new().set_fg(Some(Color::Green)))?;
    writeln!(stdout, "{:>7}\u{1F512} ENCRYPTED", " ")?;
    stdout.reset()?;
    stdout.set_color(ColorSpec::new().set_fg(Some(Color::Rgb(0, 144, 255))))?;
    write!(stdout, "{}{}.", " ".repeat(7), parts[0])?;

    // // Print first "." in yellow
    // stdout.set_color(ColorSpec::new().set_fg(Some(Color::Yellow)))?;
    // write!(stdout, ".")?;

    // Print "local" in green
    stdout.set_color(ColorSpec::new().set_fg(Some(Color::Rgb(0, 144, 255))))?;
    write!(stdout, "{}.", parts[1])?;

    // // Print second "." in yellow
    // stdout.set_color(ColorSpec::new().set_fg(Some(Color::Yellow)))?;
    // write!(stdout, ".")?;

    // Print payload in magenta, with wrapping
    stdout.set_color(ColorSpec::new().set_fg(Some(Color::White)))?;
    let wrapped_payload = wrap_text(parts[2]);
    writeln!(stdout, "{}", wrapped_payload)?;

    stdout.reset()?;
    Ok(())
}
fn read_key_from_stdin() -> anyhow::Result<String> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;
    Ok(buffer.trim().to_string())
}

struct PasetoClaims<'a> {
    subject: &'a Option<String>,
    issuer: &'a Option<String>,
    audience: &'a Option<String>,
    identifier: &'a Option<String>,
    expiration: &'a Option<String>,
    not_before: &'a Option<String>,
    issued_at: &'a Option<String>,
}
fn generate_token(
    key: &str,
    claims: PasetoClaims,
    custom: &[(String, String)],
) -> anyhow::Result<String> {
    let symmetric_key = PasetoSymmetricKey::<V4, Local>::from(Key::from(key.as_bytes()));

    let mut builder = PasetoBuilder::<V4, Local>::default();

    if let Some(s) = claims.subject {
        builder.set_claim(SubjectClaim::from(s.as_str()));
    }
    if let Some(i) = claims.issuer {
        builder.set_claim(IssuerClaim::from(i.as_str()));
    }
    if let Some(a) = claims.audience {
        builder.set_claim(AudienceClaim::from(a.as_str()));
    }
    if let Some(id) = claims.identifier {
        builder.set_claim(TokenIdentifierClaim::from(id.as_str()));
    }

    if let Some(exp) = claims.expiration {
        let exp_time = parse_expiration(exp)
            .map_err(|e| anyhow::anyhow!("Invalid expiration format. Use ISO 8601 (e.g., '2024-07-23T00:20:32Z') or relative time (e.g., '5m', '-1h', '2d'): {}", e))?;
        builder.set_claim(ExpirationClaim::try_from(exp_time.format(&time::format_description::well_known::Rfc3339)?)?);
    }

    if let Some(nb) = claims.not_before {
        let not_before_time = parse_expiration(nb)
            .map_err(|e| anyhow::anyhow!("Invalid not_before format. Use ISO 8601 (e.g., '2024-07-23T00:20:32Z') or relative time (e.g., '5m', '-1h', '2d'): {}", e))?;
        builder.set_claim(NotBeforeClaim::try_from(not_before_time.format(&time::format_description::well_known::Rfc3339)?)?);
    }

    if let Some(ia) = claims.issued_at {
        let issue_time = parse_expiration(ia)
            .map_err(|e| anyhow::anyhow!("Invalid issued_at format. Use ISO 8601 (e.g., '2024-07-23T00:20:32Z') or relative time (e.g., '5m', '-1h', '2d'): {}", e))?;
        builder.set_claim(IssuedAtClaim::try_from(issue_time.format(&time::format_description::well_known::Rfc3339)?)?);
    }

    for (key, value) in custom {
        builder.set_claim(CustomClaim::try_from((key.as_str(), value.as_str()))?);
    }

    let token = builder.build(&symmetric_key)?;

    Ok(token)
}

fn validate_token(
    key: &str,
    token: &str,
    claims: PasetoClaims,
    custom: &[(String, String)],
) -> Result<String, CustomError> {
    let symmetric_key = PasetoSymmetricKey::<V4, Local>::from(Key::from(key.as_bytes()));

    let mut parser = PasetoParser::<V4, Local>::default();
    if let Some(exp) = claims.expiration {
        let exp_time = parse_expiration(exp)
            .map_err(|e| anyhow::anyhow!("Invalid expiration format. Use ISO 8601 (e.g., '2024-07-23T00:20:32Z') or relative time (e.g., '5m', '-1h', '2d'): {}", e))?;
        let exp_str = exp_time.format(&time::format_description::well_known::Rfc3339)?;
        let exp_str: &'static str = Box::leak(exp_str.into_boxed_str());
        parser.check_claim(ExpirationClaim::try_from(exp_str)?);
    }
    if let Some(nb) = claims.not_before {
        let not_before_time = parse_expiration(nb)
            .map_err(|e| anyhow::anyhow!("Invalid not_before format. Use ISO 8601 (e.g., '2024-07-23T00:20:32Z') or relative time (e.g., '5m', '-1h', '2d'): {}", e))?;
        let not_before_str = not_before_time.format(&time::format_description::well_known::Rfc3339)?;
        let not_before_str: &'static str = Box::leak(not_before_str.into_boxed_str());
        parser.check_claim(NotBeforeClaim::try_from(not_before_str)?);
    }
    if let Some(ia) = claims.issued_at {
        let issued_time = parse_expiration(ia)
            .map_err(|e| anyhow::anyhow!("Invalid issued_at format. Use ISO 8601 (e.g., '2024-07-23T00:20:32Z') or relative time (e.g., '5m', '-1h', '2d'): {}", e))?;
        let issued_str = issued_time.format(&time::format_description::well_known::Rfc3339)?;
        let issued_str: &'static str = Box::leak(issued_str.into_boxed_str());
        parser.check_claim(IssuedAtClaim::try_from(issued_str)?);
    }
    if let Some(s) = claims.subject {
        let s: &'static str = Box::leak(s.to_string().into_boxed_str());
        parser.check_claim(SubjectClaim::from(s));
    }
    if let Some(i) = claims.issuer {
        let i: &'static str = Box::leak(i.to_string().into_boxed_str());
        parser.check_claim(IssuerClaim::from(i));
    }
    if let Some(a) = claims.audience {
        let a: &'static str = Box::leak(a.to_string().into_boxed_str());
        parser.check_claim(AudienceClaim::from(a));
    }
    if let Some(id) = claims.identifier {
        let a: &'static str = Box::leak(id.to_string().into_boxed_str());
        parser.check_claim(TokenIdentifierClaim::from(a));
    }

    for (key, value) in custom {
        let value: &'static str = Box::leak(value.to_string().into_boxed_str());
        parser.check_claim(CustomClaim::try_from((key.as_str(), value))?);
    }

    let parsed_token: Value = parser.parse(token, &symmetric_key)?;

    Ok(serde_json::to_string_pretty(&parsed_token)?)
}


fn parse_key_val(s: &str) -> Result<(String, String), String> {
    // Check if the string is empty
    if s.is_empty() {
        return Err("Empty input: expected format KEY=value".to_string());
    }

    // Find the position of the first '=' character
    match s.find('=') {
        Some(pos) => {
            let key = s[..pos].trim();
            let value = s[pos + 1..].trim();

            // Check if the key is empty
            if key.is_empty() {
                Err("Missing key: expected format KEY=value".to_string())
            }
            // Check if the value is empty
            else if value.is_empty() {
                Err(format!("Missing value for key '{}': expected format KEY=value", key))
            } else {
                Ok((key.to_string(), value.to_string()))
            }
        }
        None => Err(format!("Invalid format '{}': expected KEY=value", s)),
    }
}
fn parse_expiration(exp: &str) -> anyhow::Result<OffsetDateTime> {
    if exp.ends_with('s') || exp.ends_with('m') || exp.ends_with('h') || exp.ends_with('d') {
        let now = OffsetDateTime::now_utc();
        let duration = parse_duration(exp)?;
        Ok(now + duration)
    } else {
        Ok(OffsetDateTime::parse(exp, &time::format_description::well_known::Rfc3339)?)
    }
}

fn parse_duration(duration: &str) -> anyhow::Result<Duration> {
    let (sign, duration) = if let Some(stripped) = duration.strip_prefix('-') {
        (-1, stripped)
    } else {
        (1, duration)
    };

    let (amount, unit) = duration.split_at(duration.len() - 1);
    let amount: i64 = amount.parse()?;
    let duration = match unit {
        "s" => Duration::seconds(amount),
        "m" => Duration::minutes(amount),
        "h" => Duration::hours(amount),
        "d" => Duration::days(amount),
        _ => return Err(anyhow::anyhow!("Invalid duration unit")),
    };

    Ok(duration * sign)
}

fn wrap_text(text: &str) -> String {
    let width = terminal_size().map(|(Width(w), _)| w as usize).unwrap_or(80);
    let available_width = width.saturating_sub(1); // 4 spaces for indentation
    textwrap::fill(text, Options::new(available_width).break_words(true).word_splitter(WordSplitter::NoHyphenation).subsequent_indent(" ".repeat(16).as_str()))
}

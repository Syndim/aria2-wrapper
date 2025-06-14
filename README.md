# Downloader Wrapper

A simple wrapper for aria2c/curl that allows URL replacement based on configured patterns.


## Features

- Intercepts and modifies URLs passed to aria2c
- Supports URL replacement via regex patterns
- Works with direct URL parameters and input files
- Passes all other arguments unchanged to aria2c

## Installation

```bash
cargo install --path .
```

## Usage

```bash
# Using direct URLs
aria2-wrapper https://example.com/file.zip

# Using an input file
aria2-wrapper -i urls.txt

# With custom config
aria2-wrapper --config my-config.toml -i urls.txt

# Passing additional aria2c parameters
aria2-wrapper --config my-config.toml -x 16 -j 4 https://example.com/file.zip
```

## Configuration

Create a `config.toml` file with URL replacement rules:

```toml
# Downloader Wrapper Configuration File

# Path to the aria2c executable (defaults to "aria2c" if not specified)
aria2c_path = "aria2c"
curl_path = "curl"

# URL replacements - patterns are regular expressions
[[replacements]]
pattern = "^https://example.com/"
replacement = "https://mirror.example.com/"

[[replacements]]
pattern = "^https://slow-cdn.com/files/"
replacement = "https://fast-cdn.com/mirror/"

[[replacements]]
pattern = "^magnet:.*dn=([^&]+).*"
replacement = "magnet:?xt=urn:btih:$1"
```

The patterns are regular expressions that will be applied to each URL before passing to aria2c.

## Development

### Building

```bash
cargo build --release
```

### Testing

```bash
cargo test
```

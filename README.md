# RequestCLI : Rust API Testing CLI

## Overview

This Rust CLI program allows users to perform API testing by sending HTTP requests with customizable parameters. It supports various HTTP methods and allows for JSON bodies, headers, and URL parameters.

## Features

- Support for multiple HTTP methods: GET, POST, PUT, DELETE, PATCH
- Ability to send JSON bodies in requests
- Customizable headers for requests
- Support for URL query parameters
- Easy command-line interface for quick API testing

## Prerequisites

Make sure you have the following installed:

- [Rust](https://www.rust-lang.org/tools/install)
- [Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html)

## Installation

Clone the repository:

```bash
git clone [https://github.com/akashchekodu/RequestCLI.git](https://github.com/akashchekodu/RequestCLI)
cd rust-api-testing-cli
```

Build the project:

```bash
cargo build --release
```

## Usage

You can run the CLI by executing the following command:

```bash
./target/release/rust-api-testing-cli -m <HTTP_METHOD> -u <URL> [OPTIONS]
```

### Arguments

- `-m`, `--method`: The HTTP method to use (e.g., `GET`, `POST`, `PUT`, `DELETE`, `PATCH`).
- `-u`, `--url`: The URL for the API request.
- `-b`, `--body`: JSON-like string with request parameters. Optional for `GET` requests.
- `-p`, `--url-params`: URL parameters to be included in the request as a JSON-like string.
- `-a`, `--headers`: Custom headers for the request, specified as a string in the format `key1:value1;key2:value2`.

### Example Usage

#### Sending a GET Request

```bash
./target/release/rust-api-testing-cli -m GET -u "https://api.example.com/data"
```

#### Sending a POST Request with JSON Body

```bash
./target/release/rust-api-testing-cli -m POST -u "https://api.example.com/data" -b '{"field1": "value1", "field2": "value2"}'
```

#### Sending a Request with Custom Headers

```bash
./target/release/rust-api-testing-cli -m POST -u "https://api.example.com/data" -b '{"field1": "value1"}' -a "Authorization: Bearer your_token; Content-Type: application/json"
```

#### Sending URL Parameters

```bash
./target/release/rust-api-testing-cli -m GET -u "https://api.example.com/data" -p '{"param1": "value1", "param2": "value2"}'
```

## Error Handling

The program will output error messages in case of:

- Invalid JSON format in the request body or headers.
- Unsupported HTTP methods.
- Failed API calls.

## Contribution

Contributions are welcome! If you have suggestions for improvements or new features, please open an issue or submit a pull request.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

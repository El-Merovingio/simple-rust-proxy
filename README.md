# HTTP Proxy with Warp and Reqwest

This project demonstrates a simple HTTP proxy server built using Rust's `warp` and `reqwest` libraries. The proxy forwards incoming requests to a specified base URL, which must start with `http://`.

## Features

- **HTTP Proxy:** Forwards all incoming requests to a base URL specified at runtime.
- **URL Validation:** Ensures the provided base URL starts with `http://`.
- **Colored Terminal Messages:** Uses `owo-colors` to display informative and error messages with colors.

## Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (latest stable version)

## Installation

1. Clone the repository:
    ```sh
    git clone https://github.com/El-Merovingio/simple-rust-proxy.git
    cd simple-rust-proxy
    ```

## Usage

Run the proxy server with the base URL as an argument, for instance:
```sh
cargo run http://localhost:1234
```

Browse to `http://<Your_IP>:3030` 

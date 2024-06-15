# Currency Converter CLI

A simple CLI tool to convert GBP to CNY using real-time exchange rates.

## Features

- Converts British Pounds (GBP) to Chinese Yuan (CNY).
- Fetches real-time exchange rates from an API.

## Prerequisites

- [Rust](https://www.rust-lang.org/tools/install)
- [Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html)

## Installation

1. Clone the repository:

    ```sh
    git clone https://github.com/yourusername/currency_converter.git
    cd currency_converter
    ```

2. Create a `.env` file in the project root directory and add your API key:

    ```sh
    echo "EXCHANGE_RATE_API_KEY=your_api_key_here" > .env
    ```

3. Build the project:

    ```sh
    cargo build
    ```

## Usage

To convert an amount from GBP to CNY, run the following command:

```sh
cargo run -- --amount <AMOUNT>
```

For example, to convert 100 GBP to CNY:

```sh
cargo run -- --amount 100
```

## Example Output

```
100.00 GBP is 922.05 CNY
```

## API Key

This tool uses the [ExchangeRate-API](https://www.exchangerate-api.com/) to fetch real-time exchange rates. You need to sign up for an API key and add it to the `.env` file in the project root.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

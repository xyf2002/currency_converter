# Currency Converter CLI

A simple CLI tool to convert currencies using real-time exchange rates.

## Features

- Converts between multiple currencies.
- Supports forward and reverse currency conversion.
- Fetches real-time exchange rates from an API.
- Displays currency symbols in the output.
- Lists available currencies.
- Interactive menu for easy use.

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
    cargo build --release
    ```

## Usage

### Convert Currency

To convert an amount from one currency to another, run the following command:

```sh
./target/release/currency_converter convert --amount <AMOUNT> --from <SOURCE_CURRENCY> --to <TARGET_CURRENCY>
```

For example, to convert 100 GBP to CNY:

```sh
./target/release/currency_converter convert --amount 100 --from GBP --to CNY
```

### Reverse Convert Currency

To reverse convert an amount to the source currency, run the following command:

```sh
./target/release/currency_converter reverse-convert --amount <AMOUNT> --from <SOURCE_CURRENCY> --to <TARGET_CURRENCY>
```

For example, to find out how much GBP is needed to get 100 CNY:

```sh
./target/release/currency_converter reverse-convert --amount 100 --from GBP --to CNY
```

### List Available Currencies

To list all available currencies, run:

```sh
./target/release/currency_converter list-currencies
```

### Interactive Menu

You can also run the tool in an interactive menu mode:

```sh
./target/release/currency_converter
```

In the interactive menu, you will have the following options:

```
=============================
  Currency Converter Menu:
=============================
  1. Convert currency
  2. Reverse convert currency
  3. List available currencies
  Q. Quit
=============================
Enter your choice:
```

Follow the prompts to perform the desired operation.

## Example Output

```
=============================
100.00 GBP is 922.05 ¥ (Rate as of 2024-06-22 12:00:00 UTC)
=============================

or

=============================
100.00 ¥ is 10.84 GBP (Rate as of 2024-06-22 12:00:00 UTC)
=============================
```

## API Key

This tool uses the [ExchangeRate-API](https://www.exchangerate-api.com/) to fetch real-time exchange rates. You need to sign up for an API key and add it to the `.env` file in the project root.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
```


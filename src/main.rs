use clap::{Parser, Subcommand};
use dotenv::dotenv;
use reqwest::blocking::Client;
use serde::Deserialize;
use std::collections::HashMap;
use std::env;
use std::io::{self, Write};

#[derive(Parser)]
#[command(name = "currency_converter")]
#[command(about = "Convert currencies using real-time exchange rates", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Convert an amount from one currency to another
    Convert {
        #[arg(short, long)]
        amount: f64,
        #[arg(short, long)]
        from: String,
        #[arg(short, long)]
        to: String,
    },
    /// Reverse convert an amount to the source currency
    ReverseConvert {
        #[arg(short, long)]
        amount: f64,
        #[arg(short, long)]
        from: String,
        #[arg(short, long)]
        to: String,
    },
    /// List available currencies
    ListCurrencies,
}

#[derive(Deserialize)]
struct ApiResponse {
    conversion_rates: HashMap<String, f64>,
    time_last_update_utc: String,
}

fn main() {
    dotenv().ok(); // Load environment variables from .env file if present

    let cli = Cli::parse();
    let api_key = env::var("EXCHANGE_RATE_API_KEY").expect("API key not set");

    if let Some(command) = cli.command {
        match command {
            Commands::Convert { amount, from, to } => {
                convert_currency(amount, &from, &to, &api_key);
            }
            Commands::ReverseConvert { amount, from, to } => {
                reverse_convert_currency(amount, &from, &to, &api_key);
            }
            Commands::ListCurrencies => {
                list_currencies();
            }
        }
    } else {
        loop {
            print_menu();
            let choice = get_user_input("Enter your choice: ");
            match choice.trim() {
                "1" => {
                    let (amount, from, to) = get_conversion_details();
                    convert_currency(amount, &from, &to, &api_key);
                }
                "2" => {
                    let (amount, from, to) = get_conversion_details();
                    reverse_convert_currency(amount, &from, &to, &api_key);
                }
                "3" => {
                    list_currencies();
                }
                "q" | "Q" => break,
                _ => println!("Invalid choice, please try again."),
            }
        }
    }
}

fn print_menu() {
    println!("Currency Converter Menu:");
    println!("1. Convert currency");
    println!("2. Reverse convert currency");
    println!("3. List available currencies");
    println!("Q. Quit");
}

fn get_user_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    input
}

fn get_conversion_details() -> (f64, String, String) {
    let currencies = vec![
        "CNY", "USD", "GBP", "EUR",
        // Add more currencies as needed
    ];

    let from = select_currency("Enter the source currency:", &currencies);
    let to = select_currency("Enter the target currency:", &currencies);
    let amount = get_user_input(&format!("Enter the amount ({}): ", from))
        .trim()
        .parse::<f64>()
        .expect("Invalid amount");

    (amount, from, to)
}

fn select_currency(prompt: &str, currencies: &[&str]) -> String {
    println!("{}", prompt);
    for (index, &currency) in currencies.iter().enumerate() {
        println!("{}. {}", index + 1, currency);
    }

    loop {
        let choice = get_user_input("Select a currency: ")
            .trim()
            .parse::<usize>();
        match choice {
            Ok(num) if num > 0 && num <= currencies.len() => {
                return currencies[num - 1].to_string()
            }
            _ => println!("Invalid choice, please try again."),
        }
    }
}

fn convert_currency(amount: f64, from: &str, to: &str, api_key: &str) {
    match get_conversion_rate(from, to, api_key) {
        Ok((rate, update_time)) => {
            let converted_amount = amount * rate;
            let symbols = get_currency_symbols();
            let from_symbol = symbols.get(from).unwrap_or(&from);
            let to_symbol = symbols.get(to).unwrap_or(&to);
            println!(
                "{:.2} {}{} is {:.2} {}{} (Rate as of {})",
                from_symbol, amount, from, to_symbol, converted_amount, to, update_time
            );
        }
        Err(e) => {
            eprintln!("Error fetching exchange rate: {}", e);
        }
    }
}

fn reverse_convert_currency(amount: f64, from: &str, to: &str, api_key: &str) {
    match get_conversion_rate(from, to, api_key) {
        Ok((rate, update_time)) => {
            let converted_amount = amount / rate;
            let symbols = get_currency_symbols();
            let from_symbol = symbols.get(from).unwrap_or(&from);
            let to_symbol = symbols.get(to).unwrap_or(&to);
            println!(
                "{:.2} {}{} is {:.2} {}{} (Rate as of {})",
                to_symbol, amount, to, from_symbol, converted_amount, from, update_time
            );
        }
        Err(e) => {
            eprintln!("Error fetching exchange rate: {}", e);
        }
    }
}

fn list_currencies() {
    let currencies = vec![
        "CNY", "USD", "GBP", "EUR",
        // Add more currencies as needed
    ];

    println!("Available currencies:");
    for currency in currencies {
        println!("{}", currency);
    }
}

fn get_currency_symbols() -> HashMap<&'static str, &'static str> {
    let mut symbols = HashMap::new();
    symbols.insert("CNY", "¥");
    symbols.insert("USD", "$");
    symbols.insert("GBP", "£");
    symbols.insert("EUR", "€");
    // Add more currency symbols as needed
    symbols
}

fn get_conversion_rate(
    from: &str,
    to: &str,
    api_key: &str,
) -> Result<(f64, String), Box<dyn std::error::Error>> {
    let url = format!(
        "https://v6.exchangerate-api.com/v6/{}/latest/{}",
        api_key, from
    );
    let client = Client::new();
    let response = client.get(&url).send()?.text()?;

    let api_response: ApiResponse = serde_json::from_str(&response)?;
    if let Some(rate) = api_response.conversion_rates.get(to) {
        Ok((*rate, api_response.time_last_update_utc))
    } else {
        Err(From::from("Rate not found"))
    }
}


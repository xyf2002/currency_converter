use clap::Parser;
use dotenv::dotenv;
use reqwest::blocking::Client;
use serde::Deserialize;
use std::env;

#[derive(Parser)]
#[command(name = "currency_converter")]
#[command(about = "Convert GBP to CNY using real-time exchange rates", long_about = None)]
struct Cli {
    #[arg(short, long)]
    amount: f64,
}

#[derive(Deserialize)]
struct ApiResponse {
    conversion_rates: std::collections::HashMap<String, f64>,
}

fn main() {
    dotenv().ok(); // Load environment variables from .env file if present

    let cli = Cli::parse();
    let amount_in_gbp = cli.amount;

    let api_key = env::var("EXCHANGE_RATE_API_KEY").expect("API key not set");

    match get_conversion_rate("GBP", "CNY", &api_key) {
        Ok(rate) => {
            let amount_in_cny = amount_in_gbp * rate;
            println!("{:.2} GBP is {:.2} CNY", amount_in_gbp, amount_in_cny);
        }
        Err(e) => {
            eprintln!("Error fetching exchange rate: {}", e);
        }
    }
}

fn get_conversion_rate(
    from: &str,
    to: &str,
    api_key: &str,
) -> Result<f64, Box<dyn std::error::Error>> {
    let url = format!(
        "https://v6.exchangerate-api.com/v6/{}/latest/{}",
        api_key, from
    );
    let client = Client::new();
    let response = client.get(&url).send()?.text()?;

    let api_response: ApiResponse = serde_json::from_str(&response)?;
    if let Some(rate) = api_response.conversion_rates.get(to) {
        Ok(*rate)
    } else {
        Err(From::from("Rate not found"))
    }
}


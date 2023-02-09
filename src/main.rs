use clap::Parser;
use colored::Colorize;
use rust_decimal::prelude::*;
use serde::{Deserialize, Serialize};

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
struct ExchangeRate {
    Cur_ID: u32,
    Date: String,
    Cur_Abbreviation: String,
    Cur_Scale: u32,
    Cur_Name: String,
    Cur_OfficialRate: f32,
}

#[derive(Parser)]
struct Cli {
    /// Date in format YYYY-MM-DD
    #[clap(short, long)]
    date: String,

    /// Amount of received USD
    #[clap(short, long)]
    amount: Option<f32>,
}

#[tokio::main]
async fn main() {
    let args = Cli::parse();
    let date = &args.date;
    let amount = Decimal::from_f32(args.amount.unwrap_or(0.0));

    let url = format!(
        "https://www.nbrb.by/api/exrates/rates/431?ondate={date}",
        date = date
    );

    let client = reqwest::Client::new();
    let response = client.get(&url).send().await.unwrap();
    let mut official_rate = Decimal::new(0, 0);

    match response.status() {
        reqwest::StatusCode::OK => match response.json::<ExchangeRate>().await {
            Ok(exchange_rate) => {
                official_rate = Decimal::from_f32(exchange_rate.Cur_OfficialRate).unwrap();
            }
            Err(_) => println!("Error"),
        },
        reqwest::StatusCode::NOT_FOUND => println!("Rate is not found for the given date"),
        _ => println!("Unknown error"),
    }

    println!(
        "USD rate for {date} is {rate}",
        date = date.bold().green(),
        rate = official_rate.to_string().bold().green()
    );

    if let Some(amount) = amount {
        if amount == Decimal::new(0, 0) {
            return;
        }

        let result = amount * official_rate;
        println!(
            "You received (in BYN): {}",
            result.to_string().bold().green()
        );
    }
}

use serde::{Deserialize, Serialize};
use std::env;

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

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    let date = &args[1];
    // format YYYY-MM-DD
    let amount = &args[2].parse::<f32>().unwrap();

    let url = format!(
        "https://www.nbrb.by/api/exrates/rates/431?ondate={date}",
        date = date
    );

    let client = reqwest::Client::new();
    let response = client.get(&url).send().await.unwrap();
    let mut official_rate: f32 = 0.0;

    match response.status() {
        reqwest::StatusCode::OK => {
            match response.json::<ExchangeRate>().await {
                Ok(exchange_rate) => {
                    official_rate = exchange_rate.Cur_OfficialRate;
                },
                Err(_) => println!("Error"),
            }
        },
        reqwest::StatusCode::NOT_FOUND => println!("Rate is not found for the given date"),
        _ => println!("Unknown error"),
    }

    // Convert USD to BYN
    let result = amount * official_rate;
    println!("Rate: {}", official_rate);
    println!("Result: {}", result);
}

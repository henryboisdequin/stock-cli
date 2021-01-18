use exitfailure::ExitFailure;
use reqwest::Url;
use serde_derive::{Deserialize, Serialize};
use std::env;

#[derive(Serialize, Deserialize, Debug)]
struct CompanyQuote {
    c: f64,
    h: f64,
    l: f64,
    o: f64,
    pc: f64,
    t: i128,
}

impl CompanyQuote {
    async fn get(symbol: &String, api_key: &String) -> Result<Self, ExitFailure> {
        let url = format!(
            "https://finnhub.io/api/v1/quote?symbol={}&token={}",
            symbol, api_key
        );

        let url = Url::parse(&*url)?;
        let res = reqwest::get(url).await?.json::<CompanyQuote>().await?;

        Ok(res)
    }
}

#[tokio::main]
async fn main() -> Result<(), ExitFailure> {
    let api_key = "c0169gn48v6sc26qrvtg".to_string();
    let args: Vec<String> = env::args().collect();
    let mut symbol: String = "AAPL".to_string();

    if args.len() < 2 {
        println!("Since you didn't specify a company symbol, it has defaulted to AAPL.");
    } else {
        symbol = args[1].clone();
    }

    let res = CompanyQuote::get(&symbol, &api_key).await?;
    println!("{}'s current stock price: {}", symbol, res.c);

    Ok(())
}

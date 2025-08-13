use serde::Deserialize;
use std::{fs::File, io::Write, thread, time::Duration};

#[derive(Debug, Deserialize)]
struct CoinGeckoResponse {
    bitcoin: Option<CoinData>,
    ethereum: Option<CoinData>,
}

#[derive(Debug, Deserialize)]
struct CoinData {
    usd: f64,
}

#[derive(Debug, Deserialize)]
struct AlphaVantageResponse {
    #[serde(rename = "Global Quote")]
    global_quote: Option<StockData>,
}

#[derive(Debug, Deserialize)]
struct StockData {
    #[serde(rename = "05. price")]
    price: String,
}

fn fetch_crypto_price(id: &str) -> Result<f64, Box<dyn std::error::Error>> {
    let url = format!(
        "https://api.coingecko.com/api/v3/simple/price?ids={}&vs_currencies=usd",
        id
    );
    let resp: CoinGeckoResponse = ureq::get(&url)
        .call()?
        .into_json()?;

    match id {
        "bitcoin" => Ok(resp.bitcoin.ok_or("No Bitcoin data")?.usd),
        "ethereum" => Ok(resp.ethereum.ok_or("No Ethereum data")?.usd),
        _ => Err("Unknown crypto ID".into()),
    }
}

fn fetch_stock_price(symbol: &str) -> Result<f64, Box<dyn std::error::Error>> {
    let api_key = "demo"; // Replace with your real Alpha Vantage API key
    let url = format!(
        "https://www.alphavantage.co/query?function=GLOBAL_QUOTE&symbol={}&apikey={}",
        symbol, api_key
    );
    let resp: AlphaVantageResponse = ureq::get(&url)
        .call()?
        .into_json()?;

    let price_str = resp.global_quote.ok_or("No stock data")?.price;
    Ok(price_str.parse::<f64>()?)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    loop {
        // Fetch crypto prices (CoinGecko)
        let btc_price = fetch_crypto_price("bitcoin")?;
        let eth_price = fetch_crypto_price("ethereum")?;
        println!("Bitcoin: ${:.2}", btc_price);
        println!("Ethereum: ${:.2}", eth_price);

        // Fetch stock price (Alpha Vantage with demo key)
        let ibm_price = fetch_stock_price("IBM")?;
        println!("IBM: ${:.2}", ibm_price);

        // Wait 10 seconds before the next cycle
        std::thread::sleep(std::time::Duration::from_secs(10));
    }
}

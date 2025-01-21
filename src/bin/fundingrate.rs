use futures_util::{SinkExt, StreamExt};
use serde::Deserialize;
use tokio_tungstenite::connect_async;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let binance_ws_url = "wss://fstream.binance.com/ws/!markPrice@arr";

    println!("Connecting to Binance WebSocket...");
    let (ws_stream, _) = connect_async(binance_ws_url).await?;
    println!("Connected!");

    let (mut _write, mut read) = ws_stream.split();

    // Listen for messages
    while let Some(message) = read.next().await {
        match message {
            Ok(msg) if msg.is_text() => {
                println!("{}", msg.to_text()?);
                /*let data: BinanceFundingUpdate = serde_json::from_str(msg.to_text()?)?;
                println!(
                    "Funding Update - Symbol: {}, Mark Price: {}, Funding Rate: {}",
                    data.symbol, data.mark_price, data.funding_rate
                );*/
            }
            Ok(_) => {}
            Err(e) => {
                eprintln!("Error: {:?}", e);
                break;
            }
        }
    }

    Ok(())
}

#[derive(Deserialize)]
struct BinanceFundingUpdate {
    symbol: String,
    mark_price: String,
    funding_rate: String,
}

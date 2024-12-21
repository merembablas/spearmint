use super::{bot, result, storage};
use binance::websockets::*;
use std::sync::atomic::AtomicBool;
use ta::indicators::MoneyFlowIndex;
use ta::DataItem;
use ta::Next;

pub fn run(path: &str) {
    let keep_running = AtomicBool::new(true);
    let mut mfi = MoneyFlowIndex::new(14).unwrap();
    let mut web_socket: WebSockets = WebSockets::new(|event: WebsocketEvent| {
        match event {
            WebsocketEvent::Kline(kline_event) => {
                let symbol = kline_event.kline.symbol.clone();
                let kline = kline_event.kline;
                let open = kline.open.parse::<f64>().unwrap_or(0.0);
                let high = kline.high.parse::<f64>().unwrap_or(0.0);
                let low = kline.low.parse::<f64>().unwrap_or(0.0);
                let close = kline.close.parse::<f64>().unwrap_or(0.0);
                let volume = kline.volume.parse::<f64>().unwrap_or(0.0);
                let is_kline_closed = kline.is_final_bar;

                if is_kline_closed {
                    let di = DataItem::builder()
                        .high(high)
                        .low(low)
                        .close(close)
                        .open(open)
                        .volume(volume)
                        .build()
                        .unwrap();
                    let mf_val = mfi.next(&di);

                    storage::create_ticker(
                        path,
                        result::Ticker {
                            pair: symbol.clone(),
                            open,
                            high,
                            low,
                            close,
                            volume,
                            mfi: mf_val,
                        },
                    );

                    println!(
                        "Symbol: {}, High: {:.4}, Low: {:.4}, Close: {:.4}, Volume: {:.2}, MFI: {:.2}",
                        symbol, high, low, close, volume, mf_val
                    );
                }
            }
            _ => (),
        }

        Ok(())
    });

    let bots = bot::active().unwrap();

    let endpoints: Vec<String> = bots
        .iter()
        .map(|bot| format!("{}@kline_5m", bot.pair.to_lowercase()))
        .collect();

    web_socket.connect_multiple_streams(&endpoints).unwrap();

    if let Err(e) = web_socket.event_loop(&keep_running) {
        println!("Error: {:?}", e);
    }
}

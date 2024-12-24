use super::{bot, result, storage};
use binance::websockets::*;
use std::collections::HashMap;
use std::sync::atomic::AtomicBool;
use ta::indicators::MoneyFlowIndex;
use ta::DataItem;
use ta::Next;

pub fn run(path: &str) {
    let keep_running = AtomicBool::new(true);
    let bots = bot::active().unwrap();

    let mut mfis: HashMap<String, MoneyFlowIndex> = HashMap::new();
    for val in bots.iter() {
        let tickers = storage::get_tickers(&val.pair, 20);
        let mut mf = MoneyFlowIndex::new(14).unwrap();
        for tick in tickers.iter().rev() {
            let ticker = tick.as_ref().unwrap();
            let di = DataItem::builder()
                .high(ticker.high)
                .low(ticker.low)
                .close(ticker.close)
                .open(ticker.open)
                .volume(ticker.volume)
                .build()
                .unwrap();

            mf.next(&di);
        }

        mfis.insert(val.pair.clone(), mf);
    }

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

                    let mut mf_val = 0.0;
                    if let Some(value) = mfis.get_mut(&symbol) {
                        mf_val = value.next(&di);
                    }

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

    let endpoints: Vec<String> = bots
        .iter()
        .map(|bot| format!("{}@kline_5m", bot.pair.to_lowercase()))
        .collect();

    web_socket.connect_multiple_streams(&endpoints).unwrap();

    if let Err(e) = web_socket.event_loop(&keep_running) {
        println!("Error: {:?}", e);
        run(path);
    }
}

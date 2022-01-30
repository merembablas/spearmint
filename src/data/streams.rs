use binance::websockets::*;
use comfy_table::Table;
use std::collections::HashMap;
use std::sync::atomic::AtomicBool;

pub fn run(exchange: &str, symbols: Vec<String>) {
    if exchange == "binance" {
        let mut endpoints: Vec<String> = Vec::new();
        let mut prices: HashMap<String, f32> = HashMap::new();

        for symbol in symbols.iter() {
            endpoints.push(format!("{}@ticker", symbol.to_lowercase()));
        }

        let keep_running = AtomicBool::new(true);
        let mut web_socket: WebSockets<'_> = WebSockets::new(|event: WebsocketEvent| {
            if let WebsocketEvent::DayTicker(ticker_event) = event {
                prices.insert(
                    ticker_event.symbol,
                    ticker_event.current_close.parse().unwrap(),
                );

                print!("\x1B[2J\x1B[1;1H");

                let mut table = Table::new();
                table.set_header(vec!["Pair", "Price"]);
                for (pair, price) in &prices {
                    table.add_row(vec![pair, &format!("{}", price)]);
                }

                println!("{}", table);
            }

            Ok(())
        });

        web_socket.connect_multiple_streams(&endpoints).unwrap();

        if let Err(e) = web_socket.event_loop(&keep_running) {
            println!("Error: {:?}", e);
        }

        web_socket.disconnect().unwrap();
    }
}

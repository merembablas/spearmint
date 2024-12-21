use crate::model::{storage, Exchange, Strategy};
use crate::strategy;
use binance::websockets::*;
use comfy_table::Table;
use std::sync::atomic::AtomicBool;
use std::sync::Arc;
use std::time::{Duration, Instant};

pub fn run(bot: Arc<super::bot::Bot<impl Exchange, impl Strategy>>, duration: u64) {
    let keep_running = AtomicBool::new(true);
    let mut last_block_time = Instant::now();
    let block_interval = Duration::from_secs(duration);
    let info = bot.info.as_ref().unwrap();

    let mut web_socket: WebSockets<'_> = WebSockets::new(|event: WebsocketEvent| {
        if let WebsocketEvent::DayTicker(ticker_event) = event {
            print!("\x1B[2J\x1B[1;1H");

            let state = match storage::get_latest_state(&info.platform, &info.pair) {
                Ok(state) => state,
                Err(_error) => Default::default(),
            };

            let wallet = storage::get_wallet(&info.quote);
            let avg_price = storage::get_avg_price(&info.platform, &info.pair, state.cycle);
            let avg_percent_change = strategy::calculate_percent_change(
                avg_price,
                ticker_event.current_close.parse().unwrap(),
            );
            let mfi = storage::get_latest_mfi(&info.pair);

            let mut table = Table::new();
            table.set_header(vec![
                "MFI",
                "Pair",
                "Price",
                "AVG",
                "P.Change",
                "T.Price",
                "B.Price",
                "Wallet",
                "Cycle",
                "M.Position",
            ]);
            table.add_row(vec![
                &format!("{:.4}", mfi),
                &info.pair,
                &format!("{:.4}", ticker_event.current_close),
                &format!("{:.4}", avg_price),
                &format!("{:.2}%", avg_percent_change),
                &format!("{:.4}", state.top_price),
                &format!("{:.4}", state.bottom_price),
                &format!("{:.4}", wallet),
                &format!("{}", state.cycle),
                &format!("{}", state.margin_position),
            ]);

            println!("{}", table);

            if last_block_time.elapsed() >= block_interval {
                bot.update(ticker_event.current_close.parse().unwrap());
                last_block_time = Instant::now();
            }
        }

        Ok(())
    });

    web_socket
        .connect(&format!("{}@ticker", &info.pair.to_lowercase()))
        .unwrap();

    if let Err(e) = web_socket.event_loop(&keep_running) {
        println!("Error: {:?}", e);
        run(bot.clone(), duration);
    }

    web_socket.disconnect().unwrap();
}

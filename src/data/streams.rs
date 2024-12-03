use crate::data::action;
use crate::data::result::Bot;
use crate::strategy::{helldiver, martingle};
use binance::websockets::*;
use comfy_table::Table;
use std::sync::atomic::AtomicBool;
use std::time::{Duration, Instant};

pub fn run(exchange: &str, bot: &Bot, duration: u64) {
    let keep_running = AtomicBool::new(true);
    let mut last_block_time = Instant::now();
    let block_interval = Duration::from_secs(duration);

    let mut web_socket: WebSockets<'_> = WebSockets::new(|event: WebsocketEvent| {
        if let WebsocketEvent::DayTicker(ticker_event) = event {
            print!("\x1B[2J\x1B[1;1H");

            let state = match action::get_latest_state(&bot.platform, &bot.pair) {
                Ok(state) => state,
                Err(_error) => Default::default(),
            };

            let wallet = action::get_wallet();
            let avg_price = action::get_avg_price(&bot.platform, &bot.pair, state.cycle);
            let avg_percent_change = action::calculate_percent_change(
                avg_price,
                ticker_event.current_close.parse().unwrap(),
            );

            let mut table = Table::new();
            table.set_header(vec![
                "Platform",
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
                &bot.platform,
                &bot.pair,
                &format!("{}", ticker_event.current_close),
                &format!("{}", avg_price),
                &format!("{}%", avg_percent_change),
                &format!("{}", state.top_price),
                &format!("{}", state.bottom_price),
                &format!("{}", wallet),
                &format!("{}", state.cycle),
                &format!("{}", state.margin_position),
            ]);

            println!("{}", table);

            if last_block_time.elapsed() >= block_interval {
                println!("Execute strategy...");

                if bot.strategy == "helldiver" {
                    helldiver::execute_strategy(bot, &ticker_event.current_close);
                } else {
                    martingle::execute_strategy(bot, &ticker_event.current_close);
                }

                last_block_time = Instant::now(); // Reset the timer
            }
        }

        Ok(())
    });

    web_socket
        .connect(&format!("{}@ticker", bot.pair.to_lowercase()))
        .unwrap();

    if let Err(e) = web_socket.event_loop(&keep_running) {
        println!("Error: {:?}", e);

        run(exchange, bot, duration);
    }

    web_socket.disconnect().unwrap();
}

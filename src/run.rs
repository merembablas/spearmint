use crate::bot::{BotBuilder, BotInfo};
use crate::connector::binance as conn_binance;
use crate::model;
use crate::model::{result, storage, Exchange, Strategy};
use crate::strategy;
use binance::websockets::*;
use comfy_table::Table;
use log::error;
use std::sync::atomic::AtomicBool;
use std::sync::Arc;
use std::thread;
use std::time::{Duration, Instant};
use std::{fs, panic};

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
            let mfi_dir = if mfi[0] > mfi[1] {
                "UP".to_string()
            } else {
                "DOWN".to_string()
            };

            let mut table = Table::new();
            table.set_header(vec![
                "MFI",
                "Pair",
                "Price",
                "AVG",
                "P.Change",
                "T.Price",
                "B.Price",
                "B.MFI",
                "MFI Dir",
                "Wallet",
                "Cycle",
                "M.Position",
            ]);
            table.add_row(vec![
                &format!("{:.4}", mfi[0]),
                &info.pair,
                &format!("{:.4}", ticker_event.current_close),
                &format!("{:.4}", avg_price),
                &format!("{:.2}%", avg_percent_change),
                &format!("{:.4}", state.top_price),
                &format!("{:.4}", state.bottom_price),
                &format!("{:.4}", state.bottom_mfi),
                &format!("{}", mfi_dir),
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

pub fn run_all(bots: Vec<result::Bot>, duration: u64) {
    if let Err(e) = setup_logger() {
        eprintln!("Failed to set up logger: {}", e);
        return;
    }

    panic::set_hook(Box::new(|info| {
        let panic_message = match info.payload().downcast_ref::<&str>() {
            Some(msg) => *msg,
            None => "Unknown panic message.",
        };

        error!("Application panicked: {}", panic_message);
    }));

    loop {
        print!("{esc}c", esc = 27 as char);

        let mut table = Table::new();
        table.set_header(vec![
            "MFI",
            "Pair",
            "Price",
            "AVG",
            "P.Change",
            "T.Price",
            "B.Price",
            "B.MFI",
            "MFI Dir",
            "Wallet",
            "Cycle",
            "M.Position",
        ]);

        for val in bots.iter() {
            let strategy = strategy::helldiver::HellDiverStrategy {
                first_buy_in: val.parameters.first_buy_in,
                entry: val.parameters.entry.clone(),
                take_profit: val.parameters.take_profit.clone(),
                margin_configuration: val.margin.margin_configuration.clone(),
            };
            let credential = model::bind::get(&val.platform);
            let account =
                conn_binance::Connector::from_credential(credential.api, credential.secret);
            let bot =
                BotBuilder::<conn_binance::Connector, strategy::helldiver::HellDiverStrategy>::new(
                )
                .with_info(BotInfo {
                    platform: val.platform.clone(),
                    pair: val.pair.clone(),
                    base: val.base.clone(),
                    quote: val.quote.clone(),
                })
                .with_strategy(strategy)
                .with_connector(account)
                .build();

            let ticker = storage::get_latest_price("ticker1m.db", &val.pair);

            bot.update(ticker.close);

            let state = match storage::get_latest_state(&val.platform, &val.pair) {
                Ok(state) => state,
                Err(_error) => Default::default(),
            };

            let wallet = storage::get_wallet(&val.quote);
            let avg_price = storage::get_avg_price(&val.platform, &val.pair, state.cycle);
            let avg_percent_change = strategy::calculate_percent_change(avg_price, ticker.close);
            let mfi = storage::get_latest_mfi1m(&val.pair);
            let mfi_dir = if mfi[0] > mfi[1] {
                "UP".to_string()
            } else {
                "DOWN".to_string()
            };

            table.add_row(vec![
                &format!("{:.4}", &mfi[0]),
                &val.pair,
                &format!("{:.4}", ticker.close),
                &format!("{:.4}", avg_price),
                &format!("{:.2}%", avg_percent_change),
                &format!("{:.4}", state.top_price),
                &format!("{:.4}", state.bottom_price),
                &format!("{:.4}", state.bottom_mfi),
                &format!("{}", mfi_dir),
                &format!("{:.4}", wallet),
                &format!("{}", state.cycle),
                &format!("{}", state.margin_position),
            ]);
        }

        println!("{}", table);

        thread::sleep(Duration::from_secs(duration));
    }
}

fn setup_logger() -> Result<(), Box<dyn std::error::Error>> {
    let log_file_path = "error.log";

    fs::OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(false)
        .open(log_file_path)?;

    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{} [{}] {}",
                chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
                record.level(),
                message
            ))
        })
        .level(log::LevelFilter::Info)
        .level_for("my_crate", log::LevelFilter::Error)
        .chain(fern::log_file(log_file_path)?)
        .apply()?;
    Ok(())
}

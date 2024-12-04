use crate::data::action;
use crate::data::bot;
use crate::data::result::Bot;
use reqwest::blocking::Client;
use reqwest::header;
use serde::Serialize;
use std::collections::HashMap;
use std::thread;
use std::time::{Duration, Instant};

pub fn run(token: String, chat_id: u64, duration: u64) {
    let interval = Duration::from_secs(duration);
    let mut last_execution_time = Instant::now();
    let bots = bot::active().unwrap();
    let mut trackers: HashMap<String, i32> = HashMap::new();

    for bot in bots.clone() {
        trackers.insert(bot.pair, 0);
    }

    println!("Notification process running...");

    loop {
        if last_execution_time.elapsed() >= interval {
            perform_task(&mut trackers, &bots, &token, chat_id);
            last_execution_time = Instant::now();
        }
        thread::sleep(Duration::from_millis(100));
    }
}

fn perform_task(trackers: &mut HashMap<String, i32>, bots: &Vec<Bot>, token: &str, chat_id: u64) {
    let mut table = String::from("```\n");
    table.push_str("| Name      | Cycle   | PNL        |\n");
    table.push_str("|-----------|---------|------------|\n");

    let mut is_send = false;
    for bot in bots {
        let pnl = action::get_latest_pnl_trade(&bot.platform, &bot.pair).unwrap();

        let cycle = match trackers.get(&pnl.pair) {
            Some(&number) => number,
            _ => 0,
        };

        if pnl.cycle > cycle as u64 {
            table.push_str(&format!(
                "| {:<8}  | {:<7} | {:<7.3}    |\n",
                pnl.pair,
                pnl.cycle,
                pnl.pnl - 0.05,
            ));

            is_send = true;
            trackers.insert(String::from(&pnl.pair), pnl.cycle as i32);
        }
    }

    table.push_str("```");

    if is_send {
        send_telegram_notification(token, chat_id, &table).unwrap();
    }

    println!("Task executed at {:?}", Instant::now());
}

#[derive(Serialize)]
struct TelegramMessage {
    chat_id: u64,
    text: String,
    parse_mode: String,
}

fn send_telegram_notification(
    bot_token: &str,
    chat_id: u64,
    message: &str,
) -> Result<(), reqwest::Error> {
    let api_url = format!("https://api.telegram.org/bot{}/sendMessage", bot_token);

    let payload = TelegramMessage {
        chat_id: chat_id,
        text: message.to_string(),
        parse_mode: "MarkdownV2".to_string(),
    };

    let client = Client::new();
    let response = client
        .post(&api_url)
        .header(header::CONTENT_TYPE, "application/json")
        .json(&payload)
        .send()?;

    if response.status().is_success() {
        println!("Message sent successfully!");
    } else {
        eprintln!("Failed to send message: {:?}", response.text()?);
    }

    Ok(())
}

mod cli;
mod connector;
mod data;
mod notification;
mod run;
mod strategy;

use clap::{Parser, Subcommand};
use connector::binance;
use std::path::PathBuf;

#[derive(Parser)]
#[clap(author, version, about, long_about=None)]
struct Args {
    #[clap(subcommand)]
    command: Option<Commands>,

    #[clap(short, long, value_name = "FILE", default_value = "spearmint.db")]
    database: String,
}

#[derive(Subcommand)]
enum Commands {
    Apply {
        #[clap(short, long, parse(from_os_str), value_name = "FILE")]
        file: Option<PathBuf>,
    },

    Test {},

    Account {
        #[clap(short, long)]
        platform: String,
    },

    Status {
        #[clap(short, long)]
        name: String,
    },

    Start {
        #[clap(short, long)]
        name: String,
    },

    Stop {
        #[clap(short, long)]
        name: String,
    },

    Delete {
        #[clap(short, long)]
        name: String,
    },

    List {},

    Notification {
        #[clap(short, long)]
        token: String,

        #[clap(short, long)]
        chat_id: u64,

        #[clap(short, long, default_value = "30")]
        duration: u64,
    },

    Bot {
        #[clap(short, long)]
        name: String,
    },

    Setup {
        #[clap(short, long, value_name = "FILE", default_value = "spearmint.db")]
        path: String,
    },

    Run {
        #[clap(short, long)]
        name: String,

        #[clap(short, long, default_value = "30")]
        duration: u64,
    },
}

fn main() {
    let args = Args::parse();

    match &args.command {
        Some(Commands::Apply { file }) => {
            if let Some(config_path) = file.as_deref() {
                let content = std::fs::read_to_string(config_path).unwrap();
                let kind: data::args::Kind = toml::from_str(&content).unwrap();

                if kind.kind == "bot" {
                    let bot: data::args::Config = toml::from_str(&content).unwrap();
                    let config = data::bot::save(bot);
                    cli::display_bot(config);
                } else if kind.kind == "bind" {
                    let binding: data::args::ApiCredential = toml::from_str(&content).unwrap();
                    let api = data::bind::save(binding);
                    cli::display_bind(api);
                }
            }
        }

        Some(Commands::Status { name }) => {
            println!("args {}", args.database);
            println!("Status ID: {}", name);
        }

        Some(Commands::Start { name }) => {
            data::start(name);
        }

        Some(Commands::Stop { name }) => {
            data::stop(name);
        }

        Some(Commands::Delete { name }) => {
            data::delete(name);
        }

        Some(Commands::List {}) => match data::bot::all() {
            Ok(bots) => cli::display_bots(bots),
            Err(e) => println!("error: {}", e),
        },

        Some(Commands::Bot { name }) => match data::bot::get(name) {
            Ok(bot) => cli::display_bot(data::args::Config {
                title: bot.title,
                general: data::args::General {
                    pair: bot.pair,
                    platform: bot.platform,
                    strategy: bot.strategy,
                },
                parameters: data::args::Parameters {
                    cycle: bot.parameters.cycle,
                    first_buy_in: bot.parameters.first_buy_in,
                    take_profit_ratio: bot.parameters.take_profit_ratio,
                    earning_callback: bot.parameters.earning_callback,
                },
                margin: data::args::Margin {
                    margin_configuration: bot.margin.margin_configuration,
                },
            }),
            Err(e) => println!("error: {}", e),
        },

        Some(Commands::Setup { path }) => {
            data::setup(path).expect("Failed to create database");
        }

        Some(Commands::Run { name, duration }) => match data::bot::get(name) {
            Ok(bot) => data::streams::run("binance", &bot, *duration),
            Err(e) => println!("error: {}", e),
        },

        Some(Commands::Account { platform }) => {
            let api_credential = data::bind::get(platform);
            let account = binance::Account {
                api_key: api_credential.api,
                api_secret: api_credential.secret,
            };

            cli::display_balances(account.get_balances());
        }

        Some(Commands::Test {}) => {}

        Some(Commands::Notification {
            token,
            chat_id,
            duration,
        }) => {
            notification::telegram::run(token.clone(), *chat_id, *duration);
        }

        None => {}
    }
}

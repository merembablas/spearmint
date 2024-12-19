mod bot;
mod cli;
mod connector;
mod model;
mod notification;
mod run;
mod strategy;

use crate::model::Exchange;
use bot::BotBuilder;
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
                let kind: model::args::Kind = toml::from_str(&content).unwrap();

                if kind.kind == "bot" {
                    let bot: model::args::Config = toml::from_str(&content).unwrap();
                    let config = model::bot::save(bot);
                    cli::display_bot(config);
                } else if kind.kind == "bind" {
                    let binding: model::args::ApiCredential = toml::from_str(&content).unwrap();
                    let api = model::bind::save(binding);
                    cli::display_bind(api);
                }
            }
        }

        Some(Commands::Status { name }) => {
            println!("args {}", args.database);
            println!("Status ID: {}", name);
        }

        Some(Commands::Start { name }) => {
            model::start(name);
        }

        Some(Commands::Stop { name }) => {
            model::stop(name);
        }

        Some(Commands::Delete { name }) => {
            model::delete(name);
        }

        Some(Commands::List {}) => match model::bot::all() {
            Ok(bots) => cli::display_bots(bots),
            Err(e) => println!("error: {}", e),
        },

        Some(Commands::Bot { name }) => match model::bot::get(name) {
            Ok(bot) => cli::display_bot(model::args::Config {
                title: bot.title,
                general: model::args::General {
                    pair: bot.pair,
                    base: bot.base,
                    quote: bot.quote,
                    platform: bot.platform,
                    strategy: bot.strategy,
                },
                parameters: model::args::Parameters {
                    cycle: bot.parameters.cycle,
                    first_buy_in: bot.parameters.first_buy_in,
                    take_profit_ratio: bot.parameters.take_profit_ratio,
                    earning_callback: bot.parameters.earning_callback,
                },
                margin: model::args::Margin {
                    margin_configuration: bot.margin.margin_configuration,
                },
            }),
            Err(e) => println!("error: {}", e),
        },

        Some(Commands::Setup { path }) => {
            model::setup(path).expect("Failed to create database");
        }

        Some(Commands::Run { name, duration }) => match model::bot::get(name) {
            Ok(config) => {
                let strategy = strategy::helldiver::HellDiverStrategy {
                    first_buy_in: config.parameters.first_buy_in,
                    first_entry: vec![-1.0, 0.3],
                    take_profit_ratio: config.parameters.take_profit_ratio,
                    earning_callback: config.parameters.earning_callback,
                    margin_configuration: config.margin.margin_configuration,
                };
                let credential = model::bind::get(&config.platform);
                let account =
                    binance::Connector::from_credential(credential.api, credential.secret);
                let bot =
                    BotBuilder::<binance::Connector, strategy::helldiver::HellDiverStrategy>::new()
                        .with_info(bot::BotInfo {
                            platform: config.platform,
                            pair: config.pair,
                            base: config.base,
                            quote: config.quote,
                        })
                        .with_strategy(strategy)
                        .with_connector(account)
                        .build();

                run::run(bot, *duration);
            }
            Err(e) => println!("error: {}", e),
        },

        Some(Commands::Account { platform }) => {
            let credential = model::bind::get(platform);
            let account = binance::Connector::from_credential(credential.api, credential.secret);

            cli::display_balances(account.get_balances());
        }

        Some(Commands::Test {}) => {
            println!("test");
        }

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

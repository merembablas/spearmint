mod cli;
mod data;

use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[clap(author, version, about, long_about=None)]
struct Args {
    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Apply {
        #[clap(short, long, parse(from_os_str), value_name = "FILE")]
        file: Option<PathBuf>,
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

    Bot {
        #[clap(short, long)]
        name: String,
    },

    Setup {
        #[clap(short, long, value_name = "FILE", default_value = "spearmint.db")]
        path: String,
    },

    Run {},
}

fn main() {
    let args = Args::parse();

    match &args.command {
        Some(Commands::Apply { file }) => {
            if let Some(config_path) = file.as_deref() {
                let content = std::fs::read_to_string(config_path).unwrap();
                let kind: data::args::Kind = toml::from_str(&content).unwrap();

                if kind.kind == "bot" {
                    let channel: data::args::Config = toml::from_str(&content).unwrap();
                    let config = data::save(channel);
                    cli::display_bot(config);
                } else if kind.kind == "bind" {
                    let binding: data::args::ApiKey = toml::from_str(&content).unwrap();
                    let api = data::save_api_key(binding);
                    cli::display_bind(api);
                }
            }
        }

        Some(Commands::Status { name }) => {
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

        Some(Commands::List {}) => {
            let bots = data::list();
            cli::display_bots(bots.unwrap());
        }

        Some(Commands::Bot { name }) => {
            let bot = data::get_bot(name).unwrap();
            cli::display_bot(data::args::Config {
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
            });
        }

        Some(Commands::Setup { path }) => {
            data::setup(path).expect("Failed to create database");
        }

        Some(Commands::Run {}) => {
            let bots: Vec<_> = data::list_active().unwrap();
            data::streams::run("binance", bots);
        }

        None => {}
    }
}

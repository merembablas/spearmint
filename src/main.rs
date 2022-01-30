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
        name: String,
    },

    Start {
        name: String,
    },

    Stop {
        name: String,
    },

    Delete {
        name: String,
    },

    List {},

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
                let package_info: data::args::Config = toml::from_str(&content).unwrap();

                let config = data::save(package_info);
                cli::display_config(config);
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

        Some(Commands::Setup { path }) => {
            data::setup(path).expect("Failed to create database");
        }

        Some(Commands::Run {}) => {
            let symbols: Vec<_> = data::list_active()
                .unwrap()
                .into_iter()
                .map(String::from)
                .collect();

            data::streams::run("binance", symbols);
        }

        None => {}
    }
}

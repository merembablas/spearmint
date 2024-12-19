use comfy_table::Table;

use crate::model;

pub fn display_bot(package_info: model::args::Config) {
    let mut table = Table::new();
    table
        .set_header(vec!["Name", "Value"])
        .add_row(vec!["Title", &package_info.title])
        .add_row(vec!["Pair", &package_info.general.pair])
        .add_row(vec!["Base", &package_info.general.base])
        .add_row(vec!["Quote", &package_info.general.quote])
        .add_row(vec!["Platform", &package_info.general.platform])
        .add_row(vec!["Strategy", &package_info.general.strategy])
        .add_row(vec!["Cycle", &package_info.parameters.cycle])
        .add_row(vec![
            "First Buy In",
            &format!("{}", package_info.parameters.first_buy_in),
        ])
        .add_row(vec![
            "Take Profit Ratio",
            &format!("{}", package_info.parameters.take_profit_ratio),
        ])
        .add_row(vec![
            "Earning Callback",
            &format!("{}", package_info.parameters.earning_callback),
        ]);

    println!("{}", table);

    let mut margin_table = Table::new();
    margin_table.set_header(vec!["Margin", "Allocation"]);
    let mut margin_configuration = package_info.margin.margin_configuration;
    margin_configuration.reverse();
    while let Some(set) = margin_configuration.pop() {
        margin_table.add_row(&set);
    }

    println!("{}", margin_table);
}

pub fn display_bots(bots: Vec<crate::model::result::Bot>) {
    let mut table = Table::new();
    table.set_header(vec![
        "Title", "Pair", "Platform", "Strategy", "Cycle", "Status",
    ]);

    for bot in bots {
        table.add_row(vec![
            bot.title,
            bot.pair,
            bot.platform,
            bot.strategy,
            bot.parameters.cycle,
            bot.status,
        ]);
    }

    println!("{}", table);
}

pub fn display_bind(api: crate::model::result::ApiCredential) {
    let mut table = Table::new();
    table
        .set_header(vec!["Platform", "Api Key", "Secret Key"])
        .add_row(vec![api.platform, api.api, api.secret]);

    println!("{}", table);
}

pub fn display_balances(balances: Vec<crate::model::result::Balance>) {
    let mut table = Table::new();
    table.set_header(vec!["Asset", "Free"]);

    for b in balances {
        table.add_row(vec![b.asset, format!("{}", b.free)]);
    }

    println!("{}", table);
}

use comfy_table::Table;

use crate::data;

pub fn display_channel(package_info: data::args::Config) {
    let mut table = Table::new();
    table
        .set_header(vec!["Name", "Value"])
        .add_row(vec!["Title", &package_info.title])
        .add_row(vec!["Pair", &package_info.general.pair])
        .add_row(vec!["Exchange", &package_info.general.exchange])
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
            "Margin Call Limit",
            &format!("{}", package_info.parameters.margin_call_limit),
        ])
        .add_row(vec![
            "Earning Callback",
            &format!("{}", package_info.parameters.earning_callback),
        ])
        .add_row(vec![
            "Sub Position Callback",
            &format!("{}", package_info.parameters.sub_position_start),
        ])
        .add_row(vec![
            "Sub Position Earning Callback",
            &format!("{}", package_info.margin.sub_position_earning_callback),
        ]);

    println!("{}", table);

    let mut margin_table = Table::new();
    margin_table.set_header(vec!["Margin", "Allocation"]);
    let mut margin_configuration = package_info.margin.margin_configuration;
    margin_configuration.reverse();
    while let Some(set) = margin_configuration.pop() {
        margin_table.add_row(vec![&set[0], &set[1]]);
    }

    println!("{}", margin_table);

    let mut sub_position_table = Table::new();
    sub_position_table.set_header(vec!["Sub Position"]);
    let mut sub_position_profit_ratio = package_info.margin.sub_position_profit_ratio;
    sub_position_profit_ratio.reverse();
    while let Some(set) = sub_position_profit_ratio.pop() {
        sub_position_table.add_row(vec![&set]);
    }

    println!("{}", sub_position_table);
}

pub fn display_bots(bots: Vec<crate::data::result::Bot>) {
    let mut table = Table::new();
    table.set_header(vec![
        "Title", "Pair", "Exchange", "Strategy", "Cycle", "Status",
    ]);

    for bot in bots {
        table.add_row(vec![
            bot.title,
            bot.pair,
            bot.exchange,
            bot.strategy,
            bot.cycle,
            bot.status,
        ]);
    }

    println!("{}", table);
}

pub fn display_api(api: crate::data::args::ApiKey) {
    let mut table = Table::new();
    table
        .set_header(vec!["Platform", "Api Key", "Secret Key"])
        .add_row(vec![api.platform, api.api_key, api.secret_key]);

    println!("{}", table);
}

use comfy_table::Table;

use crate::model;

pub fn display_bot(bot: model::result::Bot) {
    let mut table = Table::new();
    table
        .set_header(vec!["Name", "Value"])
        .add_row(vec!["Title", &bot.title])
        .add_row(vec!["Pair", &bot.pair])
        .add_row(vec!["Base", &bot.base])
        .add_row(vec!["Quote", &bot.quote])
        .add_row(vec!["Platform", &bot.platform])
        .add_row(vec!["Strategy", &bot.strategy])
        .add_row(vec!["Cycle", &bot.parameters.cycle])
        .add_row(vec![
            "First Buy In",
            &format!("{}", bot.parameters.first_buy_in),
        ])
        .add_row(vec!["Entry", &format!("{:?}", bot.parameters.entry)])
        .add_row(vec![
            "Take Profit",
            &format!("{:?}", bot.parameters.take_profit),
        ]);

    println!("{}", table);

    let mut margin_table = Table::new();
    margin_table.set_header(vec!["Margin"]);
    let mut margin_configuration = bot.margin.margin_configuration;
    margin_configuration.reverse();
    while let Some(set) = margin_configuration.pop() {
        margin_table.add_row(vec![&format!("{:?}", &set)]);
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

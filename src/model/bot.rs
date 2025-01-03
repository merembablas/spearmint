use super::result;
use rusqlite::{params, Connection, Result};

pub fn save(config: result::Bot) -> result::Bot {
    let conn = Connection::open(super::DB_PATH).unwrap();

    let mut stmt = conn
        .prepare("SELECT id FROM bots WHERE pair=:pair AND platform=:platform LIMIT 1")
        .unwrap();
    let pair = &config.pair;
    let platform = &config.platform;
    let bots: Vec<Result<u64>> = stmt
        .query_map([pair, platform], |row| Ok(row.get(0)?))
        .unwrap()
        .collect();

    if bots.iter().count() != 0 {
        let bot = bots.into_iter().next().unwrap();
        conn.execute(
            "UPDATE bots SET
                title=?1,
                pair=?2,
                base=?3,
                quote=?4,
                platform=?5,
                strategy=?6,
                cycle=?7,
                first_buy_in=?8,
                entry=?9,
                take_profit=?10,
                margin=?11
            WHERE id=?12",
            params![
                config.title,
                config.pair,
                config.base,
                config.quote,
                config.platform,
                config.strategy,
                config.parameters.cycle,
                config.parameters.first_buy_in,
                serde_json::to_string(&config.parameters.entry).unwrap(),
                serde_json::to_string(&config.parameters.take_profit).unwrap(),
                serde_json::to_string(&config.margin.margin_configuration).unwrap(),
                bot.unwrap()
            ],
        )
        .unwrap();
    } else {
        conn.execute(
            "INSERT INTO bots (
                title,
                pair,
                base,
                quote,
                platform,
                strategy,
                cycle,
                first_buy_in,
                entry,
                take_profit,
                margin,
                status
            ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, 'PAUSED')",
            params![
                config.title,
                config.pair,
                config.base,
                config.quote,
                config.platform,
                config.strategy,
                config.parameters.cycle,
                config.parameters.first_buy_in,
                serde_json::to_string(&config.parameters.entry).unwrap(),
                serde_json::to_string(&config.parameters.take_profit).unwrap(),
                serde_json::to_string(&config.margin.margin_configuration).unwrap()
            ],
        )
        .unwrap();
    }

    config
}

pub fn get(name: &str) -> Result<result::Bot> {
    let conn = Connection::open(super::DB_PATH).unwrap();
    let mut stmt = conn
        .prepare("SELECT * FROM bots WHERE title=:name LIMIT 1")
        .unwrap();
    let mut bots: Vec<Result<result::Bot>> = stmt
        .query_map([name], |row| {
            let entry: String = row.get(9)?;
            let take_profit: String = row.get(10)?;
            let margin_configuration: String = row.get(11)?;

            Ok(result::Bot {
                title: row.get(1)?,
                pair: row.get(2)?,
                base: row.get(3)?,
                quote: row.get(4)?,
                platform: row.get(5)?,
                strategy: row.get(6)?,
                parameters: result::Parameters {
                    cycle: row.get(7)?,
                    first_buy_in: row.get(8)?,
                    entry: serde_json::from_str(&entry).unwrap(),
                    take_profit: serde_json::from_str(&take_profit).unwrap(),
                },
                margin: result::Margin {
                    margin_configuration: serde_json::from_str(&margin_configuration).unwrap(),
                },
                status: row.get(12)?,
            })
        })
        .unwrap()
        .collect();

    bots.remove(0)
}

pub fn all() -> Result<Vec<result::Bot>> {
    let conn = Connection::open(super::DB_PATH).unwrap();
    let mut stmt = conn.prepare("SELECT * FROM bots")?;
    let bots = stmt.query_map([], |row| {
        let entry: String = row.get(9)?;
        let take_profit: String = row.get(10)?;
        let margin_configuration: String = row.get(11)?;

        Ok(result::Bot {
            title: row.get(1)?,
            pair: row.get(2)?,
            base: row.get(3)?,
            quote: row.get(4)?,
            platform: row.get(5)?,
            strategy: row.get(6)?,
            parameters: result::Parameters {
                cycle: row.get(7)?,
                first_buy_in: row.get(8)?,
                entry: serde_json::from_str(&entry).unwrap(),
                take_profit: serde_json::from_str(&take_profit).unwrap(),
            },
            margin: result::Margin {
                margin_configuration: serde_json::from_str(&margin_configuration).unwrap(),
            },
            status: row.get(12)?,
        })
    })?;

    let mut result: Vec<result::Bot> = Vec::new();
    for bot in bots {
        result.push(bot.unwrap());
    }

    Ok(result)
}

#[allow(dead_code)]
pub fn active() -> Result<Vec<result::Bot>> {
    let conn = Connection::open(super::DB_PATH).unwrap();
    let mut stmt = conn.prepare("SELECT * FROM bots WHERE status='ACTIVE'")?;
    let bots = stmt.query_map([], |row| {
        let entry: String = row.get(9)?;
        let take_profit: String = row.get(10)?;
        let margin_configuration: String = row.get(11)?;

        Ok(result::Bot {
            title: row.get(1)?,
            pair: row.get(2)?,
            base: row.get(3)?,
            quote: row.get(4)?,
            platform: row.get(5)?,
            strategy: row.get(6)?,
            parameters: result::Parameters {
                cycle: row.get(7)?,
                first_buy_in: row.get(8)?,
                entry: serde_json::from_str(&entry).unwrap(),
                take_profit: serde_json::from_str(&take_profit).unwrap(),
            },
            margin: result::Margin {
                margin_configuration: serde_json::from_str(&margin_configuration).unwrap(),
            },
            status: row.get(12)?,
        })
    })?;

    let mut result: Vec<result::Bot> = Vec::new();
    for bot in bots {
        result.push(bot.unwrap());
    }

    Ok(result)
}

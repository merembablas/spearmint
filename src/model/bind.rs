use super::{args, result};
use rusqlite::{params, Connection, Result};

pub fn save(api: args::ApiCredential) -> result::ApiCredential {
    let conn = Connection::open(super::DB_PATH).unwrap();

    let mut stmt = conn
        .prepare("SELECT * FROM bindings WHERE platform=:platform LIMIT 1")
        .unwrap();

    let apis: Vec<Result<result::ApiCredential>> = stmt
        .query_map([&api.platform], |row| {
            Ok(result::ApiCredential {
                api: row.get(1)?,
                secret: row.get(2)?,
                platform: row.get(3)?,
            })
        })
        .unwrap()
        .collect();

    if apis.iter().count() != 0 {
        conn.execute(
            "UPDATE bindings SET
                api_key=?1,
                secret_key=?2
            WHERE platform=?3",
            params![api.api_key, api.secret_key, api.platform],
        )
        .unwrap();
    } else {
        conn.execute(
            "INSERT INTO bindings (
                api_key,
                secret_key,
                platform
            ) VALUES (?1, ?2, ?3)",
            params![api.api_key, api.secret_key, api.platform],
        )
        .unwrap();
    }

    result::ApiCredential {
        api: api.api_key,
        secret: api.secret_key,
        platform: api.platform,
    }
}

pub fn get(platform: &str) -> result::ApiCredential {
    let conn = Connection::open(super::DB_PATH).unwrap();
    let mut stmt = conn
        .prepare(
            "SELECT api_key, secret_key, platform FROM bindings WHERE platform=:platform LIMIT 1",
        )
        .unwrap();
    let mut bindings: Vec<Result<result::ApiCredential>> = stmt
        .query_map([platform], |row| {
            Ok(result::ApiCredential {
                api: row.get(0)?,
                secret: row.get(1)?,
                platform: row.get(2)?,
            })
        })
        .unwrap()
        .collect();

    if bindings.len() > 0 {
        bindings.remove(0).unwrap()
    } else {
        Default::default()
    }
}

use crate::connector::binance;
use crate::data::{bind, result};

#[allow(dead_code)]
pub fn ticks(bots: Vec<result::Bot>) {
    let api_credential = bind::get("binance");
    let account = binance::Account {
        api_key: api_credential.api,
        api_secret: api_credential.secret,
    };

    // create handler and duration
    binance::ticks(account, bots);
}

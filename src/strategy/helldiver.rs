use super::calculate_percent_change;
use crate::model::{result, BotCommand, Session, Strategy};

#[derive(Debug)]
pub struct HellDiverStrategy {
    pub first_buy_in: f64,
    pub entry: result::OpenCriteria,
    pub take_profit: result::CloseCriteria,
    pub margin_configuration: Vec<result::OpenCriteria>,
}

impl Strategy for HellDiverStrategy {
    fn run(&self, price: f64, session: Session) -> BotCommand {
        let avg_percent_change = calculate_percent_change(session.avg_price, price);
        let top_percent_change = calculate_percent_change(session.top_price, price);
        let bottom_percent_change = calculate_percent_change(session.bottom_price, price);
        let mfi_bottom_change = session.mfi - session.bottom_mfi;

        if session.status == "OPEN" {
            let margin_len = self.margin_configuration.len() as u64;

            if self.is_sell_signal(top_percent_change, avg_percent_change) {
                return BotCommand::Sell();
            }

            if session.margin_position < margin_len
                && self.is_avg_buy_signal(
                    avg_percent_change,
                    bottom_percent_change,
                    session.margin_position as usize,
                )
                && self.is_mfi_approved(
                    session.mfi,
                    mfi_bottom_change,
                    &session.mfi_dir,
                    session.margin_position as usize,
                )
            {
                return BotCommand::Buy(
                    self.margin_configuration[session.margin_position as usize].amount_ratio
                        * self.first_buy_in,
                );
            }
        } else if session.status == "WAIT" {
            if self.is_entry_signal(top_percent_change, bottom_percent_change)
                && self.is_entry_mfi_approved(session.mfi, mfi_bottom_change, &session.mfi_dir)
            {
                return BotCommand::Entry(self.first_buy_in);
            }
        }

        BotCommand::Pause()
    }

    fn is_entry_signal(&self, top_percent_change: f64, bottom_percent_change: f64) -> bool {
        top_percent_change < self.entry.price_change_below
            && bottom_percent_change > self.entry.price_callback
    }

    fn is_sell_signal(&self, top_percent_change: f64, avg_percent_change: f64) -> bool {
        avg_percent_change > self.take_profit.price_change_above
            && top_percent_change < self.take_profit.price_callback
    }

    fn is_avg_buy_signal(
        &self,
        avg_percent_change: f64,
        bottom_percent_change: f64,
        margin_position: usize,
    ) -> bool {
        avg_percent_change < self.margin_configuration[margin_position].price_change_below
            && bottom_percent_change > self.margin_configuration[margin_position].price_callback
    }

    fn is_entry_mfi_approved(&self, mfi: f64, mfi_bottom_change: f64, mfi_dir: &str) -> bool {
        mfi < self.entry.mfi_below && mfi_bottom_change > self.entry.mfi_callback && mfi_dir == "UP"
    }

    fn is_mfi_approved(
        &self,
        mfi: f64,
        mfi_bottom_change: f64,
        mfi_dir: &str,
        margin_position: usize,
    ) -> bool {
        mfi < self.margin_configuration[margin_position].mfi_below
            && mfi_bottom_change > self.margin_configuration[margin_position].mfi_callback
            && mfi_dir == "UP"
    }
}

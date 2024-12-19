use super::calculate_percent_change;
use crate::model::{BotCommand, Session, Strategy};

#[derive(Debug)]
pub struct HellDiverStrategy {
    pub first_buy_in: f64,
    pub first_entry: Vec<f64>,
    pub take_profit_ratio: f64,
    pub earning_callback: f64,
    pub margin_configuration: Vec<Vec<f64>>,
}

impl Strategy for HellDiverStrategy {
    fn run(&self, price: f64, session: Session) -> BotCommand {
        let avg_percent_change = calculate_percent_change(session.avg_price, price);
        let top_percent_change = calculate_percent_change(session.top_price, price);
        let bottom_percent_change = calculate_percent_change(session.bottom_price, price);

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
            {
                return BotCommand::Buy(
                    self.margin_configuration[session.margin_position as usize][2]
                        * self.first_buy_in,
                );
            }
        } else if session.status == "WAIT" {
            if self.is_entry_signal(top_percent_change, bottom_percent_change) {
                return BotCommand::Entry(self.first_buy_in);
            }
        }

        BotCommand::Pause()
    }

    fn is_entry_signal(&self, top_percent_change: f64, bottom_percent_change: f64) -> bool {
        top_percent_change < self.first_entry[0] && bottom_percent_change > self.first_entry[1]
    }

    fn is_sell_signal(&self, top_percent_change: f64, avg_percent_change: f64) -> bool {
        avg_percent_change > self.take_profit_ratio && top_percent_change < self.earning_callback
    }

    fn is_avg_buy_signal(
        &self,
        avg_percent_change: f64,
        bottom_percent_change: f64,
        margin_position: usize,
    ) -> bool {
        avg_percent_change < self.margin_configuration[margin_position][0]
            && bottom_percent_change > self.margin_configuration[margin_position][1]
    }
}

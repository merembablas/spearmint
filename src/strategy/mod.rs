pub mod helldiver;

pub fn calculate_percent_change(old_value: f64, new_value: f64) -> f64 {
    if old_value == 0.0 {
        return 0.0;
    }

    ((new_value - old_value) / old_value) * 100.0
}

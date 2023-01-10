use std::time::Duration;

pub trait TimeFormatter {
    fn format(&self) -> String;
}

impl TimeFormatter for Duration {
    fn format(&self) -> String {
        let seconds = self.as_secs();
        format!("{}:{}", seconds / 60, seconds % 60)
    }
}

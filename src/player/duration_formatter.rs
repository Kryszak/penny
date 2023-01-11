use std::time::Duration;

pub enum DurationFormat {
    MmSs,
}

pub trait DurationFormatter {
    fn format(&self, format: DurationFormat) -> String;
}

impl DurationFormatter for Duration {
    fn format(&self, format: DurationFormat) -> String {
        match format {
            DurationFormat::MmSs => {
                let seconds = self.as_secs();
                format!("{:0>2}:{:0>2}", seconds / 60, seconds % 60)
            }
        }
    }
}

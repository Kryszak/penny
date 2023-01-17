use std::time::Duration;

/// Formats available for [Duration](std::time::Duration) string display
pub enum DurationFormat {
    /// Formats duration as "MM:SS" with leading zeros
    /// Example:
    /// ```
    /// println!(Duration::from_secs(75).format(DurationFormat::MmSs));
    /// "01:15"
    /// ```
    MmSs,
}

pub trait DurationFormatter {
    /// Returns string representing duration in given [DurationFormat](DurationFormat)
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

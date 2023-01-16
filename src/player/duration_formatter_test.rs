#[cfg(test)]
mod tests {
    use std::time::Duration;

    use crate::player::duration_formatter::{DurationFormat, DurationFormatter};

    #[test]
    fn should_format_duration() {
        // given
        let duration = Duration::from_secs(75);

        // when
        let result = duration.format(DurationFormat::MmSs);

        // then
        assert_eq!(result, "01:15");
    }
}

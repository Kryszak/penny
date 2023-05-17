pub struct BarChartData {
    pub audio_spectrum: Vec<(&'static str, u64)>,
    pub audio_spectrum_band_count: usize,
}

impl BarChartData {
    pub fn new(bar_count: usize) -> Self {
        BarChartData {
            audio_spectrum: vec![],
            audio_spectrum_band_count: bar_count,
        }
    }

    /// Process raw spectrum from player for display
    /// Takes first half of frequencies, to display only audible range of frequencies (<20kHz)
    /// For display purposes, we divide that range into bands and sum values
    /// First two bins are skipped, as they always have high values
    pub fn update_spectrum(&mut self, raw_spectrum: Vec<u64>) {
        let usable_spectrum = &raw_spectrum[0..raw_spectrum.len() / 2];
        let band_count = self.audio_spectrum_band_count;
        if usable_spectrum.len() > band_count {
            let band_width = usable_spectrum.len() / band_count;
            self.audio_spectrum = usable_spectrum
                .chunks(band_width)
                .skip(2)
                .map(|chunk| ("", chunk.iter().copied().reduce(|a, b| a + b).unwrap_or(0)))
                .collect();
        } else {
            self.audio_spectrum = vec![];
        }
    }
}

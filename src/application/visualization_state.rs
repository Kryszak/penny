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

pub struct ChartData {
    pub audio_spectrum: Vec<(f64, f64)>,
    pub audio_spectrum_point_count: usize,
    pub max_value: f64,
}

impl ChartData {
    pub fn new(point_count: usize) -> Self {
        ChartData {
            audio_spectrum: vec![],
            audio_spectrum_point_count: point_count,
            max_value: 5.0,
        }
    }

    pub fn update_spectrum(&mut self, raw_spectrum: Vec<f64>) {
        let usable_spectrum = &raw_spectrum[0..raw_spectrum.len() / 2];
        let band_count = self.audio_spectrum_point_count;
        if usable_spectrum.len() > band_count {
            let band_width = usable_spectrum.len() / band_count;
            let banded_data: Vec<(f64, f64)> = usable_spectrum
                .chunks(band_width)
                .skip(2)
                .enumerate()
                .map(|(number, chunk)| {
                    (
                        number as f64,
                        chunk.iter().copied().reduce(|a, b| a + b).unwrap_or(0.0),
                    )
                })
                .collect();
            let max_value = banded_data.iter().map(|(_, y)| *y).fold(0.0, f64::max);
            self.audio_spectrum = banded_data
                .iter()
                .map(|(index, val)| (*index, self.scale_value(*val, max_value)))
                .collect();
        } else {
            self.audio_spectrum = vec![];
        }
    }

    fn scale_value(&self, value: f64, maximum: f64) -> f64 {
        if value > 0.0 {
            (value / maximum) * self.max_value
        } else {
            0.0
        }
    }
}

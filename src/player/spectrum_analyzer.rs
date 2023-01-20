use rustfft::{num_complex::Complex, FftPlanner};

pub struct SpectrumAnalyzer {
    planner: FftPlanner<f32>,
}

impl SpectrumAnalyzer {
    pub fn new() -> Self {
        SpectrumAnalyzer {
            planner: FftPlanner::new(),
        }
    }

    pub fn analyze(&mut self, data: &[i16]) -> Vec<f32> {
        let mut buffer = SpectrumAnalyzer::prepare_data(data);
        let fft = self.planner.plan_fft_forward(buffer.len());
        fft.process(&mut buffer);
        buffer.into_iter().map(|c| c.re).collect::<Vec<f32>>()
    }

    fn prepare_data(data: &[i16]) -> Vec<Complex<f32>> {
        // TODO maybe passing channel count instead of hardcoded 2 would be better?
        data.chunks(2)
            .map(|chunk| Complex::new((chunk[0] as f32 + chunk[1] as f32) / 2.0, 0.0))
            .collect()
    }
}

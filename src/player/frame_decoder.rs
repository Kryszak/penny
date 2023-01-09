use minimp3::Frame;
use rodio::Source;
use std::time::Duration;

pub struct FrameDecoder {
    frame: Frame,
    current_frame_offset: usize,
}

impl FrameDecoder {
    pub fn new(frame: Frame) -> Self {
        FrameDecoder {
            frame,
            current_frame_offset: 0,
        }
    }
}

impl Source for FrameDecoder {
    fn current_frame_len(&self) -> Option<usize> {
        Some(self.frame.data.len())
    }

    fn channels(&self) -> u16 {
        self.frame.channels as _
    }

    fn sample_rate(&self) -> u32 {
        self.frame.sample_rate as _
    }

    fn total_duration(&self) -> Option<Duration> {
        None
    }
}

impl Iterator for FrameDecoder {
    type Item = i16;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_frame_offset == self.frame.data.len() {
            return None;
        }

        let v = self.frame.data[self.current_frame_offset];
        self.current_frame_offset += 1;

        Some(v)
    }
}

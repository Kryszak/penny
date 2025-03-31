use minimp3_fixed as minimp3;
use minimp3::Frame;
use rodio::Source;
use std::time::Duration;

/// Implementation of Rodio's [Source](rodio::Source) trait
/// for feeding [Sink](rodio::Sink) one frame at a time.
/// This allows to perform other operations on audio frame
/// before playing it like FFT analysis.
///
/// This implementation is based on Rodio's [Mp3Decoder](https://github.com/RustAudio/rodio/blob/master/src/decoder/mp3.rs)
/// with change allowing to construct decoder for single frame instead of providing file to
/// [Decoder](rodio::Decoder)
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

pub trait FrameDuration {
    fn get_duration(&self) -> Duration;
}

impl FrameDuration for Frame {
    fn get_duration(&self) -> Duration {
        let frame_duration =
            (self.data.len() as f64 / self.channels as f64) / self.sample_rate as f64;
        Duration::from_millis((frame_duration * 1024.0) as u64)
    }
}

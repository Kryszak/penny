//! Module handling playback of mp3 files and extracting mp3
//! information for display
mod duration_formatter;
mod frame_decoder;
pub mod metadata;
pub mod mp3_player;
mod spectrum_analyzer;

use frame_decoder::FrameDecoder;
pub use metadata::MetadataReader;
pub use mp3_player::Mp3Player;

#[cfg(test)]
mod duration_formatter_test;
#[cfg(test)]
mod metadata_test;

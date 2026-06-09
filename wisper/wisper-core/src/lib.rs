pub mod audio;
pub mod error;
pub mod transcribe;

pub use error::WisperError;
pub use transcribe::{transcribe_file, TranscriptSegment};

/// Default model filename (user downloads to app data dir on first run).
pub const DEFAULT_MODEL_FILENAME: &str = "ggml-large-v3-turbo.bin";

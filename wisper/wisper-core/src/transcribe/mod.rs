use std::path::Path;

use whisper_rs::{FullParams, SamplingStrategy, WhisperContext, WhisperContextParameters};

use crate::audio;
use crate::compute::{validate_backend, ComputeBackend};
use crate::error::WisperError;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TranscriptSegment {
    pub start_ms: i64,
    pub end_ms: i64,
    pub text: String,
}

/// Transcribe a local audio file using whisper.cpp (fully offline).
pub fn transcribe_file(
    model_path: &Path,
    audio_path: &Path,
    backend: ComputeBackend,
) -> Result<Vec<TranscriptSegment>, WisperError> {
    validate_backend(backend)?;
    if !model_path.exists() {
        return Err(WisperError::ModelNotFound {
            path: model_path.to_path_buf(),
        });
    }

    if !audio_path.exists() {
        return Err(WisperError::AudioNotFound(audio_path.display().to_string()));
    }

    let pcm = audio::load_audio_pcm(audio_path)?;

    let mut ctx_params = WhisperContextParameters::default();
    ctx_params.use_gpu = backend == ComputeBackend::Gpu;

    let ctx = WhisperContext::new_with_params(
        model_path.to_str().ok_or_else(|| WisperError::WhisperInit("invalid model path".into()))?,
        ctx_params,
    )
    .map_err(|e| WisperError::WhisperInit(e.to_string()))?;

    let mut state = ctx
        .create_state()
        .map_err(|e| WisperError::Transcription(e.to_string()))?;

    let mut params = FullParams::new(SamplingStrategy::Greedy { best_of: 1 });
    params.set_translate(false);
    params.set_language(Some("en"));
    params.set_print_special(false);
    params.set_print_progress(false);
    params.set_print_realtime(false);
    params.set_print_timestamps(true);
    params.set_token_timestamps(true);

    state
        .full(params, &pcm)
        .map_err(|e| WisperError::Transcription(e.to_string()))?;

    let n = state.full_n_segments();
    let mut segments = Vec::with_capacity(n as usize);

    for i in 0..n {
        let segment = state
            .get_segment(i)
            .ok_or_else(|| WisperError::Transcription(format!("missing segment {i}")))?;

        let text = segment
            .to_str_lossy()
            .map_err(|e| WisperError::Transcription(e.to_string()))?
            .trim()
            .to_string();

        if text.is_empty() {
            continue;
        }

        let start_ms = segment.start_timestamp() * 10;
        let end_ms = segment.end_timestamp() * 10;

        segments.push(TranscriptSegment {
            start_ms,
            end_ms,
            text,
        });
    }

    Ok(segments)
}

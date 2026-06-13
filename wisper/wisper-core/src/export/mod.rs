use crate::transcribe::TranscriptSegment;

fn format_timestamp(ms: i64) -> String {
    let total_seconds = ms.max(0) / 1000;
    let minutes = total_seconds / 60;
    let seconds = total_seconds % 60;
    format!("{minutes}:{seconds:02}")
}

/// Plain-text export: one block per segment with `[start – end]` prefix.
pub fn format_transcript_txt(segments: &[TranscriptSegment]) -> String {
    segments
        .iter()
        .map(|seg| {
            format!(
                "[{} – {}] {}",
                format_timestamp(seg.start_ms),
                format_timestamp(seg.end_ms),
                seg.text.trim()
            )
        })
        .collect::<Vec<_>>()
        .join("\n\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn format_transcript_txt_includes_timestamps() {
        let text = format_transcript_txt(&[TranscriptSegment {
            start_ms: 0,
            end_ms: 1500,
            text: "Hello".into(),
        }]);
        assert!(text.contains("[0:00 – 0:01] Hello"));
    }
}

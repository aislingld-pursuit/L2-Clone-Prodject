use crate::transcribe::TranscriptSegment;

fn format_timestamp_hms(ms: i64) -> (u64, u64, u64, u64) {
    let ms = ms.max(0) as u64;
    let hours = ms / 3_600_000;
    let minutes = (ms % 3_600_000) / 60_000;
    let seconds = (ms % 60_000) / 1000;
    let millis = ms % 1000;
    (hours, minutes, seconds, millis)
}

fn format_srt_timestamp(ms: i64) -> String {
    let (hours, minutes, seconds, millis) = format_timestamp_hms(ms);
    format!("{hours:02}:{minutes:02}:{seconds:02},{millis:03}")
}

fn format_vtt_timestamp(ms: i64) -> String {
    let (hours, minutes, seconds, millis) = format_timestamp_hms(ms);
    format!("{hours:02}:{minutes:02}:{seconds:02}.{millis:03}")
}

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
        .filter_map(|seg| {
            let text = seg.text.trim();
            if text.is_empty() {
                return None;
            }
            Some(format!(
                "[{} – {}] {}",
                format_timestamp(seg.start_ms),
                format_timestamp(seg.end_ms),
                text
            ))
        })
        .collect::<Vec<_>>()
        .join("\n\n")
}

/// SubRip (.srt): numbered cues with `HH:MM:SS,mmm` timestamps.
pub fn format_transcript_srt(segments: &[TranscriptSegment]) -> String {
    let mut index = 1usize;
    let mut blocks = Vec::new();
    for seg in segments {
        let text = seg.text.trim();
        if text.is_empty() {
            continue;
        }
        blocks.push(format!(
            "{index}\n{} --> {}\n{text}",
            format_srt_timestamp(seg.start_ms),
            format_srt_timestamp(seg.end_ms),
        ));
        index += 1;
    }
    blocks.join("\n\n")
}

/// WebVTT (.vtt): header plus `HH:MM:SS.mmm` cues.
pub fn format_transcript_vtt(segments: &[TranscriptSegment]) -> String {
    let mut cues: Vec<String> = Vec::new();
    for seg in segments {
        let text = seg.text.trim();
        if text.is_empty() {
            continue;
        }
        cues.push(format!(
            "{} --> {}\n{text}",
            format_vtt_timestamp(seg.start_ms),
            format_vtt_timestamp(seg.end_ms),
        ));
    }
    if cues.is_empty() {
        return String::new();
    }
    let mut lines = vec!["WEBVTT".to_string(), String::new()];
    for cue in cues {
        lines.push(cue);
        lines.push(String::new());
    }
    while lines.last().is_some_and(|line| line.is_empty()) {
        lines.pop();
    }
    lines.join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_segment() -> TranscriptSegment {
        TranscriptSegment {
            start_ms: 0,
            end_ms: 1500,
            text: "Hello".into(),
        }
    }

    #[test]
    fn format_transcript_txt_includes_timestamps() {
        let text = format_transcript_txt(&[sample_segment()]);
        assert!(text.contains("[0:00 – 0:01] Hello"));
    }

    #[test]
    fn format_transcript_srt_standard_cue() {
        let text = format_transcript_srt(&[sample_segment()]);
        assert!(text.starts_with("1\n"));
        assert!(text.contains("00:00:00,000 --> 00:00:01,500"));
        assert!(text.contains("Hello"));
    }

    #[test]
    fn format_transcript_vtt_includes_header() {
        let text = format_transcript_vtt(&[sample_segment()]);
        assert!(text.starts_with("WEBVTT\n"));
        assert!(text.contains("00:00:00.000 --> 00:00:01.500"));
        assert!(text.contains("Hello"));
    }

    #[test]
    fn export_skips_empty_segment_text() {
        let empty = TranscriptSegment {
            start_ms: 2000,
            end_ms: 3000,
            text: "   ".into(),
        };
        assert!(format_transcript_srt(&[empty.clone()]).is_empty());
        assert!(format_transcript_vtt(&[empty]).is_empty());
    }

    #[test]
    fn srt_supports_hour_long_timestamps() {
        let seg = TranscriptSegment {
            start_ms: 3_661_500,
            end_ms: 3_662_000,
            text: "Late".into(),
        };
        let text = format_transcript_srt(&[seg]);
        assert!(text.contains("01:01:01,500 --> 01:01:02,000"));
    }
}

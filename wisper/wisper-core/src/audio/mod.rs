use std::path::Path;

use symphonia::core::audio::{AudioBufferRef, Signal};
use symphonia::core::codecs::{DecoderOptions, CODEC_TYPE_NULL};
use symphonia::core::errors::Error as SymphoniaError;
use symphonia::core::formats::FormatOptions;
use symphonia::core::io::MediaSourceStream;
use symphonia::core::meta::MetadataOptions;
use symphonia::core::probe::Hint;

use crate::error::WisperError;

const TARGET_SAMPLE_RATE: u32 = 16_000;

/// Load any supported audio file and return mono f32 PCM at 16 kHz (Whisper input).
pub fn load_audio_pcm(path: &Path) -> Result<Vec<f32>, WisperError> {
    if path.extension().and_then(|e| e.to_str()).is_some_and(|e| e.eq_ignore_ascii_case("wav")) {
        return load_wav(path);
    }

    decode_with_symphonia(path)
}

fn load_wav(path: &Path) -> Result<Vec<f32>, WisperError> {
    let reader = hound::WavReader::open(path).map_err(|e| WisperError::AudioDecode(e.to_string()))?;
    let spec = reader.spec();

    let samples: Vec<f32> = match spec.sample_format {
        hound::SampleFormat::Float => reader
            .into_samples::<f32>()
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| WisperError::AudioDecode(e.to_string()))?,
        hound::SampleFormat::Int => reader
            .into_samples::<i32>()
            .map_err(|e| WisperError::AudioDecode(e.to_string()))?
            .map(|s| s.map_err(|e| WisperError::AudioDecode(e.to_string())))
            .collect::<Result<Vec<_>, _>>()?
            .into_iter()
            .map(|s| s as f32 / i32::MAX as f32)
            .collect(),
    };

    let mono = downmix_to_mono(&samples, spec.channels as usize);
    Ok(resample_linear(&mono, spec.sample_rate, TARGET_SAMPLE_RATE))
}

fn decode_with_symphonia(path: &Path) -> Result<Vec<f32>, WisperError> {
    let src = std::fs::File::open(path).map_err(|_| WisperError::AudioNotFound(path.display().to_string()))?;
    let mss = MediaSourceStream::new(Box::new(src), Default::default());

    let mut hint = Hint::new();
    if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
        hint.with_extension(ext);
    }

    let probed = symphonia::default::get_probe()
        .format(
            &hint,
            mss,
            &FormatOptions::default(),
            &MetadataOptions::default(),
        )
        .map_err(|e| WisperError::AudioDecode(e.to_string()))?;

    let mut format = probed.format;
    let track = format
        .default_track()
        .ok_or_else(|| WisperError::AudioDecode("no default audio track".into()))?;

    let mut decoder = symphonia::default::get_codecs()
        .make(&track.codec_params, &DecoderOptions::default())
        .map_err(|e| WisperError::AudioDecode(e.to_string()))?;

    let sample_rate = track
        .codec_params
        .sample_rate
        .unwrap_or(TARGET_SAMPLE_RATE);
    let channels = track.codec_params.channels.map(|c| c.count()).unwrap_or(1);

    let mut pcm = Vec::new();

    loop {
        let packet = match format.next_packet() {
            Ok(packet) => packet,
            Err(SymphoniaError::ResetRequired) => continue,
            Err(SymphoniaError::IoError(_)) if pcm.is_empty() => {
                return Err(WisperError::AudioDecode("empty audio file".into()));
            }
            Err(SymphoniaError::IoError(_)) => break,
            Err(e) => return Err(WisperError::AudioDecode(e.to_string())),
        };

        if packet.track_id() != track.id {
            continue;
        }

        if track.codec_params.codec != CODEC_TYPE_NULL {
            match decoder.decode(&packet) {
                Ok(decoded) => append_decoded(&mut pcm, decoded),
                Err(SymphoniaError::IoError(_)) => break,
                Err(SymphoniaError::DecodeError(_)) => continue,
                Err(e) => return Err(WisperError::AudioDecode(e.to_string())),
            }
        }
    }

    if pcm.is_empty() {
        return Err(WisperError::AudioDecode("no audio samples decoded".into()));
    }

    let mono = downmix_to_mono(&pcm, channels);
    Ok(resample_linear(&mono, sample_rate, TARGET_SAMPLE_RATE))
}

fn append_decoded(pcm: &mut Vec<f32>, decoded: AudioBufferRef<'_>) {
    match decoded {
        AudioBufferRef::F32(buf) => {
            let channels = buf.spec().channels.count();
            for frame in 0..buf.frames() {
                let mut sum = 0.0f32;
                for ch in 0..channels {
                    sum += buf.chan(ch)[frame];
                }
                pcm.push(sum / channels as f32);
            }
        }
        AudioBufferRef::S16(buf) => {
            let channels = buf.spec().channels.count();
            for frame in 0..buf.frames() {
                let mut sum = 0.0f32;
                for ch in 0..channels {
                    sum += buf.chan(ch)[frame] as f32 / i16::MAX as f32;
                }
                pcm.push(sum / channels as f32);
            }
        }
        AudioBufferRef::S32(buf) => {
            let channels = buf.spec().channels.count();
            for frame in 0..buf.frames() {
                let mut sum = 0.0f32;
                for ch in 0..channels {
                    sum += buf.chan(ch)[frame] as f32 / i32::MAX as f32;
                }
                pcm.push(sum / channels as f32);
            }
        }
        _ => {}
    }
}

fn downmix_to_mono(samples: &[f32], channels: usize) -> Vec<f32> {
    if channels <= 1 {
        return samples.to_vec();
    }

    samples
        .chunks(channels)
        .map(|frame| frame.iter().sum::<f32>() / channels as f32)
        .collect()
}

fn resample_linear(samples: &[f32], from_rate: u32, to_rate: u32) -> Vec<f32> {
    if from_rate == to_rate || samples.is_empty() {
        return samples.to_vec();
    }

    let ratio = from_rate as f64 / to_rate as f64;
    let out_len = ((samples.len() as f64) / ratio).ceil() as usize;
    let mut out = Vec::with_capacity(out_len);

    for i in 0..out_len {
        let src_pos = i as f64 * ratio;
        let idx = src_pos.floor() as usize;
        let frac = (src_pos - idx as f64) as f32;
        let a = samples.get(idx).copied().unwrap_or(0.0);
        let b = samples.get(idx + 1).copied().unwrap_or(a);
        out.push(a + (b - a) * frac);
    }

    out
}

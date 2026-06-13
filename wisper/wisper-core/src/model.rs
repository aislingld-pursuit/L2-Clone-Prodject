use std::path::{Path, PathBuf};

use crate::DEFAULT_MODEL_FILENAME;

#[derive(Debug, Clone, serde::Serialize)]
pub struct ModelStatus {
    pub path: String,
    pub models_dir: String,
    pub ready: bool,
    pub hint: String,
}

/// Resolve the whisper model file under `models_dir`.
/// Prefers `DEFAULT_MODEL_FILENAME`, otherwise uses the only `.bin` present.
pub fn resolve_model_path(models_dir: &Path) -> PathBuf {
    let preferred = models_dir.join(DEFAULT_MODEL_FILENAME);
    if preferred.is_file() {
        return preferred;
    }

    let mut bins: Vec<PathBuf> = std::fs::read_dir(models_dir)
        .into_iter()
        .flatten()
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.path())
        .filter(|path| path.is_file() && path.extension().is_some_and(|ext| ext == "bin"))
        .collect();

    bins.sort();

    if bins.len() == 1 {
        return bins.remove(0);
    }

    preferred
}

/// Report whether a usable GGML model file exists under `models_dir`.
pub fn model_status(models_dir: &Path) -> ModelStatus {
    let path = resolve_model_path(models_dir);
    let ready = path.is_file();
    let hint = if ready {
        "Whisper model is ready.".into()
    } else {
        format!(
            "Download a GGML model into the models folder (e.g. run scripts/download-model.ps1 for ggml-base.en.bin, or place {DEFAULT_MODEL_FILENAME})."
        )
    };

    ModelStatus {
        path: path.to_string_lossy().into_owned(),
        models_dir: models_dir.to_string_lossy().into_owned(),
        ready,
        hint,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn model_status_ready_when_bin_exists() {
        let dir = std::env::temp_dir().join(format!("wisper-model-test-{}", uuid::Uuid::new_v4()));
        fs::create_dir_all(&dir).unwrap();
        let bin = dir.join("ggml-base.en.bin");
        fs::write(&bin, b"fake").unwrap();

        let status = model_status(&dir);
        assert!(status.ready);
        assert_eq!(status.path, bin.to_string_lossy());

        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn model_status_missing_when_no_bin() {
        let dir = std::env::temp_dir().join(format!("wisper-model-test-{}", uuid::Uuid::new_v4()));
        fs::create_dir_all(&dir).unwrap();

        let status = model_status(&dir);
        assert!(!status.ready);

        let _ = fs::remove_dir_all(&dir);
    }
}

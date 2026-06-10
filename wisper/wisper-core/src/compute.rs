use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ComputeBackend {
    Cpu,
    Gpu,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComputeInfo {
    pub gpu_available: bool,
    /// Human-readable backend name when GPU is compiled in (e.g. "Metal", "Vulkan").
    pub gpu_backend: Option<String>,
    pub default_backend: ComputeBackend,
}

pub fn compute_info() -> ComputeInfo {
    let gpu_backend = gpu_backend_name();
    ComputeInfo {
        gpu_available: gpu_backend.is_some(),
        gpu_backend,
        default_backend: ComputeBackend::Cpu,
    }
}

fn gpu_backend_name() -> Option<String> {
    if cfg!(target_os = "macos") {
        Some("Metal".to_string())
    } else if cfg!(all(windows, feature = "gpu-vulkan")) {
        Some("Vulkan".to_string())
    } else if cfg!(all(windows, feature = "gpu-cuda", not(feature = "gpu-vulkan"))) {
        Some("CUDA".to_string())
    } else {
        None
    }
}

pub fn validate_backend(backend: ComputeBackend) -> Result<(), crate::WisperError> {
    if backend == ComputeBackend::Gpu && gpu_backend_name().is_none() {
        return Err(crate::WisperError::Transcription(
            "GPU is not available in this build. On Windows, install the Vulkan SDK, set \
             VULKAN_SDK, and rebuild with --features gpu-vulkan."
                .into(),
        ));
    }
    Ok(())
}

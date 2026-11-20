use serde::{Deserialize, Serialize};

/// Compile-time guard: whisper.cpp links exactly one GPU backend per binary.
#[cfg(all(feature = "gpu-vulkan", feature = "gpu-cuda"))]
compile_error!("Enable only one GPU feature: gpu-vulkan OR gpu-cuda OR gpu-sycl");

#[cfg(all(feature = "gpu-vulkan", feature = "gpu-sycl"))]
compile_error!("Enable only one GPU feature: gpu-vulkan OR gpu-cuda OR gpu-sycl");

#[cfg(all(feature = "gpu-cuda", feature = "gpu-sycl"))]
compile_error!("Enable only one GPU feature: gpu-vulkan OR gpu-cuda OR gpu-sycl");

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ComputeBackend {
    Cpu,
    Gpu,
}

/// Which GPU stack was compiled into this binary (if any).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum GpuBackendKind {
    Metal,
    Cuda,
    Vulkan,
    IntelSycl,
}

impl GpuBackendKind {
    pub fn label(self) -> &'static str {
        match self {
            Self::Metal => "Metal",
            Self::Cuda => "CUDA",
            Self::Vulkan => "Vulkan",
            Self::IntelSycl => "Intel SYCL",
        }
    }

    pub fn hint(self) -> &'static str {
        match self {
            Self::Metal => "Apple Metal (Apple Silicon and Intel Macs). Enabled automatically on macOS builds.",
            Self::Cuda => "NVIDIA CUDA. Rebuild with --features gpu-cuda and the CUDA Toolkit installed.",
            Self::Vulkan => {
                "Vulkan (NVIDIA, AMD, Intel iGPU on Windows/Linux). Rebuild with --features gpu-vulkan."
            }
            Self::IntelSycl => {
                "Intel oneAPI SYCL. Rebuild with --features gpu-sycl and Intel oneAPI setvars loaded."
            }
        }
    }
}

/// Host CPU architecture label for the UI (ggml-cpu runs on Intel, AMD, and ARM).
pub fn cpu_architecture_label() -> &'static str {
    match () {
        _ if cfg!(all(target_arch = "aarch64", target_os = "macos")) => {
            "Apple Silicon (ARM64)"
        }
        _ if cfg!(target_arch = "aarch64") => "ARM64 (aarch64)",
        _ if cfg!(target_arch = "x86_64") => "x86_64 (Intel / AMD)",
        _ if cfg!(target_arch = "x86") => "x86 (32-bit)",
        _ => "Unknown",
    }
}

/// A GPU whisper.cpp can run on.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GpuDevice {
    /// whisper.cpp `gpu_device` index (counts GPUs only, in ggml registry order).
    pub index: i32,
    /// Driver name, e.g. "AMD Radeon(TM) 890M Graphics".
    pub name: String,
    /// Shares system RAM (iGPU) — far lower power than a dedicated card.
    pub integrated: bool,
}

/// GPUs this build can use. The first call initializes the GPU backend
/// (e.g. creates the Vulkan instance), which whisper.cpp would do on first load anyway.
pub fn gpu_devices() -> Vec<GpuDevice> {
    use whisper_rs::whisper_rs_sys as sys;

    if compiled_gpu_backend().is_none() {
        return Vec::new();
    }

    let mut devices = Vec::new();
    // Same filter and order as whisper.cpp's whisper_backend_init_gpu, so `index` lines up.
    unsafe {
        for i in 0..sys::ggml_backend_dev_count() {
            let dev = sys::ggml_backend_dev_get(i);
            let kind = sys::ggml_backend_dev_type(dev);
            let integrated = kind == sys::ggml_backend_dev_type_GGML_BACKEND_DEVICE_TYPE_IGPU;
            if !integrated && kind != sys::ggml_backend_dev_type_GGML_BACKEND_DEVICE_TYPE_GPU {
                continue;
            }
            let name = std::ffi::CStr::from_ptr(sys::ggml_backend_dev_description(dev))
                .to_string_lossy()
                .into_owned();
            devices.push(GpuDevice {
                index: devices.len() as i32,
                name,
                integrated,
            });
        }
    }
    devices
}

/// `Some(true)` when running on battery, `Some(false)` on AC, `None` if unknown.
#[cfg(windows)]
pub fn on_battery() -> Option<bool> {
    #[repr(C)]
    #[derive(Default)]
    struct SystemPowerStatus {
        ac_line_status: u8,
        battery_flag: u8,
        battery_life_percent: u8,
        system_status_flag: u8,
        battery_life_time: u32,
        battery_full_life_time: u32,
    }
    #[link(name = "kernel32")]
    extern "system" {
        fn GetSystemPowerStatus(status: *mut SystemPowerStatus) -> i32;
    }

    let mut status = SystemPowerStatus::default();
    if unsafe { GetSystemPowerStatus(&mut status) } == 0 {
        return None;
    }
    match status.ac_line_status {
        0 => Some(true),
        1 => Some(false),
        _ => None,
    }
}

// ponytail: Windows only; read /sys/class/power_supply/*/online when Linux laptops need Auto.
#[cfg(not(windows))]
pub fn on_battery() -> Option<bool> {
    None
}

/// GPU for "Auto": integrated on battery, dedicated when plugged in or power is unknown.
pub fn auto_gpu_device(devices: &[GpuDevice], on_battery: Option<bool>) -> i32 {
    let want_integrated = on_battery == Some(true);
    devices
        .iter()
        .find(|device| device.integrated == want_integrated)
        .or(devices.first())
        .map_or(0, |device| device.index)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComputeInfo {
    pub gpu_available: bool,
    /// GPUs to choose from; empty on CPU-only builds.
    pub gpu_devices: Vec<GpuDevice>,
    /// Human-readable backend name when GPU is compiled in (e.g. "Metal", "Vulkan").
    pub gpu_backend: Option<String>,
    pub gpu_backend_kind: Option<GpuBackendKind>,
    /// GPU when this build includes a GPU backend; CPU-only builds default to CPU.
    pub default_backend: ComputeBackend,
    /// Host CPU architecture (Intel, AMD, Apple Silicon, and other ARM64 are supported).
    pub cpu_architecture: String,
    /// GPU builds always retry on CPU if inference fails.
    pub supports_cpu_fallback: bool,
}

pub fn compute_info() -> ComputeInfo {
    let kind = compiled_gpu_backend();
    let gpu_backend = kind.map(GpuBackendKind::label).map(str::to_string);
    let gpu_available = gpu_backend.is_some();
    ComputeInfo {
        gpu_available,
        gpu_devices: gpu_devices(),
        gpu_backend,
        gpu_backend_kind: kind,
        default_backend: if gpu_available {
            ComputeBackend::Gpu
        } else {
            ComputeBackend::Cpu
        },
        cpu_architecture: cpu_architecture_label().to_string(),
        supports_cpu_fallback: gpu_available,
    }
}

/// Human-readable OS for About / support.
pub fn platform_os_label() -> &'static str {
    match () {
        _ if cfg!(target_os = "windows") => "Windows",
        _ if cfg!(target_os = "macos") => "macOS",
        _ if cfg!(target_os = "linux") => "Linux",
        _ => "Unknown",
    }
}

/// Release artifact name matching [GPU_BACKENDS.md] matrix.
pub fn release_artifact_label() -> &'static str {
    match () {
        _ if cfg!(target_os = "macos") => "wisper-macos-universal",
        _ if cfg!(all(target_os = "windows", feature = "gpu-cuda")) => "wisper-windows-cuda",
        _ if cfg!(all(target_os = "windows", feature = "gpu-vulkan")) => "wisper-windows-vulkan",
        _ if cfg!(all(target_os = "windows", feature = "gpu-sycl")) => "wisper-windows-sycl",
        _ if cfg!(target_os = "windows") => "wisper-windows-cpu",
        _ if cfg!(all(target_os = "linux", feature = "gpu-cuda")) => "wisper-linux-cuda",
        _ if cfg!(all(target_os = "linux", feature = "gpu-vulkan")) => "wisper-linux-vulkan",
        _ if cfg!(all(target_os = "linux", feature = "gpu-sycl")) => "wisper-linux-sycl",
        _ if cfg!(target_os = "linux") => "wisper-linux-cpu",
        _ => "wisper-unknown",
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppAbout {
    pub app_version: String,
    pub platform_os: String,
    pub release_artifact: String,
    #[serde(flatten)]
    pub compute: ComputeInfo,
}

pub fn app_about(app_version: impl Into<String>) -> AppAbout {
    AppAbout {
        app_version: app_version.into(),
        platform_os: platform_os_label().to_string(),
        release_artifact: release_artifact_label().to_string(),
        compute: compute_info(),
    }
}

/// Returns the GPU backend compiled into this binary, if any.
pub fn compiled_gpu_backend() -> Option<GpuBackendKind> {
    if cfg!(target_os = "macos") {
        return Some(GpuBackendKind::Metal);
    }

    if cfg!(feature = "gpu-cuda") {
        return Some(GpuBackendKind::Cuda);
    }

    if cfg!(feature = "gpu-sycl") {
        return Some(GpuBackendKind::IntelSycl);
    }

    if cfg!(feature = "gpu-vulkan") {
        return Some(GpuBackendKind::Vulkan);
    }

    None
}

pub fn validate_backend(backend: ComputeBackend) -> Result<(), crate::WisperError> {
    if backend == ComputeBackend::Gpu && compiled_gpu_backend().is_none() {
        return Err(crate::WisperError::Transcription(
            "GPU is not available in this build. Rebuild with gpu-vulkan (AMD/Intel/NVIDIA), \
             gpu-cuda (NVIDIA), or use a macOS build for Metal."
                .into(),
        ));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cpu_build_has_no_gpu() {
        if compiled_gpu_backend().is_none() {
            let info = compute_info();
            assert!(!info.gpu_available);
            assert!(info.gpu_backend.is_none());
            assert_eq!(info.default_backend, ComputeBackend::Cpu);
            assert!(!info.supports_cpu_fallback);
        }
    }

    #[test]
    fn gpu_build_prefers_gpu_default() {
        if compiled_gpu_backend().is_some() {
            let info = compute_info();
            assert!(info.gpu_available);
            assert_eq!(info.default_backend, ComputeBackend::Gpu);
            assert!(info.supports_cpu_fallback);
        }
    }

    #[test]
    fn gpu_device_indices_match_whisper_order() {
        let devices = gpu_devices();
        if compiled_gpu_backend().is_none() {
            assert!(devices.is_empty());
        }
        for (i, device) in devices.iter().enumerate() {
            assert_eq!(device.index, i as i32);
            assert!(!device.name.is_empty());
        }
    }

    #[test]
    fn auto_gpu_follows_power_source() {
        let gpu = |index, integrated| GpuDevice {
            index,
            name: format!("GPU {index}"),
            integrated,
        };
        let both = [gpu(0, true), gpu(1, false)];
        assert_eq!(auto_gpu_device(&both, Some(true)), 0);
        assert_eq!(auto_gpu_device(&both, Some(false)), 1);
        assert_eq!(auto_gpu_device(&both, None), 1);
        // Only one kind available: use it whatever the power source.
        assert_eq!(auto_gpu_device(&[gpu(0, true)], Some(false)), 0);
        assert_eq!(auto_gpu_device(&[gpu(0, false)], Some(true)), 0);
        assert_eq!(auto_gpu_device(&[], Some(true)), 0);
        // Real power reading must not crash; value depends on the machine.
        let _ = on_battery();
    }

    #[test]
    fn cpu_architecture_label_is_non_empty() {
        assert!(!cpu_architecture_label().is_empty());
        assert!(!compute_info().cpu_architecture.is_empty());
    }

    #[test]
    fn release_artifact_label_is_non_empty() {
        assert!(!release_artifact_label().is_empty());
    }
}

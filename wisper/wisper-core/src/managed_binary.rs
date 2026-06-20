//! Post-download prep for managed yt-dlp/ffmpeg binaries (permissions, macOS signing).

use std::path::Path;
use std::process::{Command, Stdio};

use crate::error::WisperError;

const HTTP_USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));

/// User-Agent required for reliable GitHub release downloads.
pub fn http_user_agent() -> &'static str {
    HTTP_USER_AGENT
}

pub fn http_get(url: &str) -> Result<ureq::Response, ureq::Error> {
    ureq::get(url).set("User-Agent", HTTP_USER_AGENT).call()
}

/// Mark a downloaded helper binary executable and macOS-runnable.
pub fn prepare_managed_binary(path: &Path) -> Result<(), WisperError> {
    #[cfg(unix)]
    set_unix_executable(path)?;

    #[cfg(target_os = "macos")]
    macos_adhoc_sign(path)?;

    #[cfg(not(any(unix, target_os = "macos")))]
    let _ = path;

    Ok(())
}

#[cfg(unix)]
fn set_unix_executable(path: &Path) -> Result<(), WisperError> {
    use std::os::unix::fs::PermissionsExt;

    let mut perms = std::fs::metadata(path)
        .map_err(|e| WisperError::Fetch(e.to_string()))?
        .permissions();
    perms.set_mode(0o755);
    std::fs::set_permissions(path, perms).map_err(|e| WisperError::Fetch(e.to_string()))?;
    Ok(())
}

#[cfg(target_os = "macos")]
fn macos_adhoc_sign(path: &Path) -> Result<(), WisperError> {
    // Downloads can carry quarantine; Gatekeeper blocks unsigned helpers until signed.
    let _ = Command::new("xattr")
        .args(["-cr", path.to_string_lossy().as_ref()])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status();

    let status = Command::new("codesign")
        .args(["--force", "--sign", "-", path.to_string_lossy().as_ref()])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .map_err(|e| WisperError::Fetch(format!("codesign failed: {e}")))?;

    if !status.success() {
        return Err(WisperError::Fetch(
            "macOS could not sign the downloaded tool — try Install again or add the tool to PATH"
                .into(),
        ));
    }
    Ok(())
}

pub fn binary_runnable(path: &Path, version_flag: &str) -> bool {
    Command::new(path)
        .arg(version_flag)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
}

pub fn yt_dlp_runnable(path: &Path) -> bool {
    binary_runnable(path, "--version")
}

pub fn ffmpeg_runnable(path: &Path) -> bool {
    binary_runnable(path, "-version")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn http_user_agent_is_set() {
        assert!(!http_user_agent().is_empty());
    }
}

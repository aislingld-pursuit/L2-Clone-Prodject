# Shared Cargo target dir — avoids OneDrive/sandbox paths that break MSVC/CMake on Windows.
$script:CargoTargetDir = Join-Path $env:LOCALAPPDATA "wisper-cargo\smoke"
if (-not (Test-Path $script:CargoTargetDir)) {
    New-Item -ItemType Directory -Force -Path $script:CargoTargetDir | Out-Null
}
$env:CARGO_TARGET_DIR = $script:CargoTargetDir

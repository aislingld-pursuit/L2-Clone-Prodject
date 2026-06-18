# Download GGML Whisper models into the Wisper app data folder.
# Usage:
#   .\scripts\download-model.ps1                    # ggml-base.en.bin (~150 MB)
#   .\scripts\download-model.ps1 -Model large-turbo # ~1.6 GB
#   .\scripts\download-model.ps1 -Model tiny        # ~75 MB
#   .\scripts\download-model.ps1 -All               # all three tiers (~1.8 GB total)

param(
    [ValidateSet("tiny", "base", "large-turbo")]
    [string]$Model = "base",
    [switch]$All
)

$ErrorActionPreference = "Stop"

$models = @{
    "tiny"        = @{ File = "ggml-tiny.en.bin";         Url = "https://huggingface.co/ggerganov/whisper.cpp/resolve/main/ggml-tiny.en.bin";         MinBytes = 50MB }
    "base"        = @{ File = "ggml-base.en.bin";         Url = "https://huggingface.co/ggerganov/whisper.cpp/resolve/main/ggml-base.en.bin";         MinBytes = 100MB }
    "large-turbo" = @{ File = "ggml-large-v3-turbo.bin"; Url = "https://huggingface.co/ggerganov/whisper.cpp/resolve/main/ggml-large-v3-turbo.bin"; MinBytes = 500MB }
}

$modelsDir = Join-Path $env:APPDATA "com.aislingldpursuit.wisper\models"
New-Item -ItemType Directory -Force -Path $modelsDir | Out-Null

function Download-OneModel([string]$Key) {
    $entry = $models[$Key]
    $dest = Join-Path $modelsDir $entry.File

    if (Test-Path $dest) {
        $len = (Get-Item $dest).Length
        $sizeMb = [math]::Round($len / 1MB, 1)
        if ($len -ge $entry.MinBytes) {
            Write-Host "Already exists: $($entry.File) ($sizeMb MB)" -ForegroundColor Green
            return
        }
        Write-Host "Removing invalid $($entry.File) ($sizeMb MB — expected at least $([math]::Round($entry.MinBytes / 1MB)) MB)" -ForegroundColor Yellow
        Remove-Item -Force $dest
    }

    Write-Host "Downloading $($entry.File) to:" -ForegroundColor Cyan
    Write-Host "  $dest"
    Write-Host "This may take several minutes for large models." -ForegroundColor Yellow

    try {
        Invoke-WebRequest -Uri $entry.Url -OutFile $dest -UseBasicParsing
    } catch {
        Write-Error "Download failed from $($entry.Url): $_"
        exit 1
    }

    $sizeMb = [math]::Round((Get-Item $dest).Length / 1MB, 1)
    Write-Host "Done — $sizeMb MB saved." -ForegroundColor Green
}

if ($All) {
    Write-Host "Downloading all Wisper speech models to $modelsDir" -ForegroundColor Cyan
    foreach ($key in @("tiny", "base", "large-turbo")) {
        Download-OneModel $key
    }
    Write-Host ""
    Write-Host "All models ready. Restart Wisper and pick Small / Medium / Large in Advanced options." -ForegroundColor Green
    exit 0
}

Download-OneModel $Model
Write-Host "Restart or reload Wisper, then transcribe with your chosen model tier."

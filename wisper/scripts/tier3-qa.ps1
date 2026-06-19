# Tier 3 QA — beta.21 partner gate (Aisling + Jimmy)
# Usage:
#   .\scripts\tier3-qa.ps1              # automated preflight only
#   .\scripts\tier3-qa.ps1 -Launch     # preflight then dev-cuda.ps1
#   .\scripts\tier3-qa.ps1 -Release     # also verify GitHub release v0.2.0-beta.21

param(
    [switch]$Launch,
    [switch]$Release
)

$ErrorActionPreference = "Stop"
$root = Split-Path $PSScriptRoot -Parent
$repoRoot = Split-Path $root -Parent
$expectedTag = "v0.2.0-beta.21"

Write-Host "`n========================================" -ForegroundColor Magenta
Write-Host " Tier 3 QA — $expectedTag partner gate" -ForegroundColor Magenta
Write-Host "========================================`n" -ForegroundColor Magenta

& (Join-Path $PSScriptRoot "phase1-exit-qa.ps1")
if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }

function Write-Step {
    param($msg)
    Write-Host "`n==> $msg" -ForegroundColor Cyan
}

Write-Step "beta.21 — all three model tiers on disk"
$modelsDir = Join-Path $env:APPDATA "com.aislingld-pursuit.wisper\models"
$expected = @(
    @{ Name = "ggml-tiny.en.bin"; MinMB = 50 },
    @{ Name = "ggml-base.en.bin"; MinMB = 100 },
    @{ Name = "ggml-large-v3-turbo.bin"; MinMB = 500 }
)
if (-not (Test-Path $modelsDir)) {
    Write-Host "WARN: models folder missing: $modelsDir" -ForegroundColor Yellow
    Write-Host "      Run: .\scripts\download-model.ps1 -All" -ForegroundColor Yellow
} else {
    foreach ($m in $expected) {
        $path = Join-Path $modelsDir $m.Name
        if (-not (Test-Path $path)) {
            Write-Host "WARN: missing $($m.Name)" -ForegroundColor Yellow
            continue
        }
        $mb = [math]::Round((Get-Item $path).Length / 1MB, 1)
        if ($mb -lt $m.MinMB) {
            Write-Host "WARN: $($m.Name) only ${mb} MB (expected >= $($m.MinMB) MB) — re-download" -ForegroundColor Yellow
        } else {
            Write-Host "OK: $($m.Name) (${mb} MB)" -ForegroundColor Green
        }
    }
}

if ($Release) {
    Write-Host "`n==> GitHub release $expectedTag" -ForegroundColor Cyan
    $gh = Get-Command gh -ErrorAction SilentlyContinue
    if (-not $gh) {
        Write-Host "WARN: gh CLI not found — check release manually on GitHub" -ForegroundColor Yellow
    } else {
        gh release view $expectedTag --repo aislingld-pursuit/L2-Clone-Prodject
        if ($LASTEXITCODE -ne 0) {
            Write-Host "FAIL: release $expectedTag not found" -ForegroundColor Red
            exit 1
        }
        Write-Host "OK: release published" -ForegroundColor Green
    }
}

Write-Host "`n========================================" -ForegroundColor Magenta
Write-Host " Tier 3 — manual checklist (both partners)" -ForegroundColor Magenta
Write-Host " Install: $expectedTag from GitHub Releases" -ForegroundColor Magenta
Write-Host "========================================" -ForegroundColor Magenta
Write-Host @"

AISLING (Windows CUDA)
[ ] Fresh install OR dev-cuda.ps1 on beta.21
[ ] Welcome guide: Check your system → recommendation → download tier
[ ] Download all three tiers (Advanced) — checkmarks on Small/Medium/Large
[ ] Transcribe with Medium; switch to Large and re-transcribe short clip
[ ] Remember-open Advanced: check box, restart app, Advanced still open
[ ] Privacy subtitle visible on transcribe panel
[ ] Library search + TXT export

JIMMY (Intel Mac)
[ ] Install Wisper_0.2.0-beta.21_x64.dmg
[ ] Same welcome → system check → model download flow
[ ] One end-to-end transcription (file or mic)
[ ] About shows Metal (or CPU if no GPU)

BOTH — before merge to master
[ ] Can explain app in under 10 seconds (HEART clarity test)
[ ] No blocking bugs found
[ ] Sign off in docs/TIER3-SIGNOFF.md (create when done)

"@ -ForegroundColor White

if ($Launch) {
    Write-Host "`n==> Starting dev-cuda.ps1" -ForegroundColor Cyan
    Push-Location $root
    try {
        & (Join-Path $root "dev-cuda.ps1")
    } finally {
        Pop-Location
    }
}

Write-Host "`nTier 3 preflight complete. Work through the checklist above.`n" -ForegroundColor Green

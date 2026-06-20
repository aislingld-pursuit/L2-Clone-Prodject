# Verify managed-tool download URLs respond (run before release).
$ErrorActionPreference = "Stop"

$urls = @(
    "https://github.com/yt-dlp/yt-dlp/releases/latest/download/yt-dlp.exe",
    "https://github.com/yt-dlp/yt-dlp/releases/latest/download/yt-dlp_macos",
    "https://github.com/yt-dlp/yt-dlp/releases/latest/download/yt-dlp",
    "https://github.com/BtbN/FFmpeg-Builds/releases/download/latest/ffmpeg-master-latest-win64-gpl.zip",
    "https://github.com/BtbN/FFmpeg-Builds/releases/download/latest/ffmpeg-master-latest-linux64-gpl.tar.xz",
    "https://evermeet.cx/ffmpeg/getrelease/zip",
    "https://evermeet.cx/ffmpeg/getrelease/ffprobe/zip",
    "https://www.osxexperts.net/ffmpeg80arm.zip",
    "https://www.osxexperts.net/ffprobe80arm.zip",
    "https://www.osxexperts.net/ffmpeg80intel.zip",
    "https://www.osxexperts.net/ffprobe80intel.zip"
)

$ua = "wisper-verify-tool-urls"
$failed = 0

foreach ($url in $urls) {
    try {
        $resp = Invoke-WebRequest -Uri $url -Method Head -UserAgent $ua -UseBasicParsing
        $len = $resp.Headers["Content-Length"]
        Write-Host ("OK {0} ({1} bytes)" -f $url, $len)
    } catch {
        Write-Host ("FAIL {0}: {1}" -f $url, $_.Exception.Message)
        $failed++
    }
}

if ($failed -gt 0) {
    throw "$failed tool URL(s) failed"
}

Write-Host "All tool URLs reachable."

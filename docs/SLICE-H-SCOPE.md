# Slice H — Locked scope (beta.25)

**Status:** Ready to implement  
**Ship order:** after beta.24 (Slice G)  
**Rule:** One slice → smoke test → commit → tag → Release CI

---

## Slice H — Managed tool auto-update (`beta.25`)

### In scope

- On app launch, **silently check** yt-dlp and ffmpeg copies under app data (`…/bin/`)
- If a managed copy exists and is **older than 7 days**, re-download the latest official build
- Does **not** install tools that are missing (manual install / bundled / PATH unchanged)
- Does **not** overwrite bundled installer copies in `resources/bin/`
- Reuses existing download + progress plumbing; auto-updates are non-blocking background work
- UI: no launch banner; optional progress in Advanced options only when a refresh runs

### Out of scope (not promised in beta.25)

- Auto-install missing yt-dlp/ffmpeg on launch
- Updating PATH-managed or bundled-only copies
- Wisper app self-update (already handled by `check_for_app_update`)
- User toggle for refresh interval (fixed 7-day cadence in beta.25)
- Batch export, Word/PDF/JSON/CSV, burn-in subs, diarization, word-level timing, broadcast line-wrap (Slice I+)

---

## User-facing summary

**beta.25:** yt-dlp and ffmpeg you installed through Wisper **stay current automatically** — quiet background refresh about once a week.

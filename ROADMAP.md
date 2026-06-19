# Wisper — Roadmap

Last updated: 2026-06-19 (`master` / beta.21 shipped; E+F scoped)

## Current phase

**Active user feedback → Slice E & F** on `master`. Latest tag: **v0.2.0-beta.21**.

| Milestone | Status |
|-----------|--------|
| Phase 0.5 GPU foundation | Done |
| Phase 1 import flows | Done |
| Phase 2 library minimum | Done |
| beta.19–21 (HEART UI, a11y, multi-model) | Done |
| Tier 3 partner QA + merge to `master` | Done (2026-06-19) |
| **Slice E — SRT/VTT export → beta.22** | **Next** |
| **Slice F — yt-dlp in-app installer → beta.23** | Planned |

**Authoritative plan:** [docs/Aisling-corrections.md](./docs/Aisling-corrections.md)  
**PRD status map:** [docs/Week2-PRD-STATUS.md](./docs/Week2-PRD-STATUS.md)

## HEART focus (beta.19–20)

Per [HEART framework](https://www.heartframework.com/) — prioritize **Task Success** and **Adoption** for the beta cohort.

| Goal | Metric |
|------|--------|
| First transcription started | Activation rate |
| Fast path to transcribe | TTFT (time to first transcribe) |
| Right model for hardware | Recommendation acceptance % |
| Clarity | Partner “explain app in 10s” test |

## beta.19 feature list (Slice B)

1. Advanced options label + remember-open checkbox  
2. Privacy subtitle  
3. Model tier: Small / Medium / Large (+ `large-turbo` in Rust)  
4. Hardware advisor: spec reader, optional benchmark, recommendation  
5. Model-missing inline banner  
6. Disabled-state polish; Advanced collapsed while recording  

**Explicit:** No file size limits on upload, URL, recording, or models.

## beta.20 (Slice C)

- `aria-expanded` on Advanced toggle  
- Escape collapses Advanced (confirm if URL dirty)  
- Optional video format hint (warn only)  

## Release pipeline

**Latest:** [v0.2.0-beta.21](https://github.com/aislingld-pursuit/L2-Clone-Prodject/releases/tag/v0.2.0-beta.21)

**Next tags:** `v0.2.0-beta.22` (SRT/VTT export), then `v0.2.0-beta.23` (yt-dlp installer).  
**Full scope:** [docs/SLICE-E-F-SCOPE.md](./docs/SLICE-E-F-SCOPE.md)

## beta.22 — Slice E (SRT / WebVTT export)

Export edited transcript segments as standard subtitle files for video editors and players. TXT export unchanged.

## beta.23 — Slice F (yt-dlp in-app installer)

One-click download of official yt-dlp into app data so URL import works without winget/Homebrew.

## Deferred (post beta.23)

- Pin Advanced Option C (Tauri prefs + pin icon)  
- Full analytics / PostHog  
- EmptyStateHero refactor  
- Batch export / burn-in subtitles / ffmpeg in-app (see SLICE-E-F-SCOPE out-of-scope)

# Wisper — TODO (beta.27 feature gates)

Last updated: 2026-06-08  
**Branch:** `master` (infra slices E–G); Slice UX **pending OK**  
**Rule:** After each feature → run smoke test → all green → commit → next feature.

Status: `[ ]` pending · `[~]` in progress · `[x]` done

---

## Tier 0 — Smoke gate (run after every feature)

```powershell
cd wisper
.\scripts\smoke-test.ps1
```

Must pass: `cargo test` (wisper-core), `cargo check`, `npm run build`.

---

## Slice A — Documentation

- `docs/Aisling-corrections.md` — authoritative plan + HEART  
- `docs/Week2-PRD-STATUS.md` — Jimmy PRD superseded map  
- Update README, ROADMAP, CHANGELOG, `.gitignore`  
- Branch `Jimmy-Contributions` + push

---

## Slice B — beta.19 (one feature → smoke → commit)

- **B7** — Rename “Advanced settings” → “Advanced options”  
- **B1** — Privacy subtitle on transcribe panel  
- **B5** — Collapse Advanced while recording  
- **B8** — Remember-open checkbox (`wisper-keep-advanced-open`)  
- **B6** — Model tier selector + `large-turbo` in `StarterModel`  
- **B9** — Hardware advisor (`get_system_profile`, `run_compute_benchmark`, recommend)  
- **B2** — Model-missing inline banner  
- **B3** — Disabled button styling pass  
- **B4** — GPU fallback copy alignment (if needed)  
- Bump version → **0.2.0-beta.19** + CHANGELOG  
- Tag `v0.2.0-beta.19` + Release CI

---

## Slice C — beta.20 (one feature → smoke → commit)

- **C3** — `aria-expanded` / `aria-controls` on Advanced toggle  
- **C4** — Escape closes Advanced  
- **C2** — Video format hint (warn only, no size cap)  
- **C5** — Extend `phase1-exit-qa.ps1`  
- Bump version → **0.2.0-beta.20** + CHANGELOG  
- Tag `v0.2.0-beta.20` + Release CI

---

## Slice D — beta.21 (multi-model)

- Tier-aware transcription and model status (`resolve_model_path_for_tier`)
- Download selected / download all models (UI + `start_download_all_models`)
- `download-model.ps1 -All` + `build-release.ps1` default fetch all tiers
- Minimum file-size validation (reject wrong/truncated `.bin`)
- Bump version → **0.2.0-beta.21** + CHANGELOG  
- Tag `v0.2.0-beta.21` + Release CI

---

## Tier 3 — Manual QA (beta.21 partner gate) — **complete 2026-06-19**

**Scripts:** `wisper/scripts/tier3-qa.ps1` · **Sign-off:** `docs/TIER3-SIGNOFF.md`

- [x] Automated preflight, welcome flow, model tiers, release smoke (Windows + Mac)
- [x] Both partners signed → merged `Jimmy-Contributions` → `master`

---

## Slice E — beta.22 (SRT / WebVTT export)

**Scope doc:** `docs/SLICE-E-F-SCOPE.md`

- **E1** — `format_transcript_srt` + `format_transcript_vtt` in `wisper-core/src/export/` ✅
- **E2** — Unit tests (timestamps, UTF-8, empty segments) ✅
- **E3** — Tauri export + save-file commands (mirror TXT) ✅
- **E4** — UI: Export SRT / Export VTT on transcript panel ✅
- **E5** — Bump version → **0.2.0-beta.22** + CHANGELOG + tag + Release CI (version/CHANGELOG done; tag/CI when ready to ship)

**Out of scope:** batch/zip export, Word/PDF/JSON, burn-in subs, speaker labels, word-level timing, cloud upload.

---

## Slice F — beta.23 (yt-dlp in-app installer)

**Scope doc:** `docs/SLICE-E-F-SCOPE.md`

- **F1** — Download yt-dlp to `app_data_dir()/bin/` (model-download pattern) ✅
- **F2** — Progress events + `start_yt_dlp_install` + `get_yt_dlp_status` ✅
- **F3** — Welcome guide + URL import install CTA when missing ✅
- **F4** — Install button in Advanced URL import panel ✅
- **F5** — Platform binaries: Win x64, Mac (`yt-dlp_macos`), Linux x64 ✅
- **F6** — Bump version → **0.2.0-beta.23** + CHANGELOG (tag/CI when ready to ship) ✅

**Out of scope (Slice F):** auto-update on launch, ffmpeg installer, cookies/auth, playlist UI, quality picker, proxy config.

---

## Slice G — ffmpeg installer + bundled yt-dlp (`beta.24`)

- **G1** — `download_ffmpeg` in wisper-core (BtbN zip/tar.xz → `app_data/bin/`) ✅
- **G2** — `resolve_ffmpeg` / `resolve_ffprobe` + audio decode uses app binaries ✅
- **G3** — Tauri `start_ffmpeg_install` + `get_ffmpeg_status` ✅
- **G4** — Welcome guide + Advanced options install UI ✅
- **G5** — Bundle yt-dlp in release installers (CI → `resources/bin/`) ✅
- **G6** — Bump version → **0.2.0-beta.24** + CHANGELOG ✅ (tag/CI shipped)

**Out of scope (Slice G):** ffmpeg bundled without download, auto-update ffmpeg, ffplay UI, burn-in subs.

---

## Slice H — managed tool auto-update (`beta.25`)

**Scope doc:** `docs/SLICE-H-SCOPE.md`

- **H1** — `refresh_stale_managed_tools` + 7-day staleness check in wisper-core ✅
- **H2** — `force_refresh` on `download_yt_dlp` / `download_ffmpeg` ✅
- **H3** — Tauri `start_managed_tools_refresh` (background on launch) ✅
- **H4** — Silent UI: no banner; Advanced shows progress when refresh runs ✅
- **H5** — Bump version → **0.2.0-beta.25** + CHANGELOG (tag/CI when ready to ship)

**Out of scope (Slice H):** auto-install missing tools, refresh bundled/PATH copies, user toggle for interval.

---

## Slice I — Export++ (`beta.26`)

**Scope doc:** `docs/SLICE-I-SCOPE.md`

- **I1** — `format_transcript_json` + `format_transcript_csv` in wisper-core ✅
- **I2** — `format_transcript_docx` + `format_transcript_pdf` (local generation) ✅
- **I3** — `build_transcript_bundle` ZIP (txt, srt, vtt, json, csv, docx, pdf) ✅
- **I4** — `build_library_bundle` + Tauri commands + UI ✅
- **I5** — Bump version → **0.2.0-beta.26** + CHANGELOG (tag/CI when ready to ship)

**Out of scope:** batch import queue, burn-in subs, diarization, word-level timing.

---

## Slice J — Batch import queue (`beta.27`)

**Scope doc:** `docs/SLICE-J-SCOPE.md`

- **J1** — Multi-select file picker + multi-file drag-and-drop ✅
- **J2** — Sequential import queue (same language/model/GPU per job) ✅
- **J3** — Queue progress UI + cancel clears queue ✅
- **J4** — Continue queue after per-file failure ✅
- **J5** — Bump version → **0.2.0-beta.27** + CHANGELOG (tag/CI when ready to ship)

**Out of scope:** parallel jobs, ZIP/folder import, playlist URLs, Resona UI.

---

## Explicitly out of scope (all slices)

- File size limits (upload, URL, recording, model)  
- Jimmy 13-event analytics suite  
- Pin icon / Tauri prefs file (Option C)

---

## Slice UX — Resona visual redesign (**approved — Phase A + B shipped in working tree**)

**Planning doc:** `docs/RESONA-VISUAL-REDESIGN.md`  
**Mockup:** `wisper/design/mockups/direction-ab-hybrid.html` (A + B hybrid)  
**Brand assets:** `wisper/design/brand/` (enhanced appmark)

### Phase A — Structure

- [x] **UX-A1** — Extract `EmptyStateHero.tsx` (Jimmy full dashed drop zone ~200px)
- [x] **UX-A2** — URL import row on main transcribe panel
- [x] **UX-A3** — Model-missing: full panel only; remove duplicate inline `model-banner`
- [x] **UX-A4** — Export dropdown (TXT / SRT / VTT)
- [x] **UX-A5** — Add `tokens.css` (Resona Deep Current)

### Phase B — Visual reskin + rebrand

- [x] **UX-B1** — Hybrid layout: gradient header + waveform + solid content card
- [x] **UX-B2** — Resona header: appmark + “Resona.” + tagline *a private whisper*
- [x] **UX-B3** — Reskin `App.css` / welcome guide to tokens
- [x] **UX-B4** — About / window title → Resona (Tauri `productName`; package id unchanged)
- [x] **UX-B5** — Enhanced appmark SVG (gradient leaf + wave bars + ring glow)

### Phase C — Layout (after A + B)

**Scope doc:** `docs/RESONA-PHASE-C-SCOPE.md`

- [ ] **UX-C1** — Two-column library + transcript (≥800px)
- [ ] **UX-C2** — Split Advanced: Setup vs per-job options

---

## Slice H — Resona polish layer (deferred, post Slice UX)

From original Resona app (`L2 project 1 Resona/resona/`). **Not** part of visual redesign.

- [ ] **H1** — Live streaming dictation (`streaming.rs`, `vad.rs`, partial/final events)
- [ ] **H2** — Partial transcript UI (live updating text while recording)
- [ ] **H3** — Grammar review pass (`grammar.ts` — capitalization, punctuation, duplicates)
- [ ] **H4** — Filler word removal (um, uh, you know, etc.)
- [ ] **H5** — Writing score + metrics (grammar, conciseness, readability, clarity)

**Prerequisite:** Slice UX Phase A + B shipped; partner HEART sign-off.


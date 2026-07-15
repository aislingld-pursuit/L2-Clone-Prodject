# Resona Phase C — Layout scope (UX-C1 + UX-C2)

**Status:** Decisions locked 2026-07-15 (4 of 4 — see Locked decisions)  
**Ship order:** after the Resona rebrand ships in `0.2.0-beta.30` (Phase A + B visual redesign went out in beta.29; rebrand completion is beta.30 — see release history)  
**Version target:** `0.2.0-beta.31` (proposed)  
**Rule:** One slice → smoke test → commit → tag → Release CI

Planning parent: [RESONA-VISUAL-REDESIGN.md](./RESONA-VISUAL-REDESIGN.md) · Task IDs tracked in [../TODO.md](../TODO.md) (Slice UX → Phase C).

---

## Phase C — Two-column layout + Advanced split (`beta.31`)

Phase A shipped the structure (EmptyStateHero, URL row, model-missing panel, export dropdown, `tokens.css`) and Phase B the visual reskin + Resona rebrand. Phase C is the **layout** phase: give the app breathing room on wide windows and untangle the Advanced options list. No new transcription features — this is arrangement only.

### In scope

- **UX-C1 — Two-column library + transcript (≥800px)** — on windows wider than `800px`, show the recordings **library** and the active **transcript** side by side instead of stacked. Below `800px` the layout stays the current single stacked column. The header, the Transcribe/empty-state card, and the Advanced panel remain full width above the two-column region.
- **UX-C2 — Split Advanced into Setup vs per-recording** — within the single-column Advanced panel, add two labelled sub-groups: **Setup** (install-once, machine-level: compute device, speech model + downloads, and the yt-dlp + ffmpeg tool installers consolidated here) and **Per-recording** (options that vary per file, starting with transcription language), so users stop scanning one flat list to find the one control they change per file.

### Out of scope (not promised in beta.31)

- Resizable / draggable column splitter (columns are a fixed responsive grid)
- Three-column or master-detail navigation
- Persisting column widths or a collapse-one-column toggle
- New Advanced controls, new settings, or new persistence keys beyond what the split needs
- Moving transcription features between panels; any change to transcribe/export behavior
- Resona Slice H polish layer (streaming dictation, grammar, filler-word, writing score) — separate deferred slice

---

## Technical notes

Current structure (verified against the tree):

- `App.tsx` returns `<main className="app resona-app">` — a single flex **column** (`resona.css`, `.resona-app` `flex-direction:column`, effective `max-width:960px`). Its `.app`/`.resona-app` dual max-width (720px vs 960px) resolves to **960px** only because `resona.css` imports last; reconcile these two rules as part of C1 rather than relying on import order.
- The **library** (`<section className="panel library">`, ~`App.tsx:1863-1911`) and **transcript** (`<section className="panel transcript">`, ~`App.tsx:1913-1998`) are **full-width sibling sections** rendered *below* the `.body-scroll` card, each positioned by `resona.css:397-402` (`.library, .transcript, .advanced-panel { margin: 0 1.5rem 1rem }`).
- The **Advanced panel** (`<section id="advanced-panel">`, ~`App.tsx:1648-1861`) is one collapsible unit driven by a single `showAdvanced` boolean.

UX-C1 approach (minimal-diff):

- In `App.tsx`, wrap the two existing library/transcript sections in one new container (e.g. `<div className="workspace-columns">`) **without touching their internal markup**. Keep the Advanced panel *outside* that wrapper (it stays full-width above).
- In `resona.css`, add a CSS grid on `.workspace-columns` behind `@media (min-width: 800px)`; zero the per-section side margins inside the wrapper so a grid `gap` (match the existing `1.5rem` / `1rem` spacing conventions — there are **no** spacing tokens, only color tokens) doesn't double up. Use `--hairline` for any column divider.
- **Graceful degradation is the main correctness risk:** both sections are conditionally rendered (library needs `library.length > 0 || query`; transcript needs `segments.length > 0`). When only one is present it must fill the row, not sit at half width — use a collapsing `grid-template-columns` / `:only-child` handling, not a fixed `1fr 1fr`.
- At `960px` cap, two columns land near `~450px` each. Verify the transcript row grid (`grid-template-columns: 7rem 1fr`), editable `.segment-edit` textareas, and inputs like `.language-select` (`max-width:280px`) / `.url-input` wrap cleanly.
- **Independent per-column scroll (decided):** each column scrolls on its own so a long transcript doesn't push the library down. Give the row a bounded height (row is `flex:1` under `main` with `min-height:0`) and set `overflow:auto` + `align-items:start` on the two columns, rather than relying on the current single `.body-scroll` region. Below 800px, revert to the existing whole-page scroll.
- Preserve the existing `@media (max-width:520px)` transcript-row collapse under the new wrapper.

UX-C2 approach (**decided:** two labelled sub-groups in one column — not two side-by-side columns):

- Reorder/group the panel JSX (`App.tsx:1648-1861`) into a **Setup** sub-group and a **Per-recording** sub-group under two `advanced-subtitle`-style headings, kept in the existing single column. **Language** is the only Per-recording control; everything else (CPU/GPU compute, speech-model tier + Download selected / Download all / Open setup guide / Open models folder, Install ffmpeg, background-update status, "Keep open on this computer") is Setup. Model tier stays under Setup despite its speed/accuracy trade-off.
- **Consolidate tool installers under Setup (decided):** move the yt-dlp installer out of `UrlImportRow.tsx:72-106` into the Setup sub-group next to the ffmpeg installer, so both tool installs share one home. This threads `managedToolsReady`, `ytDlpStatus`, and `onInstallYtDlp` into the panel; keep a lightweight contextual install affordance on the URL row only when URL import is attempted while yt-dlp is missing, otherwise the panel is the single source.
- Preserve all existing behavior: Escape-to-close and the discard-URL prompt (`App.tsx:752-779`), the auto-collapse while `isRecording` (`752-756`), every per-control disabled gate (`busy || isRecording`, GPU on `!gpu_available`, model tier on `modelDownloading`), and the persistence keys (`wisper-compute-backend`, `wisper-language`, `wisper-show-advanced`, `wisper-keep-advanced-open`, `wisper-model-tier`). Keep the **single `showAdvanced` toggle** — sub-groups are visual only, so no new collapse state/keys.
- New sub-group heading/wrapper classes must be styled in **both** `App.css` and `resona.css` or the Resona theme drifts (only `.advanced-panel` is themed today).

Files expected to change: `wisper/src/App.tsx`, `wisper/src/resona.css` (primary), `wisper/src/App.css` (Advanced group styling / transcript reflow). Optionally `wisper/src/UrlImportRow.tsx` if the yt-dlp installer (currently outside the panel, `UrlImportRow.tsx:72-106`) is consolidated with the in-panel ffmpeg installer.

## Locked decisions (2026-07-15)

- **Advanced split → two labelled sub-groups in one column.** Not two side-by-side columns. The Advanced panel stays single-column; add "Setup" and "Per-recording" headed sub-groups. Scales as per-recording options grow and avoids an empty-looking column while Language is the only per-recording control. Model tier stays under Setup (install-once), despite its speed/accuracy trade-off.
- **Tool installers → consolidated under Setup.** Move the yt-dlp installer from `UrlImportRow` into the Setup sub-group next to ffmpeg; one home for all tool installs, with an optional contextual fallback on the URL row when yt-dlp is missing at import time.
- **Column scroll → independent per-column.** Library and transcript scroll separately at ≥800px; whole-page scroll below 800px.

### Version — resolved: `beta.31`

Phase C ships as its own tag `v0.2.0-beta.31`, per the one-slice-per-tag rule. Phase A + B already shipped across beta.29 and beta.30 (see release history below), so Phase C is next in line.

## Release history (Phase A + B — shipped)

Phase A (UX-A1–A5) and Phase B (UX-B1–B5) are **shipped and wired** — all five redesign components (`EmptyStateHero`, `UrlImportRow`, `ModelMissingPanel`, `ExportMenu`, `AppHeader`) are imported and used in `App.tsx`; `tokens.css` + `resona.css` exist; the Resona header ("Resona." + *a private whisper*), window title, and About screen are rebranded. The rollout landed in two tags:

- **`v0.2.0-beta.29`** — Resona A+B visual redesign (header, hero, URL row, model-missing panel, export dropdown, "Deep Current" theme) plus managed-tool reliability + GPU status hardening. Tagged at `82368c5`, which still carried `productName = "Wisper"` and a few stray "Wisper" UI strings.
- **`v0.2.0-beta.30`** — rebrand completion: Tauri `productName` → `"Resona"` (installer/app name; artifacts now build as `Resona_…`) and the last 5 "Wisper" strings in `WelcomeGuide.tsx` / `App.tsx` fixed. Package identifier (`com.aislingldpursuit.wisper`) unchanged, so models and library carry over in place. `grep "Wisper" src` is now clean of user-facing copy.

Phase C (this doc) builds on beta.30 and ships as **beta.31**.

## Smoke gate

```powershell
cd wisper
.\scripts\smoke-test.ps1
```

Must pass `cargo test` (wisper-core), `cargo check`, and `npm run build`. Phase C is UI-only — pay attention to the `npm run build` (TypeScript/Vite) leg and manually verify the ≥800px and <800px layouts plus Advanced keyboard behavior (Escape, record auto-collapse).

---

## User-facing summary

**beta.31:** On a wide window, Resona shows your **recordings** and the **transcript** side by side, and the **Advanced options** are tidied into a **Setup** group (install once) and **Per-recording** options (change per file). On a narrow window everything stacks exactly as before.

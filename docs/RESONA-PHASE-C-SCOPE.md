# Resona Phase C — Layout scope (UX-C1 + UX-C2)

**Status:** Draft for review — pending Aisling OK  
**Ship order:** after Resona Phase A + B (in the working tree at `0.2.0-beta.29`)  
**Version target:** `0.2.0-beta.30` (proposed)  
**Rule:** One slice → smoke test → commit → tag → Release CI

Planning parent: [RESONA-VISUAL-REDESIGN.md](./RESONA-VISUAL-REDESIGN.md) · Task IDs tracked in [../TODO.md](../TODO.md) (Slice UX → Phase C).

---

## Phase C — Two-column layout + Advanced split (`beta.30`)

Phase A shipped the structure (EmptyStateHero, URL row, model-missing panel, export dropdown, `tokens.css`) and Phase B the visual reskin + Resona rebrand. Phase C is the **layout** phase: give the app breathing room on wide windows and untangle the Advanced options list. No new transcription features — this is arrangement only.

### In scope

- **UX-C1 — Two-column library + transcript (≥800px)** — on windows wider than `800px`, show the recordings **library** and the active **transcript** side by side instead of stacked. Below `800px` the layout stays the current single stacked column. The header, the Transcribe/empty-state card, and the Advanced panel remain full width above the two-column region.
- **UX-C2 — Split Advanced into Setup vs per-job** — group the Advanced options panel into a **Setup** section (install-once, machine-level: compute device, speech model + downloads, tool installers) and a **Per-job** section (options that vary per recording, starting with transcription language), so users stop scanning one flat list to find the one control they change per file.

### Out of scope (not promised in beta.30)

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
- At `960px` cap, two columns land near `~450px` each. Verify the transcript row grid (`grid-template-columns: 7rem 1fr`), editable `.segment-edit` textareas, and inputs like `.language-select` (`max-width:280px`) / `.url-input` wrap cleanly. Consider `align-items: start` and independent per-column scroll so a tall transcript doesn't stretch the library.
- Preserve the existing `@media (max-width:520px)` transcript-row collapse under the new wrapper.

UX-C2 approach:

- Reorder/group the panel JSX (`App.tsx:1648-1861`) into a **Setup** group and a **Per-job** group. Per the current controls, **Language** is the only genuinely per-job option; everything else (CPU/GPU compute, speech-model tier + Download selected / Download all / Open setup guide / Open models folder, Install ffmpeg, background-update status, "Keep open on this computer") is Setup.
- Preserve all existing behavior: Escape-to-close and the discard-URL prompt (`App.tsx:752-779`), the auto-collapse while `isRecording` (`752-756`), every per-control disabled gate (`busy || isRecording`, GPU on `!gpu_available`, model tier on `modelDownloading`), and the persistence keys (`wisper-compute-backend`, `wisper-language`, `wisper-show-advanced`, `wisper-keep-advanced-open`, `wisper-model-tier`). The split is visual grouping only — keep the single `showAdvanced` toggle unless a design decision adds independently-collapsible groups (which would need new state/keys).
- New group-wrapper classes must be styled in **both** `App.css` and `resona.css` or the Resona theme drifts (only `.advanced-panel` is themed today).

Files expected to change: `wisper/src/App.tsx`, `wisper/src/resona.css` (primary), `wisper/src/App.css` (Advanced group styling / transcript reflow). Optionally `wisper/src/UrlImportRow.tsx` if the yt-dlp installer (currently outside the panel, `UrlImportRow.tsx:72-106`) is consolidated with the in-panel ffmpeg installer.

## Open questions (resolve before locking)

- **Lopsided Advanced split:** Language is the *only* per-job control today. A dedicated Per-job column/section will look nearly empty. Options: (a) ship it anyway as a clear home for future per-job options; (b) reframe **model tier** as per-job (it carries a real speed/accuracy trade-off, though it's currently install-once, `wisper-model-tier`); (c) keep one column and use headed sub-groups instead of two columns. Recommend (a) + labelled sub-groups.
- **yt-dlp installer location:** ffmpeg install lives *in* the Advanced panel but its sibling yt-dlp install lives in `UrlImportRow`. Consolidate both under Setup, or leave yt-dlp on the URL row? Consolidating threads extra props (`managedToolsReady`, `ytDlpStatus`, `onInstallYtDlp`) into the panel.
- **Column scroll model:** whole-row scroll vs independent per-column scroll (library vs transcript). Independent scroll reads better for long transcripts but changes the current `.body-scroll` `flex:1 / overflow:auto` model.
- **Version/tag:** confirm Phase C ships as `beta.30` (vs folding into the unreleased `beta.29`).

## Smoke gate

```powershell
cd wisper
.\scripts\smoke-test.ps1
```

Must pass `cargo test` (wisper-core), `cargo check`, and `npm run build`. Phase C is UI-only — pay attention to the `npm run build` (TypeScript/Vite) leg and manually verify the ≥800px and <800px layouts plus Advanced keyboard behavior (Escape, record auto-collapse).

---

## User-facing summary

**beta.30:** On a wide window, Resona shows your **recordings** and the **transcript** side by side, and the **Advanced options** are tidied into a **Setup** area (install once) and **per-job** options (change per recording). On a narrow window everything stacks exactly as before.

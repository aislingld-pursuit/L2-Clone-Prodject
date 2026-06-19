# Wisper — TODO (beta.21 feature gates)

Last updated: 2026-06-18  
**Branch:** `Jimmy-Contributions`  
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

## Explicitly out of scope

- File size limits (upload, URL, recording, model)  
- Jimmy 13-event analytics suite  
- Pin icon / Tauri prefs file (Option C)


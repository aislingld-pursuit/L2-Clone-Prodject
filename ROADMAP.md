# Wisper — Roadmap

Last updated: 2026-06-19 (`Jimmy-Contributions` / beta.21)

## Current phase

**Tier 3 partner QA → merge to `master`** on branch `Jimmy-Contributions`. Latest tag: **v0.2.0-beta.21**.

| Milestone | Status |
|-----------|--------|
| Phase 0.5 GPU foundation | Done |
| Phase 1 import flows | Done |
| Phase 2 library minimum | Done |
| SEC-001 / SEC-002 | Done |
| beta.15 welcome guide + model download | Done |
| beta.17–18 update checks | Done |
| Week 2 reconciliation docs | Done — `docs/Aisling-corrections.md` |
| beta.19 HEART UI (model tier, hardware advisor) | Done |
| beta.20 a11y + format hints | Done |
| beta.21 multi-model (all tiers + download-all) | Done |
| Tier 3 partner QA + sign-off | **Done** — `docs/TIER3-SIGNOFF.md` (2026-06-19) |
| Merge `Jimmy-Contributions` → `master` | **Done** — beta.21 on `master` |

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

**Next tag:** After Tier 3 sign-off and merge to `master` (beta.22+ only if partner feedback requires it).

## Deferred (post beta.20)

- Pin Advanced Option C (Tauri prefs + pin icon)  
- Full analytics / PostHog  
- yt-dlp in-app installer  
- EmptyStateHero refactor  
- SRT/VTT export  

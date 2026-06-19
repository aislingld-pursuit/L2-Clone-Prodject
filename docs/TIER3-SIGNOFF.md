# Tier 3 QA sign-off — v0.2.0-beta.21

**Branch:** `Jimmy-Contributions`  
**Target merge:** `master` after both partners sign below  
**Release:** [https://github.com/aislingld-pursuit/L2-Clone-Prodject/releases/tag/v0.2.0-beta.21](https://github.com/aislingld-pursuit/L2-Clone-Prodject/releases/tag/v0.2.0-beta.21)

Run preflight:

```powershell
cd wisper
.\scripts\tier3-qa.ps1 -Release
.\scripts\tier3-qa.ps1 -Launch   # optional: opens dev build for manual steps
```

---

## Aisling (Windows CUDA)


| Check                                   | Pass | Notes                      |
| --------------------------------------- | ---- | -------------------------- |
| Smoke test green                        | [✅]  | `.\scripts\smoke-test.ps1` |
| Welcome → system check → model download | [✅]  |                            |
| All three tiers installed + selectable  | [✅]  |                            |
| Transcription works (Medium + Large)    | [✅]  |                            |
| Remember-open Advanced persists         | [✅]  |                            |
| Privacy subtitle + model-missing banner | [✅]  |                            |
| Library search + TXT export             | [✅]  |                            |


**Signed:** Aisling LD **Date:** 6/19/26

---

## Jimmy (Intel Mac)


| Check                                    | Pass | Notes       |
| ---------------------------------------- | ---- | ----------- |
| Installed `Wisper_0.2.0-beta.21_x64.dmg` | [✅]  |             |
| Welcome → system check → model download  | [✅]  |             |
| End-to-end transcription                 | [✅]  |             |
| About shows expected backend             | [✅]  | Metal / CPU |


**Signed:** Jimmy Ong **Date:** 06/19/26

---

## Merge gate

- [x] Both tables complete
- [x] HEART clarity test passed (explain app in <10s)
- [x] No P0/P1 bugs open
- [x] Ready to merge `Jimmy-Contributions` → `master`

**Merge approved by:** Aisling LD & Jimmy Ong **Date:** 2026-06-19
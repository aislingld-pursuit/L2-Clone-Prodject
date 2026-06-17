# Wisper — QA checklist (execution order)

Use this sequence when validating a release candidate. Each tier gates the next.

---

## 1. Local automated smoke

```powershell
cd wisper
.\scripts\smoke-test.ps1
```

**Pass criteria:** all tests pass, `cargo check` clean, `npm run build` succeeds.

---

## 2. Phase 1 preflight

```powershell
cd wisper
.\scripts\phase1-exit-qa.ps1
```

**Pass criteria:** wisper-core tests pass; warnings for missing yt-dlp/ffmpeg/model are documented (not blocking if testing file-only flows).

---

## 3. Release CI verification (after pushing a tag)

```powershell
gh run list --workflow=release.yml --limit 1
gh run view <run-id>
gh release view v0.2.0-beta.11
```

**Pass criteria:**

- macOS, Linux, Windows jobs all green
- Artifacts uploaded (not “No files were found”)
- GitHub Release published with `.exe` / `.AppImage` / `.dmg`

**Current release:** [v0.2.0-beta.11](https://github.com/aislingld-pursuit/L2-Clone-Prodject/releases/tag/v0.2.0-beta.11) — all jobs green (2026-06-08).

---

## 4. Fresh-install smoke (per platform)


| Platform       | Artifact                 | Checks                                               |
| -------------- | ------------------------ | ---------------------------------------------------- |
| Windows NVIDIA | `wisper-windows-cuda`    | App launches, About shows CUDA, one file transcribes |
| macOS          | `wisper-macos-universal` | App launches, About shows Metal                      |
| Linux          | `wisper-linux-vulkan`    | App launches, About shows Vulkan                     |


---

## 5. Phase 1 manual matrix


| #   | Flow         | Steps                            | Expected                            |
| --- | ------------ | -------------------------------- | ----------------------------------- |
| 1   | Mic          | Record → stop → transcribe       | Segments with timestamps            |
| 2   | File         | Drag MP3 or pick file            | Progress completes, library entry   |
| 3   | URL          | Paste short YouTube URL          | Download → transcribe; source label |
| 4   | URL cancel   | Cancel during download           | “Download cancelled”                |
| 5   | URL error    | Invalid URL                      | “Download failed”                   |
| 6   | Edit         | Edit segment → restart app       | Text persisted                      |
| 7   | Offline      | Block network after download     | Transcribe still works              |
| 8   | Video        | Drop MP4/MOV                     | Audio extracted, transcribed        |
| 9   | Library      | Search, export TXT, copy, delete | All work                            |
| 10  | GPU fallback | Force GPU failure if possible    | CPU fallback banner                 |


---

## 6. Security spot-check


| #   | Test                         | Expected                |
| --- | ---------------------------- | ----------------------- |
| 1   | Export TXT                   | Native save dialog only |
| 2   | URL `http://127.0.0.1`       | Rejected before yt-dlp  |
| 3   | URL `http://169.254.169.254` | Rejected                |


---

## 7. Week 2 UX (after progressive disclosure ships)


| #   | Test            | Expected                                         |
| --- | --------------- | ------------------------------------------------ |
| 1   | Cold open       | Empty-state hero; Record + Choose file prominent |
| 2   | Defaults        | Transcribe works without opening Advanced        |
| 3   | Advanced        | Language, Compute, URL visible when expanded     |
| 4   | Regression      | Download → Transcribe progress unchanged         |
| 5   | Moderated (n=5) | ≥4/5 articulate next step in 10s                 |


---

## 8. Beta sign-off

- Release notes in CHANGELOG match shipped tag
- README “Which build should I use?” matches available artifacts
- Version in About dialog matches release tag
- No open P0 bugs from tester feedback


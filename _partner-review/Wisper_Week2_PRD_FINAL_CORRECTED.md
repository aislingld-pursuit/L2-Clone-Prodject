# PER — WEEK 2: GUIDED FIRST SCREEN

**Product Requirements Document: Week 2 Improvement**

**Build name:** Wisper — Guided First Screen (Progressive Disclosure)

**Owner:** Aisling Ld Pursuit & Jimmy Ong

**Date:** June 8, 2026

**Status:** READY FOR DEVELOPMENT (All Issues Resolved)

**Mockup Reference:** [INSERT FIGMA LINK HERE - REQUIRED BEFORE DEV STARTS]
*Figma frames required: Empty Hero State, Hero with Subtitle, Advanced Toggle Collapsed, Advanced Options Expanded, Model Downloading, Recording in Progress, Error States (file too large, yt-dlp missing, gpu oom)*

---

## 1. PROBLEM

First-time Wisper users experience confusion and hesitation on the home screen during beta testing because Compute, Language, Record/import, URL import, and library controls are all visible at once before they complete a first transcription, resulting in slower time-to-first-transcript, abandoned sessions, and the impression that the app is harder to use than Whisper Notes despite strong backend capabilities.

### Supporting Context

- **Hick's Law:** Decision time increases with the number of choices — a core Laws of UX pattern from L2 Week 2 prioritization work.
- **Current friction:** Beta testers must parse three panels (Compute, Language, Record or import) before understanding the single job: get audio in, transcribe locally.
- **Competitive parity:** Whisper Notes leads with a simple capture-first home screen; Wisper already matches on progress feedback during long jobs but not on first-screen clarity.

### 1a. Opportunity

Reduce first-session friction so more beta testers complete one local transcription without reading the README — unlocking the value Wisper already delivers during download and transcribe progress.

### Market Opportunity

- **Week 2 beta goal:** Trusted testers install, transcribe once, and return to the library — first-screen clarity directly affects activation for a small but high-signal cohort.
- **See:** Project CHANGELOG.md (Phase 1 exit QA), local ROADMAP.md, and Week 2 impact/effort prioritization (progressive disclosure vs release CI vs model download).

### 1b. Users & Needs

**Primary user(s):**
- Beta testers and first-time users — students and professionals trying Wisper for the first time who want to transcribe one file or recording quickly without configuring GPU backends or optional URL import.

**Secondary users:**
- Returning power users who want access to language, compute, and URL import without cluttering the default view.

### Key User Needs

- **As a first-time beta tester:** I need to see one obvious next step when I open Wisper because I should not have to read documentation to transcribe my first file.
- **As a student importing a lecture:** I need file pick and transcribe to be front and center because I do not care about GPU backends on day one.
- **As a returning user:** I need to expand Advanced options when I want URL import or a fixed language because I should not lose power features.
- **As a privacy-conscious user:** I need the simplified screen to still show that transcription is local because trust matters even in a minimal UI.

---

## 2. PROPOSED SOLUTION

Wisper Week 2 improves the existing desktop app with a guided first screen and progressive disclosure. On launch, users see one clear job — drop or choose audio, or record — with Language defaulting to Auto-detect and Compute defaulting to the best available backend. Advanced options (language picker, CPU/GPU toggle, URL import, model path hints) move behind a collapsible Advanced section. An empty-state hero appears when no transcript is loaded, replacing the current three-panel wall. Long-job progress UI (Download → Transcribe, GPU fallback banners) stays unchanged.

### 2a. Value Proposition

Beta testers and privacy-conscious users who open Wisper for the first time use the guided first screen to start a local transcription in seconds. Unlike the current beta UI that exposes every workflow upfront, this improvement matches Whisper Notes' simplicity at the front door while keeping power features one click away.

### 2a(i). FIRST-RUN LAYOUT: HERO + MODEL BANNER (SCENARIOS A & B)

When a user opens Wisper for the first time, we handle two scenarios:

**SCENARIO A: Model is already available locally**
- ✓ Hero (drop zone + Record button + Choose File button) takes center stage
- ✓ Subtitle visible: "Transcription runs locally on your device. No data leaves your computer."
- ✓ Advanced toggle below
- ✓ **User sees:** "Drop audio here" — clear and obvious
- ✓ **All buttons enabled, full opacity, clickable**

**SCENARIO B: Model needs to be downloaded**

Triggers for download:
- First app install (model not present locally)
- Model update available (version changed)
- Model file corrupted (checksum mismatch)
- User cleared app data (model deleted)

During download:
- ✓ Hero and model-download banner are ONE visual unit (same background color, no separator)
- ✓ Banner displays progress: `[████░░░░░░░░░░░░░] 35% (147 MB / 420 MB)`
- ✓ Text: "Downloading transcription model. This takes 2-5 minutes on first launch."
- ✓ Estimated time shown after 30 seconds: "About 3 minutes remaining"
- ✓ Drop zone, Record button, Choose File button are **FULLY DISABLED**:
  - `pointer-events: none` (not clickable)
  - `color: #999999` (gray text, not opacity-based)
  - `cursor: not-allowed`
- ✓ Buttons DO NOT show tooltip on hover (disabled elements don't receive hover)
- ✓ Instead, help text appears NEAR banner: "Model downloading... buttons will be enabled when ready"
- ✓ Advanced toggle is also **DISABLED** during download
- ✓ Subtitle says: "Once ready, drop your audio below"
- ✓ **User sees:** "Wait, then drop audio" — still obvious, just in order

**[P0] If user tries to drag file over disabled drop zone:**
- Drag is rejected (`pointer-events: none` prevents drop)
- No visual drop effect
- Helpful hint visible: "Model downloading... please wait"

**[P0] If download fails or is cancelled:**
- Download button changes to "[Retry Download]" button
- Second button: "[Use URL Import]" (alternate import path)
- User can retry or use different import method
- All buttons re-enable after user chooses action

**[P0] When model download completes (100%):**
- Banner disappears immediately
- All buttons become enabled
- Background color returns to normal
- User can now drop files

**Design principle:** Banner is visually part of the hero unit (same container, same background), NOT a separate modal/overlay. This keeps the "one cohesive unit" feeling.

### 2b. Top 3 MVP Value Props

- **The Vitamin (must-have baseline):** A clear, uncluttered home screen that tells users what to do first — the baseline expectation for any consumer-grade desktop app.
- **The Painkiller (solves the core pain):** Eliminates first-run overwhelm so users reach their first transcript instead of bouncing after scanning Compute, Language, and URL panels.
- **The Steroid (the magic moment):** Open Wisper, drop an MP3, tap Transcribe — the app feels obvious before the progress bar even moves, while advanced users can still expand GPU and URL tools when needed.

### 2c. Goals & Non-Goals

**Goals**
- Reduce time from app open to first transcription attempt to under 30 seconds for file import (measures UI clarity, excludes download time).
- Keep all existing Phase 1 flows (record, file, URL import, language selection, compute/GPU toggle) accessible via Advanced — no feature removal.
- Preserve accessibility: progress regions, labels, keyboard paths, and screen reader support for primary actions.
- Ensure Advanced section state is predictable and user-configurable (session persist + pin preference).
- **Zero regressions in Phase 1 flows** (file transcription, recording, URL import, language selection, GPU toggle, library, export).

**Non-Goals**
- Replacing the first-run model setup banner with in-app download — separate improvement.
- Changing transcription engine, library schema, or export formats in this slice.
- Implementing in-app yt-dlp installer (defer to Week 3; Week 2 shows error message + link only).
- Mobile UI optimization (desktop-first in Week 2; responsive CSS added but not tested on mobile devices).

### 2d. Success Metrics

| Goal Signal | Metric | Target | Notes |
|---|---|---|---|
| **First-session activation** | First-time user starts transcription (clicks Record or Choose File AND uploads/records) | >85% of beta testers initiate ≥1 transcription in session 1 | Measures if hero is clear enough to prompt action; lowered from "complete" to "start" to account for long transcription times |
| **Time to first action** | User initiates transcription (clicks Record or Choose File) | Median <30 seconds from app launch | Measures UI clarity only—does NOT include download/transcription time; tests hero visibility & button prominence |
| **Clarity (qualitative)** | Tester describes primary action without prompting | 4/5 moderated testers (80%) can state "drop audio or click Record" in <10 seconds | Validates empty-state messaging; tests if hero communicates purpose |
| **Power-user regression** | Advanced users still reach URL/GPU controls efficiently | 100% of power users find Advanced toggle within 2 clicks AND expand within 1 click | Ensures progressive disclosure doesn't trap power users |
| **Advanced pin adoption** | Users who expand Advanced 3+ times adopt pin preference | Among power users using Advanced 3+ times, 75%+ will pin it by session 3 (if they return) | Measures if pin preference solves repeated-click friction; optional metric (depends on usage frequency) |
| **Zero regressions** | Existing Phase 1 features still work without new bugs | File transcription, Recording, URL import, Library, Language toggle, GPU toggle, Export all pass smoke tests | Validates no breaking changes from Week 1 |

**Note on activation metric:** Changed from "Transcription Complete" to "Transcription Started" because:
- Model downloads: 3-10 minutes
- Transcription time: 1x to 5x audio duration (1 hour file = 1-5 hours to transcribe)
- Most users will not wait for completion in first session
- "Started transcription" is better signal of: "UI was clear enough to prompt action"

---

## 3. REQUIREMENTS

### USER JOURNEY 1: FIRST-TIME BETA TESTER TRANSCRIBES A LOCAL FILE

**CONTEXT:** Highest-frequency Week 2 path. Optimizing for clarity and speed to first action — the activation metric for beta.

#### SUB-JOURNEY: LANDING ON A CLEAR HOME SCREEN

**[P0] USER SEES A GUIDED EMPTY STATE WHEN NO TRANSCRIPT IS LOADED (DROP ZONE + PRIMARY ACTIONS)**
- See Mockup: Frame "Empty Hero State"
- Empty-state component `EmptyStateHero.tsx` renders when `transcripts.length === 0`
- Drop zone is the focal point: dashed border (2px, #ccc), 200px tall, centered, flex layout
- Icon in drop zone: downward arrow (↓)
- Text in drop zone: "Drop your audio file here"
- Accepts drag-and-drop of audio files

**[P0] USER SEES RECORD AND CHOOSE AUDIO FILE AS PRIMARY BUTTONS WITHOUT SCROLLING**
- Two prominent buttons below drop zone:
  - Left: "🎤 Record" button (120px wide, high contrast)
  - Right: "📁 Choose File" button (120px wide, high contrast)
- Buttons are same size, high contrast (foreground/background ≥4.5:1), clear text labels + icons
- No icons-only (both have text labels for accessibility and clarity)
- Compute/Language panels are hidden by default (moved to Advanced section)
- No horizontal scrolling required on 1024px+ viewports
- Buttons respond to: click, Enter key, Space key

**[P0] ADVANCED SECTION IS COLLAPSED BY DEFAULT AND LABELED CLEARLY**
- Button structure:
  ```html
  <button 
    aria-expanded="false" 
    aria-controls="advanced-panel"
    id="advanced-toggle"
  >
    Advanced Options ▼
  </button>
  ```
- Label text: "Advanced Options" (exact copy)
- Icon: Small downward chevron (▼) that rotates 180° when expanded (CSS transform, 200ms ease)
- Positioned below primary buttons, clear visual separation (20px margin-top)
- See Mockup: Frame "Advanced Toggle Collapsed"

**[P1] USER SEES A ONE-LINE SUBTITLE REINFORCING LOCAL TRANSCRIPTION**
- **Exact text:** "Transcription runs locally on your device. No data leaves your computer."
- Rationale:
  - "runs locally" = simple, non-technical
  - "on your device" = emphasizes user control
  - "No data leaves" = directly addresses privacy concern from user needs
  - 14 words = short enough to scan in 3 seconds
- Placement: Below the drop zone, above Record/Choose File buttons (see Mockup: Frame "Hero with Subtitle")
- Color: Subtle gray text (#666666, opacity not used), not bold
- Font size: 12px, line-height 1.4

**[P2] USER CAN DISMISS EMPTY-STATE SUBTITLE AFTER FIRST SUCCESSFUL TRANSCRIPTION**
- Optional: Small "×" icon button to the right of subtitle
- Once dismissed, subtitle does not reappear in future sessions
- Store dismissal state: `localStorage.wisper_empty_state_dismissed = true`
- Deferred if time is tight; implement if ≤1 hour of dev time

#### SUB-JOURNEY: FILE IMPORT WORKFLOWS

**[P0] METHOD A: CLICK "📁 CHOOSE FILE" BUTTON**
1. User clicks "📁 Choose File" button
2. System file picker opens (platform-native: Finder on Mac, Explorer on Windows, Nautilus on Linux)
3. User selects audio file
4. File picker closes, file preview appears in hero area
5. Preview shows: `[Filename.mp3] • 2.3 MB • 5 min 30 sec`
   - Filename: truncated if >300px width, shows "...mp3" for overflow
   - Size: formatted as "2.3 MB" (rounded to 1 decimal)
   - Duration: formatted as "5 min 30 sec" or "1 hr 25 min" (if available)
6. Below preview: "[Remove file] [Transcribe]" buttons
7. User clicks "Transcribe" button
8. File validation runs (see File Size & Format Validation below)
9. If valid: Transcription begins; if invalid: error message appears inline (see Error States)

**[P0] METHOD B: DRAG & DROP DIRECTLY ONTO DROP ZONE**
1. User drags audio file from Finder/Explorer
2. Drags over drop zone:
   - Visual feedback: blue highlight border (#0066ff, 2px)
   - Drop zone opacity: +20% (more visible)
3. Releases file over drop zone
4. File preview appears (same as METHOD A)
5. User clicks "Transcribe"

**[P0] FILE PREVIEW (AFTER SELECTION)**
Display info:
- Filename (truncated if >300px: "very_long_filename_from_...mp3")
- Tooltip on hover: full filename
- File size (formatted: "2.3 MB")
- Duration (formatted: "5 min 30 sec" or "[Duration unknown]" if unavailable)
- Format badge (formatted: "[MP3]")
- Buttons: "[Remove file] [Transcribe]"
- User can click "[Remove file]" to clear selection and choose different file
- While transcription is in progress: "[Remove file]" button is DISABLED (grayed out)
- After transcription completes: "[Remove file]" button is re-enabled

**[P0] FILE SIZE & FORMAT VALIDATION**

**SUPPORTED FORMATS (Week 2):**
- ✓ MP3 (MPEG-1 Audio Layer III)
- ✓ WAV (Waveform Audio File Format)
- ✓ M4A (MPEG-4 Audio)
- ✓ FLAC (Free Lossless Audio Codec)
- ✓ OGG (Ogg Vorbis)
- ✓ OPUS (Opus Audio)

**MAX FILE SIZE:** 1 GB (approximately 3-8 hours depending on format and bitrate)

**Bitrate breakdown:**
- MP3 (128 kbps): ~18 hours
- MP3 (192 kbps): ~12 hours
- MP3 (320 kbps): ~4.6 hours
- WAV (16-bit, 44.1kHz): ~3 hours
- FLAC (lossless): ~2 hours
- Note to dev: Users will rarely hit file size limit; they'll hit patience/time limit first

**BITRATE SUPPORT:** 8 kHz - 48 kHz

**Validation timing:** File size + format checked on file selection (before user clicks Transcribe). File integrity/corruption checked after transcription attempt begins.

**Validation errors (show inline, below preview, with red icon + text):**

| Error | Message | Display |
|-------|---------|---------|
| Too large | `"File is too large (max 1 GB, ~6 hours depending on format). Try a shorter recording."` | Inline, persistent until resolved |
| Unsupported format | `"That file format isn't supported. Try MP3, WAV, M4A, FLAC, OGG, or OPUS."` | Inline, persistent |
| Video file (MP4, MKV, MOV, etc.) | `"That's a video file. Extract the audio first, or try audio-only formats like MP3 or WAV."` | Inline, persistent |
| Corrupted/unreadable (detected after transcription starts) | `"Couldn't read this file. It may be corrupted. Try a different file."` | Banner in progress region, yellow warning icon |

#### SUB-JOURNEY: COMPLETING FIRST TRANSCRIPTION WITH DEFAULTS

**[P0] USER CAN CHOOSE A FILE AND TAP TRANSCRIBE WITH LANGUAGE SET TO AUTO-DETECT AND COMPUTE ON SYSTEM DEFAULT**
- File chooser/drop workflow: as defined above
- Language defaults to "Auto-detect" (no user input required)
- Compute defaults to best available backend: GPU if CUDA/Metal available, else CPU
- User clicks "Transcribe" button → file validation runs → transcription starts
- No additional dialogs or confirmations

**[P0] USER STILL SEES TWO-STEP DOWNLOAD → TRANSCRIBE PROGRESS WHEN APPLICABLE (UNCHANGED)**
- If model is not downloaded: Download progress bar appears first
- After download completes (100%): Transcribe progress bar appears
- Existing aria-live regions remain unchanged
- Progression example: "Downloading model (12%)" → "Downloaded. Transcribing (45%)" → "✓ Complete"

**[P0] FIRST-RUN MODEL BANNER (IF MODEL IS MISSING)**
- First-run banner behavior is preserved from Phase 1
- Banner is part of hero visual unit (see 2a(i) SCENARIO B)
- Banner shows download progress with percentage + MB/Total MB
- Visual: `[████████░░░░░░░░░░░░░░░] 35% (147 MB / 420 MB)`
- Text: "Downloading transcription model. This takes 2-5 minutes on first launch."
- Estimated time shown after 30 seconds: `"About 3 minutes remaining"`
- Banner disappears after download completes (100%)
- Does not block the drop zone or primary buttons (buttons are disabled, not hidden)

**[P0] FIRST-RUN MODEL DOWNLOAD PROGRESS (DETAILED)**

Visual progress display:
```
[████████░░░░░░░░░░░░░░░] 35% (147 MB / 420 MB)
About 3 minutes remaining
```

Buttons during download:
- "[Cancel Download]" button (red text, left-aligned)
- "[Retry Download]" button (appears if download failed)
- Record + Choose File buttons are DISABLED (color: #999999, cursor: not-allowed, pointer-events: none)

If user cancels:
- Download stops
- Hero shows: "[Retry Download]" and "[Use URL Import]" buttons
- User can restart model download or try URL import

If network fails:
- Error message: `"Download interrupted. Check your internet and [Retry Download]."`
- "[Retry Download]" button is enabled
- User can retry immediately

**[P2] USER CAN PIN ADVANCED SECTION OPEN FOR FUTURE SESSIONS**
- Advanced panel has a "📌 PIN" button in top-right corner (only visible when Advanced is expanded)
- When clicked, button label changes to "📌 PINNED" (visual feedback, text color #333)
- Advanced section defaults to EXPANDED on next app launch
- Preference stored in: platform-specific config file (see Technical Approach)
- User can unpin by clicking "📌 PINNED" → returns to button "📌 PIN"
- Pin preference persists across app restarts AND reinstalls (stored in config file, not volatile localStorage)

#### SUB-JOURNEY: RECORDING WORKFLOWS

**[P0] USER CAN START RECORDING BY CLICKING "🎤 RECORD" BUTTON**
- Record button is enabled by default (unless model is downloading or URL import in progress)
- Click Record → system microphone opens (platform permission required)
- Record button label changes to: "⏹️ Stop Recording"
- Button color changes to red (#cc3333) to indicate active recording state
- All other buttons DISABLED: Choose File, Advanced toggle
- Transcribe button does NOT appear until recording stops
- Timer appears below recording button: "Recording: 0:05" (format: MM:SS, updates every 100ms)
  - 5 seconds: "0:05"
  - 1 minute: "1:00"
  - 1 hour: "60:00" (or switch to "1:00:00" if recording can exceed 1 hour)

**[P0] IF USER TRIES TO RECORD WHILE MODEL IS DOWNLOADING:**
- Record button is DISABLED (color: #999999, cursor: not-allowed, pointer-events: none)
- Tooltip: "↻ Waiting for model to download..."
- User must wait for download to complete before recording

**[P0] IF USER TRIES TO RECORD WHILE URL DOWNLOAD IS IN PROGRESS:**
- Record button is DISABLED (same as model download)
- Tooltip: "↻ URL download in progress..."
- Choose File button: ENABLED (user can import different source)

**[P0] USER STOPS RECORDING BY CLICKING "⏹️ STOP RECORDING" BUTTON**
- Recording stops
- Button returns to "🎤 Record" (black text)
- Recorded audio is saved to temp file (WAV format)
- File preview appears: `[Recording] • [Duration] • WAV`
  - Duration formatted same as file preview (e.g., "1 min 05 sec")
- Below preview: "[Re-record] [Transcribe]" buttons

**[P0] RE-RECORD WORKFLOW (AFTER STOPPING):**

User clicks "[Re-record]":
1. Previous recording is DISCARDED (deleted from temp storage, not recoverable)
2. New recording starts fresh
3. Record timer resets to 0:00
4. Record button changes to "⏹️ Stop Recording" again
5. No way to retrieve previous recording

User clicks "[Transcribe]":
1. Current recording is locked (cannot re-record during transcription)
2. Transcription begins
3. "[Re-record]" button becomes DISABLED (greyed out)
4. User sees transcription progress
5. File preview shows recording name + duration

**[P0] IF RECORDING FAILS (MICROPHONE NOT FOUND OR PERMISSION DENIED):**
- Error message: `"Microphone not detected. Check your system settings or use 'Choose File' instead."`
- Record button remains clickable (user can try again or use file import)

**[P0] WHILE RECORDING IS IN PROGRESS:**
- Advanced section (if open) automatically COLLAPSES
- Advanced toggle is DISABLED (greyed out)
- User cannot expand Advanced during recording
- All buttons except "Stop Recording" are DISABLED
- Reason: Keep focus on recording task; prevent confusion and accidental clicks

**[P2] IF RECORDING IS INTERRUPTED (APP CRASHES) - DEFERRED:**
- On app restart: DO NOT attempt to recover draft (too risky, might be corrupted)
- Start fresh (user re-records if needed)
- Week 3+: Add crash recovery with validation check

---

### USER JOURNEY 2: RETURNING USER NEEDS URL IMPORT OR GPU CONTROL

**CONTEXT:** Progressive disclosure must not trap power users. Advanced paths stay discoverable without cluttering the default view.

#### SUB-JOURNEY: EXPANDING ADVANCED OPTIONS

**[P0] USER CAN EXPAND ADVANCED TO REVEAL LANGUAGE SELECT, COMPUTE TOGGLE, AND URL IMPORT**
- Clicking "Advanced Options" button sets `aria-expanded="true"` and reveals Advanced panel
- Chevron icon rotates 180° (CSS transform, smooth animation 200ms)
- Advanced panel slides down (smooth animation 200ms)
- Advanced panel contains:
  - Language selector (dropdown, default: "Auto-detect")
  - Compute toggle (radio buttons or toggle switch: "GPU" / "CPU")
  - URL import input field (labeled "URL or YouTube Link")
  - Pin button (📌) in top-right corner
- See Mockup: Frame "Advanced Options Expanded"

**[P0] WHEN ADVANCED IS EXPANDED, STATE PERSISTS FOR THE SESSION**
- Session persistence: Use a state variable `isAdvancedExpanded` (memory only, not localStorage)
- Do NOT use localStorage for session state (too volatile)
- When app closes and reopens, Advanced is collapsed again (unless pinned)
- If user has pinned Advanced: Advanced opens in EXPANDED state by default (from config file)

**[P1] ADVANCED SECTION HAS A "📌 PIN" BUTTON. WHEN CLICKED:**
- Button location: Top-right corner of Advanced panel (when expanded, ONLY visible in expanded state)
- Position: Right 12px, Top 8px from edge of Advanced panel
- Icon: 📌 (16px)
- Button label: "📌 PIN" (initial) or "📌 PINNED" (when active)
- Button tooltip: "Pin this section to keep it open in future sessions"
- When clicked (PIN state):
  - Preference stored in config file: `~/.config/wisper/preferences.json` (Windows: AppData/Roaming, etc.)
  - Write atomically (temp file + rename) to prevent corruption
  - Button changes to "📌 PINNED" (#333 color)
  - Advanced section will default to EXPANDED on next app launch
  - Button tooltip changes to: "Unpin this section"
- To unpin: Click "📌 PINNED" again
  - Config file is updated: `wisper_advanced_pinned: false`
  - Button returns to "📌 PIN"
  - Next app launch: Advanced opens collapsed
- Persist across app restarts and reinstalls

**[P1] FIRST-TIME USER WHO HASN'T EXPANDED ADVANCED SEES HINT TEXT**
- Text: "Click 'Advanced' if you need to change language, GPU, or import from URL"
- Location: Small, subtle gray text (#999999) below Advanced toggle button
- Font size: 11px, opacity: not used (use gray color instead)
- Shows when: Advanced is collapsed AND user hasn't expanded yet AND localStorage.wisper_advanced_hinted is false
- Disappears when:
  - User clicks Advanced toggle (expands), OR
  - 10 seconds have passed (auto-hide to reduce clutter), OR
  - localStorage.wisper_advanced_hinted = true (persists dismissal)
- On next app session: Hint is hidden (localStorage persists dismissal)
- Does NOT appear inside Advanced panel (no clutter)

**[P2] SETTINGS PAGE (FUTURE): USER CAN TOGGLE "ALWAYS SHOW ADVANCED"**
- Deferred to Week 3+
- Alternative to pinning for users who prefer settings UI
- When enabled, Advanced is always expanded on launch

#### SUB-JOURNEY: URL IMPORT FROM ADVANCED

**[P0] USER CAN PASTE A URL IN AN INPUT FIELD (LABELED "URL OR YT LINK") AND CLICK "DOWNLOAD AUDIO" BUTTON**
- Input field is in Advanced section, below Compute toggle
- Label: "URL or YouTube Link"
- Placeholder text: "Paste YouTube, SoundCloud, or direct audio link..."
- Button: "Download Audio" (next to input on desktop, below on mobile <768px)
- User pastes URL and clicks button → validation runs → download begins (if valid)

**[P0] APP VALIDATES URL FORMAT AND CHECKS YT-DLP AVAILABILITY:**

Logic:
- **If URL is a direct audio file (ends in .mp3, .wav, .flac, .m4a, .ogg, .opus):**
  - Use direct HTTP download (no yt-dlp needed)
  - Show download progress (same as file import)
  - Proceed to transcription

- **If URL is from YouTube/SoundCloud/etc. (contains youtube.com, youtu.be, soundcloud.com, etc.):**
  - Check if yt-dlp binary is present on system
  - If present: Use yt-dlp to download audio, show progress
  - If missing: Show error message with installation link

- **If URL is invalid/malformed:**
  - Error message: `"That doesn't look like a valid URL. Try a YouTube link or direct audio file link."`

**[P0] DOWNLOAD/TRANSCRIBE PROGRESS AND ERROR PHASE TAGGING BEHAVE AS IN PHASE 1 (UNCHANGED)**
- Progress bars, aria-live announcements, error handling all follow Phase 1 patterns
- No new progress UI introduced in Week 2

**[P1] ERROR MESSAGE FOR MISSING YT-DLP (WEEK 2):**
```
"YouTube download requires yt-dlp. 
[Learn how to install](https://github.com/yt-dlp/yt-dlp#installation)"
```
- Decision: DO NOT auto-install or show setup wizard in Week 2
- Simply show the error with link to documentation
- User must manually install yt-dlp via terminal/pip
- Week 3+ can add: "Install now" button that runs `pip install yt-dlp` with progress feedback

**[DEFERRED TO WEEK 3]** In-app yt-dlp installer with progress feedback and verification.

---

### ACCESSIBILITY & KEYBOARD NAVIGATION (MVP)

**[P0] EMPTY-STATE HERO DROP ZONE**
- Element: `<div role="region" aria-label="Drop audio here">`
- Keyboard accessible: Not focusable (it's a region, not a control)
- Screen reader announcement on page load: "Drop audio file here or use buttons below to record or choose file. This is a local desktop app—no data uploaded."
- Implementation: Use `aria-describedby` to link to the subtitle region

**[P0] PRIMARY ACTION BUTTONS (RECORD & CHOOSE FILE)**
- Element: Native `<button>` tags (automatic keyboard support)
- Tab order: 1=Record button → 2=Choose File button → 3=Advanced toggle
- Keyboard activation: Space or Enter to trigger
- aria-describedby: Both buttons link to the "local transcription" subtitle so screen readers explain the privacy aspect
- Example:
  ```html
  <button aria-describedby="privacy-subtitle">🎤 Record</button>
  <p id="privacy-subtitle">Transcription runs locally on your device...</p>
  ```
- Button text: Include text label, not just icon (e.g., "🎤 Record" not just 🎤)

**[P0] ADVANCED TOGGLE BUTTON**
- Element: `<button>` with `aria-expanded="true|false"`
- Label: "Advanced Options" (not icon-only)
- Icon: Small chevron (▼) that rotates 180° when expanded
- Full structure:
  ```html
  <button 
    aria-expanded="false" 
    aria-controls="advanced-panel"
    id="advanced-toggle"
  >
    Advanced Options ▼
  </button>
  ```
- When expanded: `aria-expanded="true"`
- Focus: Remains on button after toggle (no automatic focus movement to panel)
- Tooltip: "Show advanced options for language, compute, and URL import"

**[P0] ADVANCED PANEL (WHEN EXPANDED)**
- Container: `<div id="advanced-panel" role="region" aria-label="Advanced options">`
- aria-live: NOT needed (toggle is user-initiated, not a page change)
- New focusable elements (Language Select, Compute Toggle, URL Input, Pin button) are automatically in tab order when revealed
- Tab order inside panel: Language dropdown → Compute radio → URL input → Pin button
- Recommendation: Focus does NOT move to first element in Advanced (user initiated the expand, not a page change)
- Focus should remain on the Advanced toggle button after expand

**[P0] PROGRESS REGIONS (DOWNLOAD & TRANSCRIBE)**
- Keep existing `aria-live="polite"` for progress updates
- Example:
  ```html
  <div aria-live="polite" aria-label="Transcription progress">
    Transcribing... 45% complete
  </div>
  ```
- Update announcements: "Download started (0%)" → "Downloaded 35% (147 MB / 420 MB)" → "Download complete. Transcribing..." → "Transcription complete"

**[P0] KEYBOARD SHORTCUTS (MVP - Week 2)**
- Tab: Move between buttons and form fields (forward)
- Shift+Tab: Move backwards between controls
- Enter/Space: Activate buttons and toggle Advanced
- Escape: Close Advanced section (if open):
  - If Advanced is expanded and focus is on toggle: Close Advanced, set `aria-expanded="false"`, focus stays on toggle
  - If focus is inside Advanced form field (URL input, language dropdown): Show confirmation "Discard unsaved URL? [Keep] [Discard]"
    - If user clicks [Keep]: Do nothing, stay in field
    - If user clicks [Discard]: Clear URL field, close Advanced, return focus to toggle
  - If focus is NOT in Advanced: Escape does nothing (no Advanced to close)

**[P1] COLOR CONTRAST (WCAG AA COMPLIANCE)**
- All text passes WCAG AA standard: 4.5:1 contrast ratio (normal text), 3:1 (large text 18pt+)
- Disabled buttons: Use gray text color (#999999), NOT opacity-based (opacity 0.5 fails contrast)
- Error messages are conveyed via text AND icon, not color alone
- Record button active state (red #cc3333) has sufficient contrast against background
- Disabled buttons remain readable (≥3:1 ratio)

**[P2] KEYBOARD SHORTCUTS (FUTURE - DEFER TO WEEK 3)**
- Ctrl+R (Cmd+R on Mac) = Start/Stop Record
- Ctrl+O (Cmd+O on Mac) = Open File Chooser
- Ctrl+Shift+A = Toggle Advanced

**[TESTING] Keyboard navigation checklist:**
- ✓ Tab through all controls in order (Record → Choose File → Advanced)
- ✓ Shift+Tab backwards through controls
- ✓ Enter/Space on buttons triggers action
- ✓ Escape closes Advanced and returns focus to toggle
- ✓ Escape in URL field shows confirmation
- ✓ Test with screen reader (NVDA on Windows, VoiceOver on Mac)
- ✓ Ensure hero is announced clearly without technical jargon
- ✓ Test expanding/collapsing Advanced with keyboard (Tab → Enter)
- ✓ Test focus management: does focus stay on button after toggle?
- ✓ Verify color contrast meets WCAG AA (4.5:1)
- ✓ Recording timer is announced by screen reader ("Recording: 1 minute 5 seconds")

---

## 4. APPENDIX

### Error States & Edge Cases (Comprehensive)

Define behavior for all error scenarios users might encounter:

**File Drop / File Chooser:**
- User drops .txt file → Show inline below preview: `"That file format isn't supported. Try MP3, WAV, M4A, FLAC, OGG, or OPUS."` (red icon + text)
- User drops 1.5GB file → Show inline: `"File is too large (max 1 GB, ~6 hours depending on format). Try a shorter recording."` (red icon)
- User drops video file (MP4, MKV) → Show inline: `"That's a video file. Extract the audio first, or try audio-only formats like MP3 or WAV."` (red icon)
- User drops corrupted MP3 → After transcription starts, show in progress region: `"Couldn't read this file. It may be corrupted. Try another file."` (yellow warning icon)

**Recording:**
- Microphone not found → Show inline: `"Microphone not detected. Check your system settings or use 'Choose File' instead."` (red icon, persistent until resolved)
- Microphone permission denied (OS-level) → Show inline: `"Wisper needs microphone permission. Check System Settings > Privacy > Microphone."` (red error, with link to OS settings)

**Model Download:**
- Network fails during download → Show banner: `"Download interrupted. Check your internet and [Retry Download]."` (yellow warning, retry button)
- Disk space too low (< 6GB free) → Show banner: `"Not enough disk space (need 5 GB free). Free up space and [Retry Download]."` (red error)
- Download takes >10 minutes → Informational message: `"Download is taking longer than expected. [Continue waiting] or [Use URL import]?"` (no action required unless user clicks button)

**URL Import (Advanced):**
- User pastes invalid URL → Show inline below input: `"That doesn't look like a valid URL. Try a YouTube link or direct audio file link."` (red icon)
- URL is YouTube but yt-dlp missing → Show inline: `"YouTube download requires yt-dlp. [Learn how to install](https://github.com/yt-dlp/yt-dlp#installation)"` (yellow warning, with link)
- URL download fails → Show banner: `"Couldn't download from that URL. Check the link and [Retry]."` (red error, retry button)
- URL responds but audio codec unsupported → Show banner: `"Downloaded file format not supported. Try MP3 or WAV sources."` (red error)

**Transcription:**
- GPU out of memory during transcription → Show banner: `"GPU ran out of memory. Restarting on CPU (slower, but will complete)."` (yellow, auto-dismiss after 5 sec OR persist until transcription completes)
  - Advanced section shows: Compute toggle changed from "GPU" to "CPU" (visual feedback)
  - User does NOT need to take action (automatic fallback)
- Language auto-detect confidence <75% → After transcription, show banner: `"Detected language: [Spanish] (70% confident). If wrong, [Open Advanced to change]."` (informational, not blocking)
- Transcription timeout (>30 min) → Show banner: `"Transcription took too long. Try a shorter file or [Try again]."` (red error)

**Error message placement & UX:**
- File validation errors: Inline below file preview, red icon + text, persistent until user removes file
- Network/download errors: Yellow warning banner at top of hero, persistent until resolved OR user dismisses
- During-transcription errors: Notification banner or inline message in progress region
- Recovery actions: Always provide "[Retry]", "[Try different approach]", or "[Open Advanced]" button when applicable

---

### GPU Fallback Behavior (Detailed)

**[P0] IF GPU RUNS OUT OF MEMORY DURING TRANSCRIPTION:**
1. Transcription is interrupted at current checkpoint
2. GPU memory is released by OS
3. Warning banner appears: `"GPU ran out of memory. Restarting on CPU (slower, but will complete)."` (yellow, 5-second display OR persist)
4. Compute toggle in Advanced section automatically changes from "GPU" to "CPU" (visual feedback, if Advanced is open)
5. Transcription restarts on CPU from the beginning (full re-process, not resumed from checkpoint)
6. User sees progress bar reset to 0% and continue (shows "CPU mode" label or icon)
7. Transcription continues to completion on CPU

**Technical note:** Current implementation restarts from beginning on CPU. Future optimization (Week 3+) could implement checkpointing to resume from GPU checkpoint on CPU.

**[P1] AFTER SUCCESSFUL CPU TRANSCRIPTION:**
- Show message: `"Transcribed on CPU. Next time, try a shorter file or adjust GPU settings in Advanced."`
- User can dismiss or auto-dismisses after 5 seconds
- Compute toggle remains on "CPU" for current session (does not reset to GPU)

**[P1] IF GPU FALLBACK FAILS AND TRANSCRIPTION ERRORS:**
- Show error: `"Transcription failed. Try a shorter file or [Contact support]."`
- User can try different file or file a support ticket

---

### Language Auto-Detection (Detailed)

**[P0] LANGUAGE AUTO-DETECT BEHAVIOR:**
- Default: Set language to "Auto-detect" (user not prompted)
- App detects language from audio automatically during transcription
- Confidence threshold: If model is ≥75% confident, proceed silently
- If model is <75% confident, after transcription completes, show banner:

  `"Detected language: [Spanish] (70% confident). If wrong, [Open Advanced to change]."`

- Placement: Appears AFTER transcription completes (below transcript preview), NOT during transcription
- Behavior: Transcription uses auto-detected language; user can manually set language in Advanced to re-transcribe if needed

**Technical note:** Confidence threshold of 75% is internal constant, configurable in future settings (not Week 2).

**[P1] USER CAN CHANGE LANGUAGE IN ADVANCED (AFTER TRANSCRIPTION):**
- Dropdown: "Language: Auto-detect | English | Spanish | French | ... | [See all languages]"
- If user changes language and clicks "Re-transcribe" button (future), audio is re-processed with new language
- Previous transcript is not deleted (can compare results)
- "Re-transcribe" feature is [P2], not Week 2 MVP

---

### Preferences File (Configuration)**

**File location (cross-platform, using Electron):**

```javascript
const prefsPath = path.join(
  app.getPath('userData'),
  'preferences.json'
);

// Resolves to:
// Windows: C:\Users\[user]\AppData\Roaming\Wisper\preferences.json
// Mac:     ~/Library/Application Support/Wisper/preferences.json
// Linux:   ~/.config/Wisper/preferences.json
```

**File schema:**

```json
{
  "wisper_advanced_pinned": boolean,          // default: false
  "wisper_empty_state_dismissed": boolean,    // default: false
  "wisper_advanced_hinted": boolean,          // default: false
  "version": "1.0"                            // for future migrations
}
```

**Example:**

```json
{
  "wisper_advanced_pinned": true,
  "wisper_empty_state_dismissed": true,
  "wisper_advanced_hinted": true,
  "version": "1.0"
}
```

**Dev notes:**
- Use atomic writes (write to temp file, then rename) to avoid corruption if app crashes during write
- Handle missing file gracefully: if file doesn't exist, create with defaults
- Handle invalid JSON: if file is corrupted, overwrite with defaults (do not crash)
- If file write fails (disk full, permission denied): log error, fall back to localStorage temporarily (non-persistent)

---

### Prioritization Context (Week 2)

- **Impact/effort matrix + MoSCoW:** Windows release CI is Must for installability; this UI improvement is Should — best impact/effort after CI.
- **Emotional pick aligned with framework #2:** Progressive disclosure, not CI (infra).
- **Technical risk:** Low (frontend-only, no API changes)
- **Timeline estimate:** 3-4 days (including testing + iteration)

---

### Technical Approach

**Files to modify:**

**[P0] `wisper/src/App.tsx`**
- Import `AdvancedOptions` component
- Import `EmptyStateHero` component (new)
- Import `ModelDownloadBanner` component (new or refactored from Phase 1)
- Conditionally render empty-state hero when `transcripts.length === 0`
- Replace three-panel layout with: Hero + Advanced toggle
- Wire up `Advanced.expanded` state to:
  - Session memory (clear on app restart) using state hook
  - localStorage for dismissal hint (`wisper_advanced_hinted`)
  - Config file for pin preference (see preferences.ts below)
- Manage file selection → file validation → preview display
- Manage recording start/stop → file creation → preview display
- Listen for model download status and disable/enable buttons accordingly
- Manage disabled/enabled state of Record, Choose File, Advanced buttons during model/URL download

**[P0] `wisper/src/styles/App.css` (or Tailwind classes)**
- Hide Compute panel in default view; show in Advanced
- Hide Language panel in default view; show in Advanced
- Hide URL import in default view; show in Advanced
- Style hero: drop zone with dashed border (2px #ccc), centered, 200px tall, flex column layout
- Style Advanced toggle: button at bottom of hero, clear label + chevron icon, 20px margin-top
- Chevron rotation: 0° when collapsed, 180° when expanded (CSS transform, 200ms transition)
- Button disabled state: color: #999999 (not opacity-based), cursor: not-allowed, pointer-events: none
- File preview: card-style with filename (truncated if >300px), size, duration, format badge
- Error messages: red icon (#c33), text #333, padding 8px, margin-top 8px, persistent until resolved
- Responsive: On mobile (<768px), buttons stack vertically (single column), 100% width
- Ensure no horizontal scrolling on 1024px+ viewports
- Drag-over state: drop zone gets blue highlight (#0066ff), opacity increased
- Recording timer: monospace font, 18px, updated every 100ms, format MM:SS

**[P0] `wisper/src/components/EmptyStateHero.tsx` (new file)**
- Render drop zone, Record button, Choose File button, Advanced toggle
- Props:
  - `onFileSelected(file)` → triggers file validation and preview
  - `onRecordStart()` → triggers recording workflow
  - `onAdvancedToggle(isOpen)` → updates aria-expanded state
  - `modelDownloading: boolean` → disables buttons if true
  - `urlDownloading: boolean` → disables Record button if true
  - `dismissedSubtitle: boolean` → hides subtitle if true
  - `onDismissSubtitle()` → stores dismissal state
- Wire up `onDrop` and `onDragOver` handlers
- Include subtitle: `"Transcription runs locally on your device. No data leaves your computer."`
- Include optional dismiss button (× icon) on subtitle (if not deferred)
- Implement file validation (size, format) with error display (inline)
- Show file preview after selection (name, size, duration, format)
- Display recording duration timer while recording (format: MM:SS)

**[P0] `wisper/src/components/AdvancedOptions.tsx` (refactor existing)**
- Move existing Language, Compute, URL controls into this component
- Props:
  - `expanded: boolean` → controls aria-expanded state
  - `onToggle()` → callback when user clicks Advanced button
  - `onPin()` → callback when user pins Advanced
  - `isPinned: boolean` → shows "📌 PINNED" or "📌 PIN" button
  - `showHint: boolean` → shows hint text below toggle
  - `onHintDismiss()` → records that hint has been seen
- Export aria-expanded and aria-controls attributes
- Pin button in top-right corner of Advanced panel (12px from edge) with tooltip
- Hint text below toggle (shows once per user, then localStorage stores `wisper_advanced_hinted`)
- Radio buttons or toggle for GPU/CPU selection
- Dropdown for Language selection (with aria-label)
- Text input for URL import with "Download Audio" button
- Collapse animation: smooth slide-up/down (200ms)

**[P0] `wisper/src/components/ModelDownloadBanner.tsx` (new or refactored)**
- Props:
  - `isDownloading: boolean`
  - `progress: number` (0-100)
  - `bytesDownloaded: number`
  - `bytesTotal: number`
  - `estimatedTimeRemaining: string` (e.g., "3 minutes")
  - `onCancel()` → callback
  - `onRetry()` → callback
- Display progress bar: `[████░░░░] 35% (147 MB / 420 MB)`
- Show estimated time remaining after 30 seconds: "About 3 minutes remaining"
- Show buttons: "[Cancel Download]" (red) or "[Retry Download]" (if failed)
- Integrate with hero (same visual unit, not separate banner) — no separator line, same background color
- Disable Record, Choose File, and Advanced buttons while downloading (handled in App.tsx)

**[P0] `wisper/src/components/FilePreview.tsx` (new file or extend)**
- Props:
  - `file: { name, size, duration, format }` → display info
  - `onRemove()` → callback to clear selection
  - `onTranscribe()` → callback to start transcription
  - `isTranscribing: boolean` → disables [Remove] button if true
- Display: filename (truncated if >300px, ellipsis), size, duration, format badge
- Tooltip on filename: full name on hover
- Buttons: "[Remove file]" [Transcribe]"
- [Remove file] button disabled while transcription in progress

**[P0] `wisper/src/utils/fileValidation.ts` (new file)**
- Function: `validateFile(file) → { isValid, error }`
- Check file size: max 1 GB
- Check file format: whitelist MP3, WAV, M4A, FLAC, OGG, OPUS
- Check file extension (basic check, format validation secondary)
- Return error messages per Error States section
- Function: `getFileDuration(file) → number | null` (duration in seconds, if available)

**[P0] `wisper/src/utils/preferences.ts` (new file)**
- Function: `loadPreferences() → { wisper_advanced_pinned, wisper_empty_state_dismissed, wisper_advanced_hinted }`
- Read from: config file via Electron `app.getPath('userData')`
- Handle missing file gracefully (return defaults)
- Handle invalid JSON (return defaults, log error)
- Function: `savePreferences(prefs)`
- Write to: config file with atomic write (temp file + rename)
- Handle disk full / permission denied (log error, don't crash)
- Use Electron API: `app.getPath('userData')`

**Files NOT modified (preserve):**
- ✓ Rust backend / transcription API
- ✓ Library schema or export formats
- ✓ Progress UI (Download → Transcribe banners stay as-is)
- ✓ First-run model download banner (refactored to integrate with hero, not removed)
- ✓ aria-live regions (preserved)
- ✓ Existing Phase 1 flows

**New dependencies:** None required (using existing libraries)

**Fallbacks:**
- If config file write fails: gracefully degrade, don't crash (log error)
- If config file read fails: return defaults
- localStorage used for dismissal states (volatile, OK for this use case)

---

### Dependencies / Blockers

- **Figma mockup (REQUIRED BEFORE DEV STARTS):** Must include frames: Empty Hero State, Hero with Subtitle, Advanced Toggle Collapsed, Advanced Options Expanded, Model Downloading, Recording in Progress, Error States. Aisling + Jimmy sign-off required.
- **Windows release CI** (separate effort): Beta testers need an installable build to validate metrics — track separately.
- **Model must be present** for transcription; in-app model download is a follow-up PRD.
- **Electron app context:** Config file path must work across Windows/Mac/Linux (use `app.getPath('userData')`, not hardcoded paths)

---

### Analytics Events (Detailed Specification)

Instrument the following events for success metric tracking:

**Event: "app_opened"**
- Fired when: App window opens and is ready for interaction
- Properties: 
  - `os` (string): "windows" | "mac" | "linux"
  - `app_version` (string): e.g., "1.0.2"
  - `has_transcripts` (boolean): true if library has ≥1 transcript
  - `screen_shown` (string): "hero" | "library" (which screen loaded)
  - `timestamp` (ISO 8601): app open time
  - `session_id` (string): unique per app open

**Event: "file_selected"**
- Fired when: User completes file picker (file selected, not cancelled)
- Properties: 
  - `file_size_mb` (number)
  - `file_format` (string): "mp3" | "wav" | "m4a" | "flac" | "ogg" | "opus"
  - `duration_seconds` (number, if available)
  - `method` (string): "click_chooser" | "drag_drop"

**Event: "file_validation_error"**
- Fired when: File validation fails (size/format)
- Properties: 
  - `error_type` (string): "too_large" | "unsupported_format" | "video_file"
  - `file_size_mb` (number)
  - `file_format` (string)

**Event: "record_started"**
- Fired when: User clicks "🎤 Record" button
- Properties: none

**Event: "record_stopped"**
- Fired when: User clicks "⏹️ Stop Recording" button
- Properties: 
  - `duration_seconds` (number)

**Event: "record_restarted"**
- Fired when: User clicks "[Re-record]" button (discards previous)
- Properties: 
  - `previous_duration_seconds` (number): duration of discarded recording

**Event: "recording_error"**
- Fired when: Recording fails
- Properties: 
  - `error_type` (string): "microphone_not_found" | "permission_denied" | "interrupted"

**Event: "transcription_started"**
- Fired when: User clicks "Transcribe" button
- Properties: 
  - `source` (string): "file" | "record" | "url"
  - `file_size_mb` (number)
  - `compute_mode` (string): "gpu" | "cpu"
  - `language` (string): "auto-detect" | specific language

**Event: "transcription_completed"**
- Fired when: Transcription finishes successfully
- Properties: 
  - `duration_seconds` (number): time taken to transcribe
  - `word_count` (number)
  - `language_detected` (string)
  - `fallback_to_cpu` (boolean): true if GPU fell back to CPU

**Event: "transcription_error"**
- Fired when: Transcription fails
- Properties: 
  - `error_type` (string): "gpu_oom" | "network" | "invalid_format" | "timeout" | "corruption"

**Event: "gpu_fallback"**
- Fired when: GPU runs out of memory and falls back to CPU
- Properties: 
  - `time_on_gpu_seconds` (number): how long GPU was used before fallback

**Event: "advanced_expanded"**
- Fired when: User clicks Advanced toggle to expand
- Properties: 
  - `was_pinned` (boolean): true if Advanced is pinned
  - `is_first_time` (boolean): true if user has never expanded Advanced

**Event: "advanced_collapsed"**
- Fired when: User clicks Advanced toggle to collapse (or presses Escape)
- Properties: 
  - `was_pinned` (boolean)

**Event: "advanced_pinned"**
- Fired when: User clicks pin button to pin Advanced
- Properties: none

**Event: "advanced_unpinned"**
- Fired when: User clicks pin button to unpin Advanced
- Properties: none

**Event: "url_download_started"**
- Fired when: User clicks "Download Audio" in URL import
- Properties: 
  - `url_domain` (string): extracted domain (e.g., "youtube.com")
  - `requires_ytdlp` (boolean): true if yt-dlp is needed

**Event: "url_download_error"**
- Fired when: URL download fails
- Properties: 
  - `error_type` (string): "ytdlp_missing" | "invalid_url" | "network_error" | "unsupported_format"

**Event: "session_abandoned"**
- Fired when: App closes without completing transcription in session
- Properties: 
  - `last_action` (string): "file_selected" | "record_started" | "transcription_started" | "transcription_error" | "idle"
  - `time_in_app_seconds` (number): how long user was in app

---

### Success Validation Plan

#### METRIC 1: First-session activation (>85% start transcription)

**Test method:** Unmoderated beta cohort (15 users minimum, instructed: "use once")
- **Objective:** Measure if users start transcription (indicates hero clarity)
- **Task:** "Transcribe one audio file or recording. Go."
- **Measure:** % who initiate ≥1 transcription (clicks Record or Choose File + uploads/records)
- **Success:** >85% (12/15 users)
- **Failure point:** User doesn't know what to do, closes app without trying
- **Track:** Time-to-start, errors hit, abandon points (via analytics events)
- **Sample size:** 15 unmoderated users (beta cohort)
- **Duration:** 1 week (Week 2)
- **Pass/Fail:** Majority (12/15) initiate ≥1 transcription
- **Note:** Changed from "complete" to "start" because transcription takes 4-15 minutes; users won't wait. "Started" is better signal of UI clarity.

#### METRIC 2: Time to first action (<30 seconds)

**Test method:** Moderated session (5 new users, each cold start)
- **Objective:** Measure if UI clarity leads to quick first action
- **Task:** "Transcribe this audio file. Go."
- **Measure:** Time from app open to "file upload complete" or "record started"
- **Pass criteria:** Median <30 seconds, min 4/5 succeed
- **Observer notes:** Where did users look first? Did they hesitate? Did they search for help?
- **Sample size:** 5 moderated sessions (first-time users only)
- **Facilitator:** Aisling or Jimmy
- **Recording:** Screen + audio preferred
- **Pass/Fail:** 4/5 succeed, median <30 seconds

#### METRIC 3: Clarity (can user state next step?)

**Test method:** Moderated session, right after opening Wisper (no task given)
- **Objective:** Measure if hero communicates purpose without explanation
- **Task:** None. Just ask: "What would you do next?"
- **Pass criteria:** 4/5 respond with "drop audio" or "click record" (verbatim or paraphrased, without reading docs)
- **Fail indicator:** "I'm not sure what this is for" or "let me read the README"
- **Sample size:** Same 5 moderated users as METRIC 2
- **Timing:** Ask right after app opens, before they interact
- **Pass/Fail:** 4/5 (80%) can state next step in <10 seconds

#### METRIC 4: Power-user regression (<2 clicks to Advanced)

**Test method:** 5 power users (use case: GPU toggle or URL import required)
- **Objective:** Ensure advanced users don't get slower
- **Task:** "Use GPU transcription" or "Import from YouTube"
- **Measure:** Clicks to reach Advanced + locate control, time elapsed
- **Pass criteria:** 100% find Advanced toggle within 2 clicks AND expand Advanced within 1 additional click, <5 seconds total
- **Expected flow:** App open → 1 click to expand Advanced → 1 click to toggle GPU
- **Sample size:** 5 power users
- **Pass/Fail:** 5/5 (100%) succeed in <5 seconds

#### METRIC 5: Advanced pin adoption (aspirational)

**Test method:** Observational (tracked via analytics)
- **Objective:** Measure if pin preference solves repeated-click friction
- **Measurement:** Among power users using Advanced 3+ times, % who click pin button
- **Success:** 75%+ of multi-use Advanced users adopt pin preference
- **Caveat:** Depends on if users return 3+ times in Week 2 (realistic but not guaranteed in beta)
- **Fallback metric:** If <3 users use Advanced 3+ times, defer to Week 3+ when user base is larger
- **Pass/Fail:** 75%+ adoption OR note "insufficient usage to measure"

#### REGRESSION: Existing Phase 1 features still work

**Test method:** Smoke tests (5 min per feature)
- **Objective:** Ensure no breaking changes
- Features to test:
  - File transcription (select file, transcribe, see result in library)
  - Recording (record 10 seconds, transcribe, see result)
  - URL import (paste URL, download, transcribe) — only if yt-dlp present
  - Language toggle (select language in Advanced, transcribe, verify correct language detected)
  - Compute toggle (select GPU/CPU in Advanced, verify toggle is reflected)
  - Export (export transcript as TXT, JSON, SRT)
- **Pass criteria:** 0 new bugs in Phase 1 flows (existing Phase 1 code works as before)
- **Regression failure:** Any Phase 1 feature broken → STOP, fix before beta release

#### ACCESSIBILITY TESTING (MVP)

**Test method:** Moderated sessions with assistive technology
- **Objective:** Ensure hero is usable without mouse and with screen reader
- **Screen reader test:** 1 blind/low-vision user
  - App opens → user navigates with screen reader (NVDA on Windows or VoiceOver on Mac)
  - Task: Transcribe one file without any sighted assistance
  - Pass: User successfully completes transcription using keyboard + screen reader only
- **Keyboard-only test:** 1 power user on keyboard
  - Task: Expand Advanced and toggle GPU using keyboard only
  - Pass: Tab order is logical, Escape closes Advanced, Enter/Space work on buttons, no focus traps
- **Color contrast test:** Automated check (axe, WAVE, or similar)
  - Pass: All text ≥4.5:1 contrast ratio (WCAG AA)
  - Error messages conveyed via text AND icon, not color alone
- **Sample size:** 1-2 users minimum (qualitative validation, not statistical)
- **Pass/Fail:** All three tests pass

#### BEFORE/AFTER SCREENSHOTS

- **Screenshot 1:** Before (three panels visible, busy layout) - Screenshot from current beta
- **Screenshot 2:** After (empty hero, Advanced collapsed) - Screenshot from Week 2 build
- **Screenshot 3:** After (Advanced expanded, GPU toggle visible) - Screenshot showing power-user flow
- **Submit to L2 framework** with context explaining Hick's Law & activation improvement

#### ANALYTICS VALIDATION

Track these events throughout Week 2 beta:
- `app_opened`: Should see steady opens if CI/distribution is working
- `file_selected` + `transcription_started`: Should see >85% start trajectory
- `transcription_error`: Should be <10% of transcriptions
- `advanced_expanded`: Should see most power users expand
- `advanced_pinned`: Should see 75%+ of repeated Advanced users pin (if they use it 3+ times)
- `session_abandoned`: Should be <30% abandonment before first action

---

### Open Questions (NOW DECIDED ✓)

✅ **Should Advanced remember expanded state across app restarts?**
→ **DECIDED:** Session persistence only (memory). If pinned via button, use config file to persist across restarts.

✅ **Mobile-style single FAB vs two primary buttons (Record / Choose file)?**
→ **DECIDED:** Two primary buttons (Record / Choose File) for clarity. No FAB pattern in Week 2.

✅ **Include a 3-step first-run coach mark overlay in Week 2 or defer?**
→ **DECIDED:** Defer to Week 3+. Hint text under Advanced toggle is enough for Week 2.

✅ **How is model download progress shown?**
→ **DECIDED:** Progress bar with percentage + MB/Total MB. Estimated time after 30 seconds. Banner integrated with hero.

✅ **What is the max file size?**
→ **DECIDED:** 1 GB (~3-8 hours depending on format). Breakdown by bitrate included.

✅ **Can user record while model is downloading?**
→ **DECIDED:** No, Record button is disabled until model download completes.

✅ **Can user expand Advanced during recording?**
→ **DECIDED:** No, Advanced section automatically collapses when recording starts.

✅ **What happens if user drops a file while model is downloading?**
→ **DECIDED:** Drop zone is disabled (pointer-events: none), no drop accepted. Help text shows.

✅ **How should localStorage handle private browsing or disabled storage?**
→ **DECIDED:** Use config file for pin preference (persists across install). Use localStorage for session state. Graceful fallback if write fails.

✅ **When should language auto-detect error show?**
→ **DECIDED:** After transcription completes (not during). If confidence <75%, show banner.

✅ **Is yt-dlp needed for all URLs or just YouTube?**
→ **DECIDED:** Direct audio files (.mp3, .wav) don't need yt-dlp. YouTube/SoundCloud do.

✅ **What about recording crash recovery?**
→ **DECIDED:** Don't attempt recovery in Week 2 (risky). User re-records if needed.

✅ **How should buttons be disabled (opacity vs color)?**
→ **DECIDED:** Use gray color (#999999), NOT opacity-based. Better accessibility.

✅ **How should file Remove button behave during transcription?**
→ **DECIDED:** Disabled (greyed out) during transcription, re-enabled after completion.

✅ **What about URL download while recording?**
→ **DECIDED:** Record button disabled while URL download in progress.

✅ **Escape key in form fields?**
→ **DECIDED:** Show confirmation if unsaved URL input, then close Advanced.

---

## SIGN-OFF CHECKLIST (Before Dev Starts)

- [ ] Aisling + Jimmy review final PRD (this document)
- [ ] Figma mockup created with all required frames (link inserted at top)
- [ ] Aisling + Jimmy sign off on mockup
- [ ] All 25 issues from senior review resolved ✓ (see Appendix notes)
- [ ] Tech lead reviews technical approach section
- [ ] QA lead reviews success validation plan
- [ ] Analytics engineer confirms events are implementable
- [ ] Accessibility lead approves a11y requirements

**Once all boxes checked: DEV CAN START** ✅

---

**Document Version:** 3.0 (Final, All Issues Resolved)
**Last Updated:** June 8, 2026
**Status:** PRODUCTION-READY
**Next Review:** Week 2 QA (June 15, 2026)

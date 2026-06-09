import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { open } from "@tauri-apps/plugin-dialog";
import "./App.css";

interface TranscriptSegment {
  start_ms: number;
  end_ms: number;
  text: string;
}

function formatTimestamp(ms: number): string {
  const totalSeconds = Math.floor(ms / 1000);
  const minutes = Math.floor(totalSeconds / 60);
  const seconds = totalSeconds % 60;
  return `${minutes}:${seconds.toString().padStart(2, "0")}`;
}

function App() {
  const [modelPath, setModelPath] = useState("");
  const [audioPath, setAudioPath] = useState<string | null>(null);
  const [segments, setSegments] = useState<TranscriptSegment[]>([]);
  const [status, setStatus] = useState("Pick an audio file to transcribe locally.");
  const [busy, setBusy] = useState(false);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    invoke<string>("get_model_path")
      .then(setModelPath)
      .catch((e) => setError(String(e)));
  }, []);

  async function pickFile() {
    setError(null);
    const selected = await open({
      multiple: false,
      filters: [
        {
          name: "Audio",
          extensions: ["wav", "mp3", "m4a", "flac", "ogg", "aac"],
        },
      ],
    });

    if (selected && typeof selected === "string") {
      setAudioPath(selected);
      setSegments([]);
      setStatus(`Selected: ${selected.split(/[/\\]/).pop()}`);
    }
  }

  async function transcribe() {
    if (!audioPath) {
      setError("Select an audio file first.");
      return;
    }

    setBusy(true);
    setError(null);
    setStatus("Transcribing on-device (no network)…");
    setSegments([]);

    try {
      const result = await invoke<TranscriptSegment[]>("transcribe_audio", {
        audioPath,
      });
      setSegments(result);
      setStatus(`Done — ${result.length} segment${result.length === 1 ? "" : "s"}.`);
    } catch (e) {
      setError(String(e));
      setStatus("Transcription failed.");
    } finally {
      setBusy(false);
    }
  }

  return (
    <main className="app">
      <header className="header">
        <div>
          <p className="eyebrow">Phase 0 · local-first</p>
          <h1>Wisper</h1>
          <p className="subtitle">
            Transcription runs entirely on your machine via whisper.cpp.
          </p>
        </div>
      </header>

      <section className="panel">
        <div className="actions">
          <button type="button" onClick={pickFile} disabled={busy}>
            Choose audio file
          </button>
          <button
            type="button"
            className="primary"
            onClick={transcribe}
            disabled={busy || !audioPath}
          >
            {busy ? "Transcribing…" : "Transcribe"}
          </button>
        </div>
        <p className="status">{status}</p>
        {error && <p className="error">{error}</p>}
      </section>

      <section className="panel model-panel">
        <h2>Whisper model</h2>
        <p className="model-path">{modelPath || "Loading…"}</p>
        <p className="hint">
          Download{" "}
          <code>ggml-large-v3-turbo.bin</code> from{" "}
          <a
            href="https://huggingface.co/ggerganov/whisper.cpp"
            target="_blank"
            rel="noreferrer"
          >
            Hugging Face
          </a>{" "}
          and place it at the path above before transcribing.
        </p>
      </section>

      {segments.length > 0 && (
        <section className="panel transcript">
          <h2>Transcript</h2>
          <ul>
            {segments.map((seg, i) => (
              <li key={`${seg.start_ms}-${i}`}>
                <span className="time">
                  {formatTimestamp(seg.start_ms)} – {formatTimestamp(seg.end_ms)}
                </span>
                <span className="text">{seg.text}</span>
              </li>
            ))}
          </ul>
        </section>
      )}
    </main>
  );
}

export default App;

<div align="center">

# OllamaGPT

**A native desktop chat client for a locally running LLM.**

Rust and Iced, rendered on the GPU. No webview, no Electron, no network —
the model runs on your machine and nothing leaves it.

<sub>Rust 1.98 · Iced 0.14 · Linux first</sub>

</div>

---

## What this is

A rewrite. The first version was Next.js talking to Ollama through an API route,
and it worked — but a chat window has no business shipping a browser engine to
draw a sidebar and a text field.

This version is a Rust workspace: a UI crate today, a headless `core` crate next,
so the expensive half never moves when the front end does.

### Why

Heavily influenced by DHH's Rails World 2026 keynote and what 37signals are doing
with HEY: the next version stops being primarily a web application and is rebuilt
as **six native applications with the core rewritten in Rust**. He reported
roughly **99% less CPU and 95% less memory** against the old stack — his own
back-of-envelope figures, but the direction is the point.

The argument that mattered here is about compute. A local LLM already wants every
cycle the machine has; the GPU is busy generating tokens and the CPU is feeding
it. Spending a large slice of what's left on a browser engine — to draw a list, a
text field and some text — is the wrong allocation. It showed up concretely in
the web version: the composer animation stuttered during generation because an
SVG blur filter was re-rasterising every frame while the model had the GPU.

The other half of his argument is that AI agents removed the old excuse for
cross-platform frameworks: if writing N native UIs is no longer prohibitive, you
stop paying a runtime tax to avoid it. This project hedges — one Rust core, one
Iced UI that can run anywhere, with the option to add a per-platform shell later
without touching the core.

```
 stage                  what it does
──────────────────────────────────────────────────────────
 1. shell        ✓      window, sidebar, composer, settings
 2. core                Ollama client, streaming, threads
 3. chat                send a message, stream the reply
 4. markdown            render replies properly
 5. storage             threads that survive a restart
```

Stage 1 is done. Nothing talks to Ollama yet.

---

## Install

Needs [Ollama](https://ollama.com) running, and a model pulled:

```bash
ollama pull qwen3:4b
```

Then:

```bash
git clone https://github.com/TomasSirotek/local-llm-chat-demo
cd local-llm-chat-demo
cargo run
```

The first build compiles ~400 crates and takes a few minutes. Every build after
is seconds.

**Linking:** `.cargo/config.toml` uses [mold](https://github.com/rui314/mold),
which takes an incremental rebuild from 20s to 3s. Install it (`pacman -S mold`)
or delete that file.

## Use

```bash
cargo run              # debug build, instant rebuilds
cargo run --release    # optimised binary
cargo build            # compile without running
```

---

## Layout

```
ui-linux/src/
├── main.rs       wiring only — 29 lines
├── app.rs        what the app IS: state, events, update
├── theme.rs      what it LOOKS like: colours and shapes
└── ui/
    ├── mod.rs    view(), and the one shared widget
    ├── icons.rs  the icon set, embedded as SVG
    ├── sidebar.rs
    ├── chat.rs
    └── settings.rs
```

Three concerns at the same level. **`ui` knows about `app`; `app` knows nothing
about `ui`.** One direction, so the entire look can change without touching a
line of state. `theme` knows about neither — it is values only, which is what
makes retinting one file.

### The architecture

Iced is [the Elm Architecture](https://guide.elm-lang.org/architecture/):

- **state** — one struct holding everything the app knows
- **`Message`** — an enum of every event that can happen
- **`update(&mut State, Message)`** — the only thing that may change state
- **`view(&State) -> Element`** — the only thing that draws it

`view` receives `&State`, not `&mut State`, so mutating from inside a render is
a compile error rather than a code review note. Adding a feature starts with a
new `Message` variant, which makes `update` fail to build until it's handled.

---

## Why not Tauri, or GPUI

**Tauri** keeps the web UI but ships WebKitGTK on Linux — the weakest of the
webviews. That trades Electron's memory for WebKit's rendering quirks and keeps
most of the CPU cost.

**GPUI** (Zed's framework) is the exciting option and the wrong one for now: as
of 2026 it is pre-1.0, unreleased as a standalone crate, and what's on crates.io
is an unofficial mirror. Building on an internal API with no docs is the kind of
risk that strands a solo project.

**Iced** is pure Rust on wgpu, so there's no FFI seam with the core, it has a
built-in markdown widget — the hot path for a chat client — and System76 ships an
entire desktop environment on it.

---

## History

The Next.js version is preserved at
[`c40c8a4`](https://github.com/TomasSirotek/local-llm-chat-demo/commit/c40c8a4),
not in an archive directory. Git history is the archive; code nobody maintains
should not sit in the tree.

`docs/billing-plan.md` survived the rewrite unchanged — it describes usage
metering and Stripe mechanics, which are the same whatever draws the window.

## License

MIT — see [LICENSE](LICENSE).

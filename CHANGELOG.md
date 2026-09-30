# Changelog

All notable changes to this project are documented here.

The format follows [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- **Native desktop shell** — Rust workspace with an [Iced](https://iced.rs) UI
  crate, rendered on the GPU through wgpu. No webview.
- **Layout** — fixed rail with brand, search and collapse controls, navigation,
  and a profile row; black chat panel with a centred empty state and a pill
  composer.
- **Profile menu** — Settings and Log out, opening upward from the avatar row.
  Built with `stack!`, since Iced has no popover primitive.
- **Settings sheet** — panel over a dimming backdrop that closes on an outside
  click. Rows are read-only until `core` owns the settings.
- **Icon set** — eleven stroke SVGs on a 24×24 grid, embedded in the binary and
  tinted per use via `stroke="currentColor"`, so one definition serves every
  colour it appears in.
- **`mold` linker config** — takes an incremental rebuild from 20s to 3s.
- **Optimised dependencies in debug builds** (`[profile.dev.package."*"]`) —
  `wgpu`, `naga` and `cosmic-text` do heavy work at startup and are 10–50×
  slower unoptimised. Costs one long build, saves it on every run.

### Removed

- **The Next.js application.** Preserved at `c40c8a4` rather than in an archive
  directory — git history is the archive, and code nobody maintains should not
  stay in the tree.
- Rail destinations `Library`, `Scheduled`, `Plugins` and `More`, which named
  nothing the app has.

### Not yet working

Nothing talks to Ollama. Send appends the typed message and no reply arrives;
the model name and endpoint in Settings are labels, not settings; `Think`, `+`
and the collapse control are inert.

---

## [0.1.0] — 2026-09-30

The web version, before the rewrite. Kept for reference; not a release anyone
installed.

### Added

- Next.js chat UI on BoardUI, streaming from a local Ollama through
  `@ai-sdk/openai-compatible`.
- `/api/chat` streaming endpoint, `/api/models` reading the pulled model list,
  `/api/analyze` demonstrating schema-enforced output.
- Single sidebar: brand, ⌘K search palette, new chat, thread history, profile.
- Model switcher backed by the live model list, with an install-guide modal.
- Markdown rendering for replies, and a reasoning panel with an elapsed timer
  and a stop button.
- `docs/billing-plan.md` — usage metering, plans, and Stripe test-mode steps.

[Unreleased]: https://github.com/TomasSirotek/local-llm-chat-demo/compare/c40c8a4...HEAD
[0.1.0]: https://github.com/TomasSirotek/local-llm-chat-demo/commit/c40c8a4

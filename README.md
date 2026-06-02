<p align="center">
  <img src="icon.png" alt="Inkwell MD" width="120" height="120" />
</p>

<h1 align="center">Inkwell MD</h1>

<p align="center">
  <strong>A clean, distraction-free Markdown editor for your desktop.</strong><br/>
  Built with Tauri v2 + React — fast, native, and beautifully minimal.
</p>

<p align="center">
  <img src="https://img.shields.io/badge/Tauri-v2-FFC131?logo=tauri&logoColor=white" alt="Tauri" />
  <img src="https://img.shields.io/badge/React-18-61DAFB?logo=react&logoColor=black" alt="React" />
  <img src="https://img.shields.io/badge/Vite-6-646CFF?logo=vite&logoColor=white" alt="Vite" />
  <img src="https://img.shields.io/badge/License-MIT-green" alt="License" />
  <img src="https://img.shields.io/badge/macOS-arm64-teal" alt="Platform" />
</p>

---

<p align="center">
  <img src="docs/screenshot.png" alt="Inkwell MD Screenshot" width="100%" />
</p>

## Why Inkwell?

Most Markdown editors try to do too much. Inkwell takes a different approach — **show only what you need, when you need it**. The sidebar appears when you're navigating, the table of contents helps when documents get long, and everything else fades away, leaving you with nothing but your words.

## Features

- **Three View Modes** — Reading, Split, and Editor. Switch with `Cmd+E`.
- **Live GFM Rendering** — Tables, task lists, strikethrough, autolinks via `remark-gfm`.
- **Syntax Highlighting** — Prism-powered code blocks that follow your theme (light/dark).
- **Inline HTML** — Renders raw HTML in Markdown; opens standalone `.html` files too.
- **Smart TOC** — Auto-generated table of contents with click-to-jump and scroll spy.
- **File Browser** — Tree view + flat path view with instant search.
- **File Watching** — Detects external file changes and auto-refreshes via Tauri's `notify` watcher.
- **Command Palette** — `Cmd+K` to search files and execute commands instantly.
- **Formatting Toolbar** — Quick-insert for Bold, Italic, Code, Links, Headings, and more.
- **Light & Dark Themes** — Seed-token design system with smooth transitions. `Cmd+/` to toggle.
- **Multi-tab Editing** — Work across multiple documents simultaneously.
- **External Link Safety** — Confirmation dialog before opening links in your browser.
- **Native Desktop App** — Lightweight `.app` / `.dmg` via Tauri, no Electron overhead.

## Keyboard Shortcuts

| Shortcut | Action |
|----------|--------|
| `Cmd+O` | Open file |
| `Cmd+Shift+O` | Open folder |
| `Cmd+E` | Cycle view modes (Read → Split → Edit) |
| `Cmd+B` | Toggle sidebar |
| `Cmd+K` | Command palette |
| `Cmd+/` | Toggle light/dark theme |
| `Cmd+S` | Save current file |

## Tech Stack

| Layer | Technology |
|-------|-----------|
| Desktop | **Tauri v2** (Rust backend) |
| Frontend | **React 18** + **Vite 6** |
| Markdown | `react-markdown` + `remark-gfm` + `rehype-slug` + `rehype-raw` |
| Code Blocks | `react-syntax-highlighter` (Prism, One Light / One Dark) |
| Icons | `lucide-react` |
| TOC Sync | `github-slugger` |
| File Watching | `notify` crate (Rust) |

## Getting Started

### Prerequisites

- [Node.js](https://nodejs.org/) 18+
- [Rust](https://www.rust-lang.org/tools/install) 1.70+ (for Tauri build)

### Development

```bash
git clone https://github.com/bitshift-byte/md-reader.git
cd md-reader
npm install
npm run dev
```

### Build Desktop App

```bash
cargo tauri build
```

Produces `.app` and `.dmg` in `src-tauri/target/release/bundle/`.

## Project Structure

```
md-reader/
├── src/
│   ├── App.jsx              # Main application component
│   ├── main.jsx             # React entry point
│   ├── styles.css           # Seed-token design system
│   ├── tauri-bridge.js      # Tauri API isolation layer
│   └── assets/              # Static assets
├── src-tauri/
│   ├── src/
│   │   ├── main.rs          # Tauri entry point
│   │   ├── lib.rs           # App builder & watcher state
│   │   └── commands.rs      # Tauri commands (read/write/watch)
│   ├── capabilities/        # Permission configuration
│   └── Cargo.toml           # Rust dependencies
├── docs/                    # Documentation & screenshots
├── icon.png                 # Application icon
├── index.html               # HTML entry point
└── vite.config.js           # Vite configuration
```

## Design System

Inkwell MD uses a **seed-token CSS custom property** system. All visual tokens (borders, shadows, surfaces, hover states) are derived from a small set of seed values via `color-mix()` and `calc()`:

```
--seed-bg        Base background
--seed-fg        Base foreground
--seed-primary   Primary UI color
--seed-accent    Accent / highlight color
--seed-surface   Elevated surfaces
--seed-radius    Border radius scale
```

Toggle between light and dark themes — every derived token transitions smoothly.

## License

[MIT](LICENSE)

---

<p align="center">
  <em>Built with care · <a href="https://github.com/bitshift-byte/md-reader">github.com/bitshift-byte/md-reader</a></em>
</p>

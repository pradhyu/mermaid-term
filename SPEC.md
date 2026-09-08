# Mermaid Terminal Viewer (`mermaid-term` / `mmview`) Spec

A fast, lightweight CLI/TUI tool written in Rust to parse and render Mermaid diagrams directly inside modern terminal emulators.

---

## 1. Problem & Objectives

- **Goal**: Render Mermaid syntax (flowcharts, sequence diagrams, class diagrams, state diagrams, ER diagrams, etc.) directly in terminal windows without requiring a separate web browser window.
- **Key Constraints & Requirements**:
  - High fidelity terminal rendering (Unicode/ASCII box drawing characters, ANSI truecolor, or terminal graphic protocols like Kitty, Sixel, iTerm2, WezTerm).
  - Fast startup and low memory footprint.
  - Interactive navigation (pan, zoom, search, fold) as well as static pipe/stdout mode (`cat diagram.mmd | mmview`).
  - Graceful fallback: Graphic protocols (Kitty/Sixel/iTerm) -> Pure text/Unicode TUI.

---

## 2. Architecture & Rendering Approaches

There are two primary approaches for rendering Mermaid in the terminal:

### Approach A: Dual-Engine Architecture (Recommended)

1. **Native Text/Unicode Render Engine**:
   - Parses a subset of Mermaid AST natively in Rust (using `nom` or `pest`).
   - Computes layout (via layout algorithms like Sugiyama framework or grid routing).
   - Draws ASCII/Unicode boxes, connectors, and labels via `ratatui` or `crossterm`.
   - **Pros**: Pure Rust, zero external dependencies, ultra-fast, perfectly selectable text.
   - **Cons**: Supporting 100% of Mermaid features is complex; limited to box/line diagrams initially.

2. **Rasterizer + Terminal Image Protocol Engine**:
   - Renders Mermaid via Headless Chromium/V8, `mermaid-cli` (`mmdc`), or embedded JS runtime (e.g. `deno_core` / `boa` / `quickjs`).
   - Converts SVG -> Raster (PNG) via `resvg` / `image`.
   - Emits image directly to terminal using modern image protocols:
     - **Kitty Graphics Protocol**
     - **iTerm2 Inline Images Protocol**
     - **Sixel Graphics**
     - **Unicode Half-block / Chafa Braille fallback**
   - **Pros**: 100% feature-complete Mermaid diagram support, pixel-perfect diagrams.
   - **Cons**: Needs a JS/rendering backend if full offline native rendering isn't bundled.

---

## 3. Proposed Feature Set

### CLI Modes
- **Pipe Mode**: `mmview diagram.mmd` or `cat diagram.mmd | mmview` (prints directly to terminal stdout).
- **Interactive TUI Mode**: `mmview -i diagram.mmd` (launches an interactive TUI).
- **Image Protocol Mode**: `mmview --protocol [kitty|sixel|iterm|ascii|auto] diagram.mmd`.
- **Export Mode**: `mmview -o diagram.png` or `mmview -o diagram.svg`.

### Interactive TUI Features (`ratatui` based)
- **Viewport Navigation**: Pan (Vim keys `hjkl` or arrow keys), Zoom (`+`/`-`), Reset (`0`).
- **Theme Switching**: Dark, Light, Nord, Catppuccin, Solarized, High-Contrast.
- **Node Search & Highlighting**: Quick regex search (`/`) for node labels.
- **File Watching**: Live-reload when editing `.mmd` files in an external editor.

---

## 4. Technology Stack

- **Core / CLI**: `clap` (CLI parser), `tokio` (async runtime), `anyhow` / `thiserror` (error handling).
- **TUI & Terminal Output**: `ratatui` + `crossterm`.
- **Terminal Graphics Protocols**: `ratatui-image` or `viuer` (Kitty, Sixel, iTerm2, WezTerm support).
- **Embedded Rendering Engine (Optional / Hybrid)**:
  - `resvg` / `usvg` for SVG rendering.
  - Native Mermaid parser (`pest` / `nom`) for Pure Unicode mode.
- **Layout & Graph Algorithms**: `petgraph` + custom Sugiyama layer layout.

---

## 5. Phased Roadmap

| Phase | Milestone | Deliverables |
| :--- | :--- | :--- |
| **Phase 1** | CLI & Terminal Graphics Protocol Pipeline | Setup Rust crate, CLI argument parsing with `clap`, integrate Mermaid headless renderer + `resvg` + `viuer` for Kitty/Sixel/iTerm2 image output. |
| **Phase 2** | Interactive TUI | Interactive viewer using `ratatui` (pan/zoom, live-reload watcher via `notify`, theme switcher). |
| **Phase 3** | Pure Rust Unicode/ASCII Engine | Native parser & graph layout engine for Flowcharts & Sequence diagrams without external JS dependencies. |

---

## 6. Open Questions & User Preferences

1. **Rendering Preference**:
   - Would you prefer **high-fidelity image-based terminal rendering** (Kitty/Sixel/iTerm2 graphics), **pure ASCII/Unicode character rendering**, or a **hybrid** that supports both?
2. **Interactive vs. Static**:
   - Do you primarily want an interactive TUI viewer (pan, zoom, live file reload) or a simple terminal output pager/CLI command?

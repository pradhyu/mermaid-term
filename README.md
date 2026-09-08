# `mermaid-term` (`mmview`)

> 🚀 **Fast, Zero-Dependency ASCII & Unicode Mermaid Diagram Viewer for your Terminal**

Written in pure Rust. No Node.js, no Chromium/Puppeteer, no external runtime required.

---

## ✨ Features

- **Zero Runtime Dependencies**: Single standalone compiled Rust binary.
- **Rich Diagram Support**:
  - **Flowcharts** (Left-to-Right `LR`, Top-to-Bottom `TD`/`TB`) with multiple shapes:
    - Rectangles: `[Label]`
    - Rounded: `(Label)`
    - Decisions / Diamonds: `{Label}`
    - Databases / Cylinders: `[(Label)]`
    - Circles: `((Label))`
  - **Sequence Diagrams**:
    - Multi-participant lifelines
    - Control flow blocks: `loop ... end`, `alt ... else ... end`
    - Annotations: `Note over ...`, `Note right of ...`
    - Synchronous & asynchronous message arrows (`->>`, `-->>`, `->`, `-->`)
  - **Class Diagrams**:
    - Classes with attributes, methods, types, and visibility (`+public`, `-private`, `#protected`)
    - Inheritance and relations (`<|--`, `*--`, `o--`, `-->`)
- **ANSI & TrueColor Highlighting**:
  - Semantic syntax highlighting for diagrams, arrows, lifelines, and loops.
  - Full support for Mermaid `style Node stroke:red,color:#ffaa00` color directives.
- **Markdown Integration**:
  - Automatically parses and renders embedded ` ```mermaid ` fences across `.md` files.
  - `--inline` mode to replace diagram fences with ASCII text blocks for terminal pagers (like `glow` or `bat`).

---

## 📦 Installation & Build

### Prerequisites
- [Rust & Cargo](https://rustup.rs/) (1.80+)

### Building from Source
```bash
git clone https://github.com/your-username/mermaid-term.git
cd mermaid-term

# Build optimized release binary
cargo build --release

# Optional: Install globally to your PATH (as mermaid-term)
cargo install --path .
```

The compiled binary is available at:
`./target/release/mermaid-term`

---

## 🛠️ Usage Guide

### 1. View Diagram from File
```bash
mermaid-term examples/sequence_auth.mmd
```

### 2. Pipe Mermaid Syntax via Stdin
```bash
mermaid-term <<< '
graph TD
    Start([User Request]) --> Check{Authorized?}
    Check -.->|Yes| Cache[(Redis Session)]
    Check ==>|No| Login[Prompt Login]
'
```

### 3. Render Diagrams Embedded in Markdown Files
```bash
# Extracts and renders all mermaid blocks in the markdown document:
mermaid-term README.md
```

### 4. Transform Markdown In-Place (`--inline`)
Replace ` ```mermaid ` code blocks inline with ASCII art (ideal for piping into Markdown CLI viewers like `glow` or `bat`):
```bash
mermaid-term --inline README.md
mermaid-term --inline docs/architecture.md | glow
```

---

## 🎨 Styling & Color Example

```mermaid
graph LR
    Alert[Critical Failure] --> Service[Alert Service] --> OK[Resolved]
    style Alert stroke:red,color:red
    style Service stroke:yellow,color:yellow
    style OK stroke:green,color:green
```

Render it directly:
```bash
mermaid-term <<< '
graph LR
    Alert[Critical Failure] --> Service[Alert Service] --> OK[Resolved]
    style Alert stroke:red,color:red
    style Service stroke:yellow,color:yellow
    style OK stroke:green,color:green
'
```

---

## 📁 Sample Examples

Explore ready-to-run examples in the [`examples/`](./examples/) folder:
- `examples/flowchart_tb.mmd` - Vertical flowchart with multiple shapes & edge styles
- `examples/flowchart_lr.mmd` - Horizontal service architecture diagram
- `examples/sequence_auth.mmd` - Authentication handshake sequence diagram
- `examples/complex_doc.md` - Complete Markdown documentation file with embedded diagrams

---

## 📄 License
MIT License

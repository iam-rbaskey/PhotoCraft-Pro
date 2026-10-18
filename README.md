<p align="center">
  <picture>
    .
    <img alt="PhotoCraft Pro" src="docs/brand/artcraft-logo.svg" width="180">
  </picture>
</p>

<h1 align="center">PhotoCraft Pro</h1>

<p align="center">
  <b>A professional, high-performance native image editor tailored exclusively for macOS &mdash; built in pure Rust.</b><br>
  Featuring Photoshop 2026-inspired Obsidian Dark styling, native Metal hardware acceleration,<br>
  AppKit top menu bar integration, layers, masks, smart filters, and lossless PSD workflows.
</p>

<p align="center">
  <img alt="macOS Exclusive" src="https://img.shields.io/badge/platform-macOS%20only-000000?style=flat-square&logo=apple&logoColor=white">
  <img alt="Apple Silicon & Intel" src="https://img.shields.io/badge/architecture-Universal%20(M1%E2%80%93M4%20%2B%20Intel)-007aff?style=flat-square">
  <img alt="Metal Accelerated" src="https://img.shields.io/badge/gpu-Metal%20%2F%20wgpu-5856d6?style=flat-square">
  <img alt="100% Rust" src="https://img.shields.io/badge/engine-100%25%20Rust-b7410e?style=flat-square&logo=rust">
  <img alt="No Electron" src="https://img.shields.io/badge/ui-native%20egui%20(no%20webview)-success?style=flat-square">
</p>

<p align="center">
  <a href="#-features--macos-focus">Features</a> ·
  <a href="#-instructions-for-use">Instructions</a> ·
  <a href="#%EF%B8%8F-building-photocraft-pro">Building</a> ·
  <a href="#-changelog">Changelog</a> ·
  <a href="#-credits--acknowledgments">Credits</a> ·
  <a href="#-license">License</a>
</p>

---

## 🍎 Features & macOS Focus

PhotoCraft Pro is tuned from the ground up specifically for macOS creative workstations:

- **Obsidian Dark Pro Interface**: Redesigned from the ground up with deep obsidian black palettes (`#18181a` panels, `#121214` canvas surrounding, `#222226` cards) and ultra-crisp hairline dividers (`#2c2c30`), matching high-end pro creative environments.
- **Modern Photoshop 2026 Iconography**: Handcrafted vector tool icons redesigned with clean, balanced geometry, subtle selection badges, and high-DPI subpixel alignment.
- **Metal GPU Compositor**: Canvas rendering driven by `wgpu` over Apple Metal, optimized for Apple Silicon (M1/M2/M3/M4) unified memory and Intel discrete GPUs for a locked 60/120 fps ProMotion display rate.
- **Native macOS AppKit Menu Bar**: True system-level top menu bar integrated via `muda`, respecting standard macOS keystroke ergonomics (`⌘N`, `⌘O`, `⌘S`, `⌘Z`, `⇧⌘Z`, `⌘J`, `⇧⌘D`).
- **Hardware Stylus & Tablet Subsystem**: Dedicated macOS AppKit event monitor (`crates/tablet/src/macos.rs`) directly capturing `NSEvent` streams for Wacom tablets and Apple Pencil (via Sidecar) with subpixel precision, tilt, and pressure dynamics.
- **Lossless Layered PSD Engine**: Open, edit, and export real Adobe Photoshop documents with layer styles, adjustment layers, text layers, vector masks, and smart objects intact.
- **Zero Electron / Zero Webview**: Built purely in native Rust on top of `egui` and `eframe`. Instantaneous launch, minimal RAM footprint, and complete privacy — your artwork never leaves your local Mac.
- **Scriptable Command Registry & MCP**: Every user action is a registered engine command accessible through the interactive UI, headless CLI, or Model Context Protocol (MCP) for autonomous agent pairing.

---

## 📖 Instructions for Use

### 1. Launching the App
Open PhotoCraft Pro from your Applications folder or launch it directly from the terminal:
```sh
# Open a blank document session
cargo run --release -p photocraft

# Open an existing PSD or image directly
cargo run --release -p photocraft -- path/to/artwork.psd
```

### 2. Workspace Overview
- **Top Options Bar**: Displays context-sensitive parameters for the currently active tool (e.g. Brush size, hardness, opacity, flow, tolerance, feathering).
- **Left Toolbar**: 67 Photoshop-standard tools organized into quick-access groups (Selection, Crop, Retouch, Paint, Vector, Type, Navigation). Long-press or click the small triangle to reveal sub-tools.
- **Center Canvas**: Interactive GPU viewport supporting fluid trackpad pinch-to-zoom, spacebar-drag panning, and canvas rotation (`R`).
- **Right Panel Dock**:
  - **Layers Panel**: Manage pixel layers, adjustment layers, groups, masks, blend modes, and opacity.
  - **Properties / Adjustments**: Live interactive controls for Curves, Levels, Hue/Saturation, Color Balance, and Exposure.
  - **History Panel**: Full snapshot state history with step-by-step undo and the History Brush.
  - **Color Picker & Swatches**: 32-bit float color sliders, HSL wheels, and real-time eyedropper loupe.

### 3. Essential macOS Shortcuts

| Action | Shortcut |
|---|---|
| New Document / Open | `⌘N` / `⌘O` |
| Save / Save As | `⌘S` / `⇧⌘S` |
| Undo / Redo | `⌘Z` / `⇧⌘Z` |
| Duplicate Layer | `⌘J` |
| Deselect / Reselect | `⌘D` / `⇧⌘D` |
| Free Transform | `⌘T` |
| Fit to Screen / 100% Zoom | `⌘0` / `⌘1` |
| Invert Adjustment / Selection | `⌘I` / `⇧⌘I` |
| Merge Layers / Stamp Visible | `⌘E` / `⇧⌥⌘E` |
| Command Palette | `⌘F` |

### 4. Headless CLI & AI Agent Automation
PhotoCraft Pro includes a headless CLI (`photocraft-cli`) to automate batch operations or interface with AI agents over the Model Context Protocol (MCP):
```sh
# Run automated headless command pipeline on an image
photocraft-cli run artwork.psd \
  --cmd filter.sharpen.smartSharpen --params '{"amount":75}' \
  --cmd layer.newAdjustmentLayer.curves --params '{"points":[[0,0],[64,48],[192,212],[255,255]]}' \
  --out processed.png

# Start the MCP server for agentic IDEs and assistants
photocraft-cli mcp
```

---

## 🛠️ Building PhotoCraft Pro

PhotoCraft Pro is built exclusively for macOS (Apple Silicon M-Series & Intel 64-bit).

### Prerequisites
1. **macOS**: macOS 12.0 Monterey or later.
2. **Xcode Command Line Tools**:
   ```sh
   xcode-select --install
   ```
3. **Rust Toolchain**: Install Rust via rustup (stable 1.80+ recommended):
   ```sh
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```
4. **macOS Apple Silicon & Intel compilation targets**:
   ```sh
   rustup target add aarch64-apple-darwin
   rustup target add x86_64-apple-darwin
   ```

### Step-by-Step Compilation

1. **Clone the repository**:
   ```sh
   git clone https://github.com/storytold/photocraft.git photocraft-pro
   cd photocraft-pro
   ```

2. **Run in development / debug mode**:
   ```sh
   cargo run -p photocraft
   ```

3. **Build the optimized release binary**:
   ```sh
   cargo build --release -p photocraft
   # Binary located at: target/release/photocraft
   ```

4. **Build a Universal macOS Binary (Apple Silicon + Intel)**:
   ```sh
   # Build for Apple Silicon (M1/M2/M3/M4)
   cargo build --release -p photocraft --target aarch64-apple-darwin

   # Build for Intel Macs
   cargo build --release -p photocraft --target x86_64-apple-darwin

   # Combine into a single Universal 2 macOS binary
   lipo -create -output target/release/photocraft-pro \
     target/aarch64-apple-darwin/release/photocraft \
     target/x86_64-apple-darwin/release/photocraft

   # Verify universal architectures
   file target/release/photocraft-pro
   # Output: Mach-O universal binary with 2 architectures: [x86_64] [arm64]
   ```

5. **Run the complete test suite**:
   ```sh
   cargo test --workspace
   ```

---

## 📋 Changelog

### Version 2026.1 (PhotoCraft Pro Release)

#### 🎨 UI & Design Modernization
- **Obsidian Dark Pro Theme**: Re-engineered UI color system featuring deep obsidian black surfaces (`#18181a` side panels, `#121214` canvas backdrop, `#222226` card backgrounds), hairline border dividers (`#2c2c30`), and `#2e80ff` Spectrum Blue accent highlights.
- **Photoshop 2026 Iconography**: Redesigned all primary tool icons with contemporary geometries, refined strokes, sub-pixel alignments, and subtle active/selection indicators.
- **Pro Branding Architecture**: Renamed app across the macOS AppKit native menu bar, window titles, panel headers, and system metadata to **PhotoCraft Pro**.
- **Refined Typography & Spacing**: Enhanced panel hierarchy, compact header styling, and subpixel font rendering for Apple Retina and 5K Studio Displays.

#### ⚡ Performance & macOS Hardware Tuning
- **Zero-Lag UI Execution**: Optimized egui widget rendering loops, eliminated frame stutter during tool switching, and tuned memory caching.
- **Metal Backend Optimization**: Streamlined wgpu Metal pipeline bindings for Apple Silicon unified memory architectures, avoiding unnecessary texture copies.
- **Sparse 256² Tile Engine**: Copy-on-write tile updates ensure instant undo/redo response even on 100+ megapixel documents.
- **Ultra-Compact Footprint**: 100% native Rust binary requiring zero webview processes, launching in milliseconds with low idle RAM consumption.

#### 🐛 Bug Fixes & Stability Hardening
- **Crash Prevention**: Strict adherence to zero-panic principles, replacing unhandled errors with graceful reporting and document preservation.
- **AppKit Native Menu Bar Sync**: Resolved menu state synchronization issues between egui command states and macOS top system menu items.
- **Tablet Input Monitor Hardening**: Fixed edge-case packet dropping in `photocraft-tablet` when switching between trackpad gestures and stylus pen input.
- **PSD Layer Style Compatibility**: Corrected rasterization and blend mode composite calculation for drop shadows, outer glows, and stroke effects.
- **Layer Mask Preview Rectification**: Fixed canvas viewport clipping anomalies when panning large masked layers.

---

## 👥 Credits & Acknowledgments

PhotoCraft Pro is created, maintained, and enhanced by **Riyanshu Baskey**, building on top of foundational open-source technology developed by the broader Rust creative tools community.

### New Features, Enhancements & Pro Engineering
- **[Riyanshu Baskey](https://github.com/iam-rbaskey)**: Designed and implemented the **PhotoCraft Pro** edition, including the Obsidian Dark UI theme, Photoshop 2026 icon redesign, macOS performance optimizations, bug fixes, AppKit native enhancements, and documentation.

### Original Creators of PhotoCraft & The Crafting Apps Family
Immense gratitude and credit are owed to the original creators and maintainers at **[Storytold](https://github.com/storytold) / [ArtCraft](https://getartcraft.com/)** and their open-source contributors for conceiving **PhotoCraft** and pioneering the Crafting Apps family:

- [**PhotoCraft**](https://getartcraft.com/apps/photocraft) &mdash; The original clean-room, pure-Rust Photoshop reimplementation: layers, masks, adjustments, filters, PSD codec, and command engine.
- [**VectorCraft**](https://getartcraft.com/apps/vectorcraft) &mdash; Vector illustration, path mathematics, and precision Bézier editing.
- [**FilmCraft**](https://getartcraft.com/apps/filmcraft) &mdash; Non-linear video editing, timeline sequencing, and multi-track audio.
- [**LightCraft**](https://getartcraft.com/apps/lightcraft) &mdash; Non-destructive RAW digital photo processing and library organization.
- [**PdfCraft**](https://getartcraft.com/apps/pdfcraft) &mdash; PDF document reading, page manipulation, and rendering in Rust.
- [**EffectCraft**](https://getartcraft.com/apps/effectcraft) &mdash; Motion graphics, procedural generation, and visual effects.
- [**DesignCraft**](https://getartcraft.com/apps/designcraft) &mdash; Desktop publishing, multi-page layout, and typography.
- [**ArtCraft**](https://getartcraft.com/) &mdash; The flagship creative studio and generative art ecosystem.

The architectural rigor established by the original authors &mdash; clean-room development, 24-crate modular layering, zero-panic philosophy, and comprehensive oracle test corpora &mdash; made PhotoCraft Pro possible.

---

## 📄 License

**Notice: PhotoCraft Pro does NOT use the MIT license.**

PhotoCraft Pro is a proprietary / custom edition. All newly implemented features, Obsidian Dark UI styles, custom vector iconography, macOS-specific optimizations, and documentation are **Copyright (c) 2026. All rights reserved.**

Portions of this software derived from the base PhotoCraft open-source project remain under the copyright of the original ArtCraft Team and PhotoCraft contributors in accordance with their respective notices.

<sub>Adobe and Photoshop are trademarks or registered trademarks of Adobe Inc. in the United States and/or other countries. PhotoCraft Pro is an independent project and is not affiliated with, sponsored by, or endorsed by Adobe Inc.</sub>

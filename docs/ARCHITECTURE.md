# ZPL Studio — Architecture & Dev Reference

Fork of [CherryGaySoda/ZebraPrintLabel](https://github.com/CherryGaySoda/ZebraPrintLabel).  
Goal: Dymo-style WYSIWYG label designer for Zebra printers. No ZPL knowledge needed — design visually, ZPL generated at print time.

**GitHub:** `elphiene/zpl-studio` · public repo  
**Deployment:** local binary only (no server, no domain)

---

## Build & Run

```bash
# Linux prerequisites (one-time)
sudo apt-get install libcups2-dev libgtk-3-dev libfontconfig-dev libexpat1-dev cmake

# Dev
cargo run

# Release
cargo build --release
./target/release/zpl-printer

# Tests
cargo test
```

**Android** (`android/` subdirectory):
```bash
cd android && ./gradlew assembleRelease
```

GitHub Actions builds Windows EXE, Linux AppImage, and Android APK on every push to `main`.

---

## Module Map

```
src/
├── main.rs              # Window setup — "ZPL Studio", 1000×600
├── app.rs               # ZplPrinterApp: mode enum, all panels, printer selector, action dispatch
├── canvas/
│   ├── mod.rs           # CanvasState, CanvasElement, TextElement, ACCENT (#ec4899), LABEL_PRESETS
│   └── zpl_gen.rs       # canvas → ZPL string (^FO, ^A font, ^FD, ^XA/^XZ)
├── ui/
│   ├── design_mode.rs   # DesignModePanel — WYSIWYG canvas, properties panel (M4, new)
│   ├── load_mode.rs     # LoadModePanel — variable form + preview (Use mode, original)
│   └── edit_mode.rs     # EditModePanel — raw ZPL code editor + preview (original)
├── zpl/
│   └── template.rs      # ZplTemplate: {{VAR}} / {{VAR:Label}} extraction + render()
├── preview/
│   └── labelary.rs      # render_preview() → DynamicImage via zpl-forge (fully offline)
├── printer/
│   ├── types.rs         # PrintJob, PrinterInfo, PrinterError
│   ├── cups.rs          # Linux: CUPS list_printers / print_raw_zpl
│   └── windows.rs       # Windows: Win32 WritePrinter()
└── persistence/
    └── file_ops.rs      # open_file_dialog, load_template, save_template
```

---

## Key Patterns

**Action dispatch:** each panel's `ui()` returns an action enum (`LoadModeAction` / `DesignModeAction` / `EditModeAction`). `app.rs` handles actions *after* the UI pass — avoids egui borrow issues.

**Canvas coordinates:** `CanvasElement` positions stored in **inches** (`egui::Pos2`). Converted to dots for ZPL (`pos_in * dpi`), to pixels for rendering (`pos_in * PX_PER_INCH` where `PX_PER_INCH = 72.0`).

**Variable syntax:** `{{NAME}}` or `{{NAME:Display Label}}`. Regex in `zpl/template.rs`. Android Kotlin port uses identical pattern.

**Preview rendering:** fully offline via `zpl-forge` crate. No internet. DPI 203/300/600. `DynamicImage` → `egui::TextureHandle`.

**Printing:** raw ZPL bytes, no driver processing. Linux = CUPS raw queue; Windows = Win32 `WritePrinter()`.

**Accent colour:** `ACCENT = Color32::from_rgb(236, 72, 153)` (`#ec4899`) — Cherrys Labs pink. Defined in `src/canvas/mod.rs`, used for selection borders, handles, active buttons.

---

## Design Decisions (locked in)

| Decision | Choice |
|---|---|
| UI model | Visual canvas — user never sees ZPL |
| App name | ZPL Studio |
| Accent colour | `#ec4899` (Cherrys Labs pink) |
| Fonts | ZPL built-in A–Z only (v0.1.0) |
| Template format | `.zlabel` JSON (new) + `.zpl` (existing, backward compat) |
| Android | Keep + maintain parity where practical |

---

## v0.1.0 Scope

**Done (M4 first demo):**
- Design mode canvas — add text, select, drag, delete
- Properties panel — content, position, font size, bold
- Label size presets + custom, DPI selector
- ZPL generation from canvas
- App renamed ZPL Studio, accent colour applied

**Remaining:**
- Image elements (import PNG/JPG, density curve adjustment)
- Clipart library (12 SVG icons from wireframe: Shipping + Animals)
- Batch print (CSV → map columns → print all)
- Templates screen (browseable .zlabel library)
- `.zlabel` save/load
- Barcode + Shape elements
- Undo/redo

---

## Linux CUPS Setup

```bash
# Add Zebra as raw queue (USB)
sudo lpadmin -p ZebraZPL -v usb://Zebra/ZTC%20ZD420-203dpi%20ZPL -E -m raw
sudo lpadmin -d ZebraZPL

# Quick test
echo "^XA^FDTest^FS^XZ" | lp -d ZebraZPL
```

See `LINUX_NOTES.md` for full troubleshooting.

---

## Wireframes

Interactive HTML wireframe (all screens): `docs/wireframe.html` — open in browser.  
ASCII wireframe reference: `docs/wireframe.md`.

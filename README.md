# ZPL Printer Tool

![Rust Build](https://github.com/CherryGaySoda/ZebraPrintLabel/workflows/Build%20Rust%20Label%20Printer/badge.svg)

**A flexible, template-based ZPL label printer for Zebra thermal printers with automatic variable detection and offline preview.**

## Features

- **Cross-Platform** - Works on Windows and Linux
- **Template System** - Load ZPL files with automatic variable detection
- **Direct ZPL Editor** - Write and edit ZPL code directly in the GUI
- **Live Preview** - Offline ZPL rendering with instant visual feedback
- **Smart Variable Detection** - Automatically finds `{{VARIABLE}}` placeholders
- **Printer Selection** - Choose from any available system printer
- **File Management** - Open and save .zpl template files
- **Two Modes:**
  - **Use Mode** - Load templates and fill in variables
  - **Edit Mode** - Write raw ZPL code directly
- **Compact Size** - ~4-6 MB executable
- **Instant Startup** - < 0.1 seconds
- **Compile-time Safety** - If it compiles, it works

## Template Variable Syntax

Templates use a simple variable placeholder system:

```zpl
{{VARIABLE_NAME}}              # Shows "VARIABLE_NAME" in UI
{{VARIABLE_NAME:Display Label}} # Shows "Display Label" in UI
```

Example:
```zpl
^XA
^FO50,50^A0N,40,40^FD{{NAME:Full Name}}^FS
^FO50,100^A0N,30,30^FD{{TITLE:Job Title}}^FS
^XZ
```

The application automatically detects these variables and creates input fields in the UI.

## Download

**Pre-built executables available from GitHub Actions:**
- Go to [Actions](../../actions)
- Click latest successful build
- Download the artifact for your platform:
  - **Windows**: `zpl-printer-windows-{sha}` (EXE)
  - **Linux**: `zpl-printer-linux-{sha}` (AppImage)

## Usage

### Use Mode (Template Mode)

1. Run `zpl-printer.exe`
2. Click **"📁 Load Template"** at the top and select a .zpl file
3. Fill in the detected variables
4. Preview updates automatically as you type
5. Select your printer from the dropdown
6. Click **"🖨 Print Label"**

### Edit Mode (Direct ZPL)

1. Run `zpl-printer.exe`
2. Click **"✏ Edit"** mode
3. Write or paste your ZPL code
4. Preview updates as you type
5. Optionally save with **"💾 Save"**
6. Select your printer and click **"🖨 Print"**

## Example Templates

See the `examples/` directory for:
- `simple_name_tag.zpl` - Basic name tag template

## Building from Source

### Prerequisites

**All Platforms:**
- [Rust](https://rustup.rs/) (latest stable)

**Linux:**
- CUPS development libraries: `sudo apt-get install libcups2-dev`
- GTK3 development libraries: `sudo apt-get install libgtk-3-dev`

**Windows:**
- Windows 10/11 with Visual Studio Build Tools

### Build

```bash
# Build release version
cargo build --release

# Run
cargo run --release

# Executable location
# Windows: ./target/release/zpl-printer.exe
# Linux:   ./target/release/zpl-printer
```

### Optimizations

The release build is optimized for size:
- `opt-level = "z"` - Optimize for size
- `lto = true` - Link Time Optimization
- `strip = true` - Strip symbols
- `codegen-units = 1` - Better optimization

Result: ~4-6 MB executable

### Linux Setup

For Linux-specific CUPS configuration and troubleshooting, see [LINUX_NOTES.md](LINUX_NOTES.md).

## Technical Details

### Stack
- **GUI:** `egui` + `eframe` - Immediate-mode UI framework
- **Printing (Windows):** Win32 API (direct FFI calls via `windows-sys`)
- **Printing (Linux):** CUPS (Common Unix Printing System via `cups` crate)
- **ZPL Rendering:** `zpl-forge` - Offline ZPL to PNG renderer
- **File Dialogs:** `rfd` - Native file picker
- **Template Parsing:** `regex` - Variable extraction
- **Language:** Rust 2021 edition

### Architecture

```
src/
├── main.rs              # Entry point
├── app.rs               # Main application state and coordination
├── printer/
│   ├── mod.rs          # Module exports (platform-specific)
│   ├── types.rs        # PrintJob, PrinterInfo, PrinterError
│   ├── windows.rs      # Windows printing (Win32 API)
│   └── cups.rs         # Linux printing (CUPS)
├── zpl/
│   ├── mod.rs          # Module exports
│   └── template.rs     # Template parsing and rendering
├── ui/
│   ├── mod.rs          # Module exports
│   ├── load_mode.rs    # Use mode panel (template + variables)
│   └── edit_mode.rs    # Edit mode panel (raw ZPL editor)
├── preview/
│   ├── mod.rs          # Module exports
│   └── labelary.rs     # Offline ZPL renderer integration
└── persistence/
    ├── mod.rs          # Module exports
    └── file_ops.rs     # File I/O and dialogs
```

### Variable Detection

The template parser uses regex to extract variables:
```rust
// Matches: {{VARIABLE}} or {{VARIABLE:Display Label}}
Regex::new(r"\{\{([A-Z_][A-Z0-9_]*)(?::([^}]+))?\}\}")
```

Variables are automatically deduplicated and sorted.

### Printing

**Windows (Win32 API via `windows-sys`):**
- `GetDefaultPrinterW()` - Enumerate printers
- `OpenPrinterW()` - Open printer handle
- `StartDocPrinterW()` - Start print job
- `WritePrinter()` - Send RAW ZPL data
- `EndDocPrinter()` + `ClosePrinter()` - Clean up

**Linux (CUPS via `cups` crate):**
- `cups::get_dests()` - Enumerate CUPS destinations
- `Printer::new()` - Open printer connection
- `printer.start_job()` - Start print job
- `job.write_all()` - Send RAW ZPL data
- `job.finish()` - Complete print job

Both implementations send raw ZPL data directly to the printer without driver processing.

### Offline Preview

Uses `zpl-forge` for rendering:
- Fully offline ZPL parsing and rendering
- No external API dependencies
- Renders at 203/300/600 DPI
- Instant visual feedback
- Works without internet connection

## Design Principles

This project follows:

**KISS (Keep It Simple, Stupid)**
- Single executable
- Clean, intuitive UI
- No complex configuration
- Works out of the box

**DRY (Don't Repeat Yourself)**
- Modular architecture
- Reusable components
- No code duplication

**YAGNI (You Aren't Gonna Need It)**
- No database
- No network features
- No user accounts
- No complex settings
- Just loads templates and prints

## GitHub Actions

Automatic builds on push:
- Compiles with `cargo build --release`
- Uploads artifact (30-day retention)
- Windows-latest runner

## Version

**2.0.0** - Template-based printer with offline preview and automatic variable detection

## License

MIT OR Apache-2.0

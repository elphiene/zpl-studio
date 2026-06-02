# ZPL Studio — Wireframe v0.1

> For Cherry's review. This is the proposed direction for the fork.
> Existing **Use** and **Edit** modes are untouched — Design mode is purely additive.

---

## What's changing

The original tool has two modes: **Use** (fill in a template's variables and print) and **Edit** (write raw ZPL code). We're keeping both exactly as they are and adding a third mode:

**Design** — a visual canvas where you build a label by placing and styling elements, with no ZPL knowledge needed. ZPL is generated invisibly at print time.

---

## Application layout

```
┌─────────────────────────────────────────────────────────────────────┐
│  ZPL Studio                                          [printer ▾] [🔄] │
├─────────────────────────────────────────────────────────────────────┤
│  [📄 Use]   [✏ Edit]   [🎨 Design]          ← mode tabs             │
├─────────────────────────────────────────────────────────────────────┤
│                                                                     │
│   (mode panel fills this area)                                      │
│                                                                     │
├─────────────────────────────────────────────────────────────────────┤
│  Ready                                                              │
└─────────────────────────────────────────────────────────────────────┘
```

---

## Design Mode layout

```
┌──────────────────────────────────────────────────────────────────────┐
│  Label: [4" × 2" ▾]  [Custom...]   DPI: [203 ▾]                     │
│  [+ Text]  [+ Image]  [+ Clipart]  │  Align: [⬛⬜⬜] [⬜⬛⬜] [⬜⬜⬛] │
│                                    │  [↕ Distribute]                 │
├────────────────────────────────────┤─────────────────────────────────┤
│                                    │                                 │
│   C A N V A S                      │   PROPERTIES                    │
│                                    │   (context panel — changes      │
│   ┌────────────────────────────┐   │    based on what's selected)    │
│   │                            │   │                                 │
│   │  ▌Hello World        ╔══╗  │   │   (nothing selected)           │
│   │                      ║🖼️ ║  │   │   Click an element to edit it  │
│   │  [Cherrys Labs logo] ╚══╝  │   │                                 │
│   │                            │   │                                 │
│   └────────────────────────────┘   │                                 │
│                                    │                                 │
├────────────────────────────────────┴─────────────────────────────────┤
│  [🖨 Print]   [💾 Save Template]   [📂 Load Template]                 │
└──────────────────────────────────────────────────────────────────────┘
```

---

## Canvas interactions

### Selecting an element

Click any element on the canvas to select it. Selected elements show a highlight border and resize handles at the corners and edges.

```
   ┌ ─ ─ ─ ─ ─ ─ ─ ─ ─ ┐
   ◆  Hello World        ◆   ← corner handles (drag to resize)
   └ ─ ─ ─ ─ ─ ─ ─ ─ ─ ┘
   ◆                     ◆
```

### Dragging

Click and drag any selected element to reposition it. Grid snapping optional (future).

### Multi-select

Click canvas background to deselect. `Ctrl+click` to add to selection. Alignment tools operate on the selection group.

### Keyboard

| Key | Action |
|---|---|
| `Delete` / `Backspace` | Remove selected element |
| Arrow keys | Nudge element 1px |
| `Ctrl+Z` | Undo |
| `Ctrl+Y` | Redo |
| `Ctrl+D` | Duplicate selected |
| `Escape` | Deselect |

---

## Properties panel — per element type

### Text element selected

```
┌─────────────────────────────┐
│  TEXT                       │
│                             │
│  Content:                   │
│  ┌─────────────────────┐   │
│  │ Hello World         │   │
│  └─────────────────────┘   │
│                             │
│  Font:    [A (Default) ▾]   │
│  Size:    [─────●───] 40    │
│  Style:   [B] [I]           │
│  Align:   [⬛][⬜][⬜]       │
│                             │
│  ┌─────────────────────┐   │
│  │ ☐ Variable field    │   │
│  │   Name: [________]  │   │
│  └─────────────────────┘   │
│                             │
│  Position: X [50 ] Y [50 ] │
│  (dots at current DPI)      │
└─────────────────────────────┘
```

**Font options** (ZPL built-in A–Z, always available on any Zebra printer):

| ID | Description |
|---|---|
| A | Standard default — clean, readable |
| B | Smaller compact font |
| D | OCR-B — barcode-adjacent use |
| E | OCR-A |
| F | Tall + narrow |
| G | Bold |
| H | Extra large |

Most designs will use A or G. We'll surface the most useful subset rather than all 26.

**Variable field toggle:** when enabled, this text element becomes a fill-in-the-blank for batch printing. A name (e.g. `Name`, `Address`) identifies the column in a CSV import.

---

### Image element selected

```
┌─────────────────────────────┐
│  IMAGE                      │
│                             │
│  [📂 Replace image...]      │
│                             │
│  Size:                      │
│  W [──●──] 1.5"             │
│  H [──●──] 1.0"             │
│  [🔒 Lock aspect ratio]     │
│                             │
│  Density curve:             │
│  Shadows  [────●──]  100%   │
│  Midtones [───●───]  50%    │
│  Highlights[──●───]  20%    │
│                             │
│  Position: X [20 ] Y [20 ] │
└─────────────────────────────┘
```

The density curve adjusts how the image converts to 1-bit (black/white) for thermal printing. Thermal printers spread dots — without adjustment, photos print too dark. Pulling highlights left brightens, midtones controls the overall grey threshold.

---

### Clipart element selected

```
┌─────────────────────────────┐
│  CLIPART                    │
│                             │
│  ┌────┐ ┌────┐ ┌────┐       │
│  │ ★  │ │ ♥  │ │ ✿  │  ←   │
│  └────┘ └────┘ └────┘  scroll│
│  ┌────┐ ┌────┐ ┌────┐       │
│  │ ⬟  │ │ 🏷 │ │ ✂  │       │
│  └────┘ └────┘ └────┘       │
│                             │
│  Size:                      │
│  W [──●──] 0.5"             │
│  H [──●──] 0.5"             │
│  [🔒 Lock aspect ratio]     │
│                             │
│  Position: X [10 ] Y [10 ] │
└─────────────────────────────┘
```

Clipart is embedded in the app — no internet needed. Clicking a piece in the panel swaps it on the canvas. Categories TBD with Cherry (see open questions).

---

## Label size presets

Dropdown at the top of Design mode:

| Preset | Dimensions |
|---|---|
| 4" × 2" (shipping) | 4.00 × 2.00 in |
| 4" × 3" | 4.00 × 3.00 in |
| 4" × 6" (large shipping) | 4.00 × 6.00 in |
| 2" × 1" (small product) | 2.00 × 1.00 in |
| 2.25" × 1.25" | 2.25 × 1.25 in |
| 3" × 2" | 3.00 × 2.00 in |
| Custom… | opens W / H number inputs |

---

## Batch print flow

For users who need to print the same label template with different data per label (e.g. name badges, shipping labels from a spreadsheet).

### Step 1 — Import CSV

```
┌─────────────────────────────────────────────────────┐
│  Batch Print                                        │
│                                                     │
│  ┌─────────────────────────────────────────────┐   │
│  │  📄 names.csv                               │   │
│  │     3 columns detected, 42 rows             │   │
│  └─────────────────────────────────────────────┘   │
│                                                     │
│  [📂 Choose different file]                         │
└─────────────────────────────────────────────────────┘
```

### Step 2 — Map columns to variable fields

```
┌─────────────────────────────────────────────────────┐
│  Map data to label fields                           │
│                                                     │
│  Label field       CSV column                       │
│  ─────────────     ────────────                     │
│  Name          →   [First Name ▾]                   │
│  Address       →   [Street     ▾]                   │
│  Tracking ID   →   [Order #    ▾]                   │
│                                                     │
│  Preview (row 1):                                   │
│  ┌──────────────────┐                               │
│  │ Jane Smith       │                               │
│  │ 42 Baker St      │                               │
│  │ ORD-00192        │                               │
│  └──────────────────┘                               │
│                                                     │
│  [← Back]                    [Print 42 labels →]   │
└─────────────────────────────────────────────────────┘
```

### Step 3 — Print

Prints one label per CSV row using the current printer. Progress shown in the status bar.

---

## Template save format

Design-mode templates save as `.zlabel` (JSON). This is separate from `.zpl` — both formats are supported on load.

```json
{
  "version": 1,
  "label": {
    "width_in": 4.0,
    "height_in": 2.0,
    "dpi": 203
  },
  "elements": [
    {
      "type": "text",
      "id": 1,
      "x": 0.25,
      "y": 0.25,
      "content": "Hello World",
      "font": "A",
      "size": 40,
      "bold": false,
      "italic": false,
      "align": "left",
      "is_variable": false
    },
    {
      "type": "image",
      "id": 2,
      "x": 2.5,
      "y": 0.5,
      "width": 1.0,
      "height": 1.0,
      "data_base64": "...",
      "density_curve": [1.0, 0.75, 0.5, 0.25, 0.1]
    }
  ]
}
```

Existing `.zpl` files open in Use mode and Edit mode as before, unchanged.

---

## What's NOT in v0.1.0

- Barcodes / QR codes (high value, but complex ZPL — v0.2)
- TTF font upload (built-in ZPL fonts only for now)
- Cloud sync / template sharing
- Print history
- Undo beyond the current session
- Wi-Fi printer discovery on Android (desktop only for v0.1.0)

---

## Open questions for Cherry

1. **Clipart categories and style** — what fits the Cherrys Labs aesthetic? (Stars, hearts, floral, labels, craft icons?) Aiming for ~15–20 pieces in the initial library.
2. **Colour palette** — the app is dark mode. What specific Cherrys Labs colours should we use for the UI accent (buttons, selected element highlight, etc.)?
3. **App name** — repo is `zpl-studio`, but what should the title bar and about screen say? "ZPL Studio"? Something else?
4. **Logo** — LOGO_INSTRUCTIONS.txt mentions using the Zebra label printer logo. Is that still the plan, or do you want something custom?

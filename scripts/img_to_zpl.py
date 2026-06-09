#!/usr/bin/env python3
"""
Convert an image to a ZPL label ready for Zebra thermal printing.

Supports two modes:
  dither (default) — Floyd-Steinberg error diffusion. Best for photos,
                     gradients, and logos with grey tones.
  threshold        — Hard 50% cut. Best for pure black/white logos where
                     you want crisp edges with no dot noise.

Usage:
    python3 img_to_zpl.py <image> [options]

Options:
    -w, --width    Label width in inches  (default: 4.0)
    -h, --height   Label height in inches (default: 2.0)
    -d, --dpi      Printer DPI            (default: 203)
    -m, --mode     dither | threshold     (default: dither)
    -t, --threshold  0-255, threshold cut (default: 128, threshold mode only)
    -s, --sharpen  Apply unsharp mask before dithering (recommended for logos)
    -o, --output   Output .zpl path       (default: <image>.zpl)

Examples:
    # Logo with grey tones — dither + sharpen
    python3 img_to_zpl.py logo.png -w 4 -h 2 --sharpen

    # Pure black/white logo — hard threshold for crisp edges
    python3 img_to_zpl.py logo.png -w 4 -h 2 --mode threshold

    # 4x6 shipping label at 203dpi
    python3 img_to_zpl.py label.png -w 4 -h 6
"""

import sys
import math
import argparse
from pathlib import Path

try:
    from PIL import Image, ImageFilter, ImageEnhance
except ImportError:
    sys.exit("Pillow not installed.  Run:  pip install pillow")


def convert(image_path, label_w_in, label_h_in, dpi, mode, threshold, sharpen):
    img = Image.open(image_path).convert("RGBA")

    # Flatten transparency onto white background
    bg = Image.new("RGBA", img.size, (255, 255, 255, 255))
    bg.paste(img, mask=img.split()[3])
    img = bg.convert("L")  # grayscale

    # Resize to label dimensions (fit inside, preserve aspect ratio)
    target_w = int(label_w_in * dpi)
    target_h = int(label_h_in * dpi)
    img.thumbnail((target_w, target_h), Image.LANCZOS)

    # Optional sharpening — recommended for logos to preserve hard edges
    # after the resize step blurs them slightly
    if sharpen:
        img = img.filter(ImageFilter.UnsharpMask(radius=1.5, percent=180, threshold=3))

    # Convert to 1-bit
    if mode == "threshold":
        img_1bit = img.point(lambda p: 0 if p < threshold else 255)
        img_1bit = img_1bit.convert("1")
    else:
        img_1bit = img.convert("1", dither=Image.Dither.FLOYDSTEINBERG)

    w, h = img_1bit.size
    bytes_per_row = math.ceil(w / 8)
    pixels = img_1bit.load()

    # Pack into ZPL hex — bit 1 = black dot (print)
    rows = []
    for y in range(h):
        row = bytearray(bytes_per_row)
        for xb in range(bytes_per_row):
            byte = 0
            for bit in range(8):
                x = xb * 8 + bit
                if x < w and pixels[x, y] == 0:  # PIL 1-bit: 0 = black
                    byte |= 0x80 >> bit
            row[xb] = byte
        rows.append(row.hex().upper())

    data = "".join(rows)
    total = bytes_per_row * h

    # Centre image on label
    label_px_w = int(label_w_in * dpi)
    label_px_h = int(label_h_in * dpi)
    x_off = (label_px_w - w) // 2
    y_off = (label_px_h - h) // 2

    zpl = (
        f"^XA\n"
        f"^PW{label_px_w}\n"
        f"^LL{label_px_h}\n"
        f"^FO{x_off},{y_off}\n"
        f"^GFA,{total},{total},{bytes_per_row},{data}\n"
        f"^XZ"
    )
    return zpl, w, h


def main():
    parser = argparse.ArgumentParser(
        description="Convert image to ZPL label with dithering",
        formatter_class=argparse.RawDescriptionHelpFormatter,
        epilog=__doc__.split("Usage:")[0].strip(),
    )
    parser.add_argument("image", help="Input image (PNG, JPG, etc.)")
    parser.add_argument("-w", "--width",  type=float, default=4.0, metavar="IN",
                        help="Label width in inches (default: 4.0)")
    parser.add_argument("-H", "--height", type=float, default=2.0, metavar="IN",
                        help="Label height in inches (default: 2.0)")
    parser.add_argument("-d", "--dpi",    type=int,   default=203,
                        help="Printer DPI (default: 203)")
    parser.add_argument("-m", "--mode",   choices=["dither", "threshold"],
                        default="dither",
                        help="Conversion mode (default: dither)")
    parser.add_argument("-t", "--threshold", type=int, default=128,
                        help="Threshold 0-255 (threshold mode only, default: 128)")
    parser.add_argument("-s", "--sharpen", action="store_true",
                        help="Apply unsharp mask before dithering (recommended for logos)")
    parser.add_argument("-o", "--output", metavar="FILE",
                        help="Output .zpl path (default: <image>.zpl)")

    args = parser.parse_args()

    out_path = args.output or str(Path(args.image).with_suffix(".zpl"))
    zpl, img_w, img_h = convert(
        args.image, args.width, args.height, args.dpi,
        args.mode, args.threshold, args.sharpen,
    )

    Path(out_path).write_text(zpl)
    size_kb = Path(out_path).stat().st_size / 1024

    print(f"Output : {out_path}  ({size_kb:.0f} KB)")
    print(f"Label  : {args.width}\" × {args.height}\" @ {args.dpi} dpi")
    print(f"Image  : {img_w} × {img_h} dots  (centred on label)")
    print(f"Mode   : {args.mode}" + (f"  sharpen=yes" if args.sharpen else ""))
    print()
    print("Next steps:")
    print("  1. Open ZPL Studio → Edit ZPL tab")
    print("  2. Load Template → select the .zpl file")
    print("  3. Select your Zebra printer and click Print")


if __name__ == "__main__":
    main()

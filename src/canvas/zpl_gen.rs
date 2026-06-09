use super::{CanvasElement, CanvasState, ZplFont};

pub fn generate(canvas: &CanvasState) -> String {
    let mut out = String::from("^XA\n");

    let width_dots = (canvas.label_width_in * canvas.dpi as f32) as u32;
    let height_dots = (canvas.label_height_in * canvas.dpi as f32) as u32;
    out.push_str(&format!("^PW{}\n^LL{}\n", width_dots, height_dots));

    for elem in &canvas.elements {
        match elem {
            CanvasElement::Text(t) => {
                let x = (t.pos.x * canvas.dpi as f32) as u32;
                let y = (t.pos.y * canvas.dpi as f32) as u32;
                // 1pt = 1/72 inch; dots = pt * dpi / 72
                let h = ((t.font_size as f32 * canvas.dpi as f32 / 72.0) as u32).max(4);
                // Natural width ~55% of height for ^A0N; other fonts keep same ratio
                let w = (h * 55 / 100).max(4);
                let letter = t.font.zpl_letter();
                // Font G is inherently bold; bold flag also triggers double-print
                let do_bold = t.bold || t.font == ZplFont::G;
                if do_bold {
                    out.push_str(&format!(
                        "^FO{},{}\n^A{}N,{},{}\n^FD{}^FS\n^FO{},{}\n^A{}N,{},{}\n^FD{}^FS\n",
                        x, y, letter, h, w, t.content,
                        x + 1, y, letter, h, w, t.content
                    ));
                } else {
                    out.push_str(&format!(
                        "^FO{},{}\n^A{}N,{},{}\n^FD{}^FS\n",
                        x, y, letter, h, w, t.content
                    ));
                }
            }
            CanvasElement::Clipart(c) => {
                let x = (c.pos.x * canvas.dpi as f32) as u32;
                let y = (c.pos.y * canvas.dpi as f32) as u32;
                let w_dots = ((c.width_in * canvas.dpi as f32) as u32).max(1);
                let h_dots = ((c.height_in * canvas.dpi as f32) as u32).max(1);
                if let Some(gf) = png_to_gf(c.png_bytes, w_dots, h_dots) {
                    out.push_str(&format!("^FO{},{}\n{}\n", x, y, gf));
                }
            }
            CanvasElement::Image(i) => {
                let x = (i.pos.x * canvas.dpi as f32) as u32;
                let y = (i.pos.y * canvas.dpi as f32) as u32;
                let w_dots = ((i.width_in * canvas.dpi as f32) as u32).max(1);
                let h_dots = ((i.height_in * canvas.dpi as f32) as u32).max(1);
                if let Some(gf) = png_to_gf_with_curve(
                    &i.png_bytes, w_dots, h_dots,
                    i.shadows, i.midtones, i.highlights, i.use_dither,
                ) {
                    out.push_str(&format!("^FO{},{}\n{}\n", x, y, gf));
                }
            }
        }
    }

    out.push_str("^XZ");
    out
}

fn png_to_gf(png_bytes: &[u8], target_w: u32, target_h: u32) -> Option<String> {
    png_to_gf_with_curve(png_bytes, target_w, target_h, 0, 1.0, 255, false)
}

fn png_to_gf_with_curve(
    png_bytes: &[u8],
    target_w: u32, target_h: u32,
    shadows: u8, midtones: f32, highlights: u8,
    use_dither: bool,
) -> Option<String> {
    let img = image::load_from_memory(png_bytes).ok()?;
    let img = img.resize_exact(target_w, target_h, image::imageops::FilterType::Lanczos3);
    let gray = img.to_luma8();

    let w = target_w as usize;
    let h = target_h as usize;

    // Apply levels curve to a float buffer
    let mut buf: Vec<f32> = Vec::with_capacity(w * h);
    for y in 0..h {
        for x in 0..w {
            let raw = gray.get_pixel(x as u32, y as u32)[0];
            buf.push(apply_levels(raw, shadows, midtones, highlights) as f32);
        }
    }

    // Quantise to 1-bit — Floyd-Steinberg or hard threshold
    let dots = if use_dither {
        floyd_steinberg(&mut buf, w, h)
    } else {
        buf.iter().map(|&v| v < 128.0).collect()
    };

    // Pack bits into ZPL hex
    let bytes_per_row = (target_w + 7) / 8;
    let total_bytes = bytes_per_row * target_h;
    let mut hex = String::with_capacity(total_bytes as usize * 2);

    for y in 0..h {
        for byte_idx in 0..(bytes_per_row as usize) {
            let mut byte_val: u8 = 0;
            for bit in 0..8usize {
                let x = byte_idx * 8 + bit;
                if x < w && dots[y * w + x] {
                    byte_val |= 0x80 >> bit;
                }
            }
            hex.push_str(&format!("{:02X}", byte_val));
        }
    }

    Some(format!("^GFA,{},{},{},{}", total_bytes, total_bytes, bytes_per_row, hex))
}

/// Floyd-Steinberg error diffusion — distributes quantisation error to neighbours.
/// Returns a flat vec of booleans: true = print dot (dark pixel).
fn floyd_steinberg(buf: &mut Vec<f32>, w: usize, h: usize) -> Vec<bool> {
    let mut dots = vec![false; w * h];
    for y in 0..h {
        for x in 0..w {
            let old = buf[y * w + x];
            let new = if old < 128.0 { 0.0 } else { 255.0 };
            dots[y * w + x] = new < 128.0;
            let err = old - new;
            if err == 0.0 { continue; }
            if x + 1 < w          { buf[y * w + x + 1]       = (buf[y * w + x + 1]       + err * 7.0 / 16.0).clamp(0.0, 255.0); }
            if y + 1 < h {
                if x > 0           { buf[(y+1) * w + x - 1]   = (buf[(y+1) * w + x - 1]   + err * 3.0 / 16.0).clamp(0.0, 255.0); }
                                     buf[(y+1) * w + x]         = (buf[(y+1) * w + x]         + err * 5.0 / 16.0).clamp(0.0, 255.0);
                if x + 1 < w       { buf[(y+1) * w + x + 1]   = (buf[(y+1) * w + x + 1]   + err * 1.0 / 16.0).clamp(0.0, 255.0); }
            }
        }
    }
    dots
}

/// Input-levels: maps pixel through (shadows black point, midtones gamma, highlights white point).
fn apply_levels(pixel: u8, shadows: u8, midtones: f32, highlights: u8) -> u8 {
    if highlights <= shadows { return if pixel < shadows { 0 } else { 255 }; }
    let range = highlights as f32 - shadows as f32;
    let n = ((pixel as f32 - shadows as f32) / range).clamp(0.0, 1.0);
    (n.powf(1.0 / midtones.clamp(0.1, 5.0)) * 255.0).round() as u8
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::canvas::{CanvasElement, CanvasState, TextElement, ZplFont};
    use eframe::egui;

    #[test]
    fn test_generate_empty() {
        let canvas = CanvasState::new(4.0, 6.0, 203);
        let zpl = generate(&canvas);
        assert!(zpl.starts_with("^XA"));
        assert!(zpl.ends_with("^XZ"));
        assert!(zpl.contains("^PW812"));
        assert!(zpl.contains("^LL1218"));
    }

    #[test]
    fn test_generate_text_element() {
        let mut canvas = CanvasState::new(4.0, 6.0, 203);
        canvas.elements.push(CanvasElement::Text(TextElement {
            id: 1,
            pos: egui::pos2(0.5, 0.5),
            content: "Hello".to_string(),
            font_size: 36,
            bold: false,
            font: ZplFont::A,
        }));
        let zpl = generate(&canvas);
        assert!(zpl.contains("^FO101,101"));
        assert!(zpl.contains("^A0N,101,55"));
        assert!(zpl.contains("^FDHello^FS"));
    }

    #[test]
    fn test_generate_bold_text_element() {
        let mut canvas = CanvasState::new(4.0, 6.0, 203);
        canvas.elements.push(CanvasElement::Text(TextElement {
            id: 1,
            pos: egui::pos2(0.5, 0.5),
            content: "Bold".to_string(),
            font_size: 36,
            bold: true,
            font: ZplFont::A,
        }));
        let zpl = generate(&canvas);
        assert!(zpl.contains("^FO101,101"));
        assert!(zpl.contains("^FO102,101"));
        assert_eq!(zpl.matches("^FDBold^FS").count(), 2);
    }

    #[test]
    fn test_font_g_is_bold() {
        let mut canvas = CanvasState::new(4.0, 6.0, 203);
        canvas.elements.push(CanvasElement::Text(TextElement {
            id: 1,
            pos: egui::pos2(0.5, 0.5),
            content: "Wide".to_string(),
            font_size: 36,
            bold: false,
            font: ZplFont::G,
        }));
        let zpl = generate(&canvas);
        assert_eq!(zpl.matches("^FDWide^FS").count(), 2);
        assert!(zpl.contains("^AGN"));
    }

    #[test]
    fn test_pt_to_dots_conversion() {
        let mut canvas = CanvasState::new(4.0, 6.0, 600);
        canvas.elements.push(CanvasElement::Text(TextElement {
            id: 1,
            pos: egui::pos2(0.0, 0.0),
            content: "Hi".to_string(),
            font_size: 12,
            bold: false,
            font: ZplFont::A,
        }));
        let zpl = generate(&canvas);
        assert!(zpl.contains("^A0N,100,55"));
    }
}

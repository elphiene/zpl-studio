use super::{CanvasElement, CanvasState};

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
                // Convert pt → dots: 1pt = 1/72 inch; dots = pt * dpi / 72
                let h = ((t.font_size as f32 * canvas.dpi as f32 / 72.0) as u32).max(4);
                // ^A0N = scalable CG Triumvirate font. Natural width is ~55% of height;
                // h==h looks condensed/bold. w=0 is auto-proportion but breaks zpl-forge preview.
                let w = (h * 55 / 100).max(4);
                if t.bold {
                    // Simulate bold by printing the field twice with a 1-dot x-offset
                    out.push_str(&format!(
                        "^FO{},{}\n^A0N,{},{}\n^FD{}^FS\n^FO{},{}\n^A0N,{},{}\n^FD{}^FS\n",
                        x, y, h, w, t.content,
                        x + 1, y, h, w, t.content
                    ));
                } else {
                    out.push_str(&format!(
                        "^FO{},{}\n^A0N,{},{}\n^FD{}^FS\n",
                        x, y, h, w, t.content
                    ));
                }
            }
        }
    }

    out.push_str("^XZ");
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::canvas::{CanvasState, CanvasElement, TextElement};
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
            font_size: 36, // 36pt @ 203dpi = 101 dots
            bold: false,
        }));
        let zpl = generate(&canvas);
        assert!(zpl.contains("^FO101,101"));
        assert!(zpl.contains("^A0N,101,55")); // w = 101*55/100 = 55
        assert!(zpl.contains("^FDHello^FS"));
    }

    #[test]
    fn test_generate_bold_text_element() {
        let mut canvas = CanvasState::new(4.0, 6.0, 203);
        canvas.elements.push(CanvasElement::Text(TextElement {
            id: 1,
            pos: egui::pos2(0.5, 0.5),
            content: "Bold".to_string(),
            font_size: 36, // 36pt @ 203dpi = 101 dots
            bold: true,
        }));
        let zpl = generate(&canvas);
        // Bold prints the field twice; second pass is offset by 1 dot in x
        assert!(zpl.contains("^FO101,101"));
        assert!(zpl.contains("^FO102,101"));
        assert_eq!(zpl.matches("^FDBold^FS").count(), 2);
    }

    #[test]
    fn test_pt_to_dots_conversion() {
        let mut canvas = CanvasState::new(4.0, 6.0, 600);
        canvas.elements.push(CanvasElement::Text(TextElement {
            id: 1,
            pos: egui::pos2(0.0, 0.0),
            content: "Hi".to_string(),
            font_size: 12, // 12pt @ 600dpi = 100 dots
            bold: false,
        }));
        let zpl = generate(&canvas);
        assert!(zpl.contains("^A0N,100,55")); // w = 100*55/100 = 55
    }
}

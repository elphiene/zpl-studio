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
                // ZPL font: A=standard, B=bold-ish (using height for both width and height)
                let font = if t.bold { "B" } else { "A" };
                out.push_str(&format!(
                    "^FO{},{}\n^A{}N,{},{}\n^FD{}^FS\n",
                    x, y, font, t.font_size, t.font_size, t.content
                ));
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
            font_size: 30,
            bold: false,
        }));
        let zpl = generate(&canvas);
        assert!(zpl.contains("^FO101,101"));
        assert!(zpl.contains("^FDHello^FS"));
    }
}

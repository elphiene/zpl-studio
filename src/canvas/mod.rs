pub mod zpl_gen;

use eframe::egui;

pub const ACCENT: egui::Color32 = egui::Color32::from_rgb(236, 72, 153);

pub const LABEL_PRESETS: &[(&str, f32, f32)] = &[
    ("4\" × 6\" (shipping)",    4.0, 6.0),
    ("4\" × 2\"",               4.0, 2.0),
    ("4\" × 3\"",               4.0, 3.0),
    ("2\" × 1.2\"",             2.0, 1.2),
    ("2\" × 1\" (product tag)", 2.0, 1.0),
    ("2.25\" × 1.25\"",         2.25, 1.25),
    ("3\" × 2\"",               3.0, 2.0),
];

#[derive(Clone, Debug, PartialEq, Default)]
pub enum ZplFont {
    #[default]
    A,
    B,
    D,
    E,
    G,
    H,
}

impl ZplFont {
    pub fn label(&self) -> &'static str {
        match self {
            ZplFont::A => "A — Standard",
            ZplFont::B => "B — Compact",
            ZplFont::D => "D — OCR-B",
            ZplFont::E => "E — OCR-A",
            ZplFont::G => "G — Bold",
            ZplFont::H => "H — Large",
        }
    }

    pub fn zpl_letter(&self) -> &'static str {
        match self {
            ZplFont::A => "0",
            ZplFont::B => "B",
            ZplFont::D => "D",
            ZplFont::E => "E",
            ZplFont::G => "G",
            ZplFont::H => "H",
        }
    }

    pub fn all() -> &'static [ZplFont] {
        &[ZplFont::A, ZplFont::B, ZplFont::D, ZplFont::E, ZplFont::G, ZplFont::H]
    }
}

#[derive(Clone, Debug)]
pub struct TextElement {
    pub id: u64,
    pub pos: egui::Pos2,  // inches from top-left
    pub content: String,
    pub font_size: u32,   // points; converted to dots in zpl_gen
    pub bold: bool,
    pub font: ZplFont,
}

#[derive(Clone, Debug)]
pub struct ClipartElement {
    pub id: u64,
    pub pos: egui::Pos2,
    pub width_in: f32,
    pub height_in: f32,
    pub clipart_id: &'static str,
    pub clipart_label: &'static str,
    pub png_bytes: &'static [u8],
    pub lock_aspect: bool,
}

#[derive(Clone, Debug)]
pub enum CanvasElement {
    Text(TextElement),
    Clipart(ClipartElement),
}

impl CanvasElement {
    pub fn id(&self) -> u64 {
        match self {
            CanvasElement::Text(t) => t.id,
            CanvasElement::Clipart(c) => c.id,
        }
    }

    pub fn pos(&self) -> egui::Pos2 {
        match self {
            CanvasElement::Text(t) => t.pos,
            CanvasElement::Clipart(c) => c.pos,
        }
    }

    pub fn set_pos(&mut self, pos: egui::Pos2) {
        match self {
            CanvasElement::Text(t) => t.pos = pos,
            CanvasElement::Clipart(c) => c.pos = pos,
        }
    }
}

pub struct CanvasState {
    pub elements: Vec<CanvasElement>,
    pub selected_id: Option<u64>,
    next_id: u64,
    pub label_width_in: f32,
    pub label_height_in: f32,
    pub dpi: u16,
    pub preset_index: usize,
}

impl Default for CanvasState {
    fn default() -> Self {
        Self::new(4.0, 6.0, 203)
    }
}

impl CanvasState {
    pub fn new(width_in: f32, height_in: f32, dpi: u16) -> Self {
        Self {
            elements: Vec::new(),
            selected_id: None,
            next_id: 1,
            label_width_in: width_in,
            label_height_in: height_in,
            dpi,
            preset_index: 0,
        }
    }

    pub fn add_text(&mut self) {
        let offset = (self.elements.len() as f32) * 0.15;
        self.add_text_at(egui::pos2(0.25 + offset, 0.25 + offset));
    }

    pub fn add_text_at(&mut self, pos: egui::Pos2) {
        let id = self.next_id;
        self.next_id += 1;
        self.elements.push(CanvasElement::Text(TextElement {
            id,
            pos,
            content: "Text".to_string(),
            font_size: 18,
            bold: false,
            font: ZplFont::A,
        }));
        self.selected_id = Some(id);
    }

    pub fn add_clipart(
        &mut self,
        clipart_id: &'static str,
        clipart_label: &'static str,
        png_bytes: &'static [u8],
        width_in: f32,
        height_in: f32,
        pos: egui::Pos2,
    ) {
        let id = self.next_id;
        self.next_id += 1;
        self.elements.push(CanvasElement::Clipart(ClipartElement {
            id,
            pos,
            width_in,
            height_in,
            clipart_id,
            clipart_label,
            png_bytes,
            lock_aspect: true,
        }));
        self.selected_id = Some(id);
    }

    pub fn delete_selected(&mut self) {
        if let Some(id) = self.selected_id.take() {
            self.elements.retain(|e| e.id() != id);
        }
    }

    pub fn duplicate_selected(&mut self) {
        if let Some(id) = self.selected_id {
            if let Some(elem) = self.elements.iter().find(|e| e.id() == id).cloned() {
                let new_id = self.next_id;
                self.next_id += 1;
                let new_pos = elem.pos() + egui::vec2(0.12, 0.12);
                let mut clone = elem;
                clone.set_pos(new_pos);
                match &mut clone {
                    CanvasElement::Text(t) => t.id = new_id,
                    CanvasElement::Clipart(c) => c.id = new_id,
                }
                self.elements.push(clone);
                self.selected_id = Some(new_id);
            }
        }
    }

    pub fn nudge_selected(&mut self, dx_in: f32, dy_in: f32) {
        if let Some(id) = self.selected_id {
            for elem in &mut self.elements {
                if elem.id() == id {
                    let pos = elem.pos();
                    let nx = (pos.x + dx_in).max(0.0).min(self.label_width_in);
                    let ny = (pos.y + dy_in).max(0.0).min(self.label_height_in);
                    elem.set_pos(egui::pos2(nx, ny));
                    break;
                }
            }
        }
    }

    pub fn selected_text_mut(&mut self) -> Option<&mut TextElement> {
        let id = self.selected_id?;
        self.elements.iter_mut().find_map(|e| {
            if let CanvasElement::Text(t) = e {
                if t.id == id { return Some(t); }
            }
            None
        })
    }

    pub fn selected_clipart_mut(&mut self) -> Option<&mut ClipartElement> {
        let id = self.selected_id?;
        self.elements.iter_mut().find_map(|e| {
            if let CanvasElement::Clipart(c) = e {
                if c.id == id { return Some(c); }
            }
            None
        })
    }

    pub fn apply_preset(&mut self, index: usize) {
        if let Some(&(_, w, h)) = LABEL_PRESETS.get(index) {
            self.preset_index = index;
            self.label_width_in = w;
            self.label_height_in = h;
        }
    }
}

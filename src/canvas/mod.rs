pub mod zpl_gen;

use eframe::egui;

pub const ACCENT: egui::Color32 = egui::Color32::from_rgb(236, 72, 153);

// Label size presets (width_in, height_in, label)
pub const LABEL_PRESETS: &[(&str, f32, f32)] = &[
    ("4\" × 6\" (shipping)",      4.0, 6.0),
    ("4\" × 2\"",                 4.0, 2.0),
    ("4\" × 3\"",                 4.0, 3.0),
    ("2\" × 1\" (product tag)",   2.0, 1.0),
    ("2.25\" × 1.25\"",           2.25, 1.25),
    ("3\" × 2\"",                 3.0, 2.0),
];

#[derive(Clone, Debug)]
pub struct TextElement {
    pub id: u64,
    pub pos: egui::Pos2, // inches from top-left of label
    pub content: String,
    pub font_size: u32, // ZPL dot height
    pub bold: bool,
}

#[derive(Clone, Debug)]
pub enum CanvasElement {
    Text(TextElement),
}

impl CanvasElement {
    pub fn id(&self) -> u64 {
        match self {
            CanvasElement::Text(t) => t.id,
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
        let id = self.next_id;
        self.next_id += 1;
        // Stagger new elements so they don't stack exactly
        let offset = (self.elements.len() as f32) * 0.15;
        self.elements.push(CanvasElement::Text(TextElement {
            id,
            pos: egui::pos2(0.25 + offset, 0.25 + offset),
            content: "Text".to_string(),
            font_size: 30,
            bold: false,
        }));
        self.selected_id = Some(id);
    }

    pub fn delete_selected(&mut self) {
        if let Some(id) = self.selected_id.take() {
            self.elements.retain(|e| e.id() != id);
        }
    }

    pub fn selected_text_mut(&mut self) -> Option<&mut TextElement> {
        let id = self.selected_id?;
        self.elements.iter_mut().find_map(|e| {
            if let CanvasElement::Text(t) = e {
                if t.id == id {
                    return Some(t);
                }
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

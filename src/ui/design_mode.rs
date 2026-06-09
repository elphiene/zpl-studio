use crate::canvas::{CanvasElement, CanvasState, ZplFont, ACCENT, LABEL_PRESETS};
use crate::canvas::zpl_gen;
use crate::ui::clipart_data::{self, CAT_ALL};
use eframe::egui;
use std::collections::HashMap;

const PX_PER_INCH: f32 = 72.0;
const RULER_SIZE: f32 = 18.0;
const NUDGE_IN: f32 = 1.0 / 72.0;

pub enum ActiveTool {
    Select,
    Text,
    Clipart,
}

pub enum DesignModeAction {
    Print,
    None,
}

pub struct DesignModePanel {
    pub canvas: CanvasState,
    pub status_message: String,
    custom_w: f32,
    custom_h: f32,
    use_custom: bool,
    pub active_tool: ActiveTool,
    pub zoom: f32,
    pub show_grid: bool,
    selected_clipart_id: Option<&'static str>,
    clipart_category: &'static str,
    texture_cache: HashMap<String, egui::TextureHandle>,
    // Handle resize drag state
    active_handle: Option<u8>,       // 0=TL 1=TC 2=TR 3=MR 4=BR 5=BC 6=BL 7=ML
    drag_ptr_start: Option<egui::Pos2>,
    drag_elem_start: Option<(egui::Pos2, f32, f32)>, // (pos_in, w_in, h_in)
}

impl Default for DesignModePanel {
    fn default() -> Self {
        Self {
            canvas: CanvasState::default(),
            status_message: "Select a tool on the left, or click an element to edit".to_string(),
            custom_w: 4.0,
            custom_h: 6.0,
            use_custom: false,
            active_tool: ActiveTool::Select,
            zoom: 1.0,
            show_grid: false,
            selected_clipart_id: None,
            clipart_category: CAT_ALL,
            texture_cache: HashMap::new(),
            active_handle: None,
            drag_ptr_start: None,
            drag_elem_start: None,
        }
    }
}

impl DesignModePanel {
    pub fn get_zpl(&self) -> String {
        zpl_gen::generate(&self.canvas)
    }

    // ── Tool palette (left side panel) ────────────────────────────────
    pub fn ui_tools(&mut self, ui: &mut egui::Ui) {
        ui.add_space(8.0);
        ui.vertical_centered(|ui| {
            let select_active = matches!(self.active_tool, ActiveTool::Select);
            if tool_button(ui, "↖", select_active, "Select  (V)") {
                self.active_tool = ActiveTool::Select;
            }
            ui.add_space(4.0);
            let text_active = matches!(self.active_tool, ActiveTool::Text);
            if tool_button(ui, "T", text_active, "Text  (T)") {
                self.active_tool = ActiveTool::Text;
                self.status_message = "Click on the canvas to place text".to_string();
            }
            ui.add_space(4.0);
            let clipart_active = matches!(self.active_tool, ActiveTool::Clipart);
            if tool_button(ui, "★", clipart_active, "Clipart  (C)") {
                self.active_tool = ActiveTool::Clipart;
                self.status_message =
                    "Choose a clipart icon in the panel, then click the canvas".to_string();
            }

            ui.add_space(16.0);
            ui.separator();
            ui.add_space(8.0);

            if tool_button(ui, "+", false, "Zoom in") {
                self.zoom = (self.zoom + 0.25).min(4.0);
            }
            ui.add_space(2.0);
            ui.label(
                egui::RichText::new(format!("{}%", (self.zoom * 100.0) as u32))
                    .size(9.0)
                    .color(egui::Color32::from_gray(150)),
            );
            ui.add_space(2.0);
            if tool_button(ui, "−", false, "Zoom out") {
                self.zoom = (self.zoom - 0.25).max(0.25);
            }

            ui.add_space(12.0);
            ui.separator();
            ui.add_space(8.0);

            if tool_button(ui, "⊞", self.show_grid, "Toggle grid") {
                self.show_grid = !self.show_grid;
            }
        });
    }

    // ── Toolbar (top panel inside design mode) ────────────────────────
    pub fn ui_toolbar(&mut self, ui: &mut egui::Ui) -> DesignModeAction {
        let mut action = DesignModeAction::None;

        ui.horizontal(|ui| {
            ui.label("Label:");
            let preset_label = if self.use_custom {
                "Custom".to_string()
            } else {
                LABEL_PRESETS[self.canvas.preset_index].0.to_string()
            };
            egui::ComboBox::from_id_source("label_preset")
                .selected_text(preset_label)
                .width(160.0)
                .show_ui(ui, |ui| {
                    for (i, &(name, _, _)) in LABEL_PRESETS.iter().enumerate() {
                        if ui
                            .selectable_label(
                                !self.use_custom && self.canvas.preset_index == i,
                                name,
                            )
                            .clicked()
                        {
                            self.canvas.apply_preset(i);
                            self.use_custom = false;
                        }
                    }
                    if ui.selectable_label(self.use_custom, "Custom…").clicked() {
                        self.use_custom = true;
                    }
                });

            if self.use_custom {
                ui.label("W:");
                ui.add(
                    egui::DragValue::new(&mut self.custom_w)
                        .speed(0.05)
                        .clamp_range(0.5..=12.0)
                        .suffix("\""),
                );
                ui.label("H:");
                ui.add(
                    egui::DragValue::new(&mut self.custom_h)
                        .speed(0.05)
                        .clamp_range(0.5..=12.0)
                        .suffix("\""),
                );
                if ui.button("Apply").clicked() {
                    self.canvas.label_width_in = self.custom_w;
                    self.canvas.label_height_in = self.custom_h;
                }
            }

            egui::ComboBox::from_id_source("dpi_select")
                .selected_text(format!("{} dpi", self.canvas.dpi))
                .width(80.0)
                .show_ui(ui, |ui| {
                    for dpi in [203u16, 300, 600] {
                        if ui
                            .selectable_label(
                                self.canvas.dpi == dpi,
                                format!("{} dpi", dpi),
                            )
                            .clicked()
                        {
                            self.canvas.dpi = dpi;
                        }
                    }
                });

            ui.separator();

            // Import Image button
            if ui.button("🖼  Import Image…").on_hover_text("Import PNG/JPG onto canvas").clicked() {
                if let Some(path) = rfd::FileDialog::new()
                    .add_filter("Image", &["png", "jpg", "jpeg", "bmp", "tiff", "webp"])
                    .pick_file()
                {
                    if let Ok(bytes) = std::fs::read(&path) {
                        if let Ok(img) = image::load_from_memory(&bytes) {
                            let orig_w = img.width();
                            let orig_h = img.height();
                            let filename = path
                                .file_name()
                                .map(|n| n.to_string_lossy().into_owned())
                                .unwrap_or_else(|| "image".to_string());
                            let center = egui::pos2(
                                self.canvas.label_width_in / 2.0,
                                self.canvas.label_height_in / 2.0,
                            );
                            self.canvas.add_image(bytes, filename, orig_w, orig_h, center);
                            self.active_tool = ActiveTool::Select;
                        }
                    }
                }
            }

            ui.separator();

            // Align buttons — only interactive when something is selected
            let has_sel = self.canvas.selected_id.is_some();
            ui.add_enabled_ui(has_sel, |ui| {
                if ui.button("◀ Left").on_hover_text("Align left edge").clicked() {
                    self.align_selected_x(0.0);
                }
                if ui.button("↔ Ctr").on_hover_text("Center horizontally").clicked() {
                    self.align_selected_x(self.canvas.label_width_in / 2.0);
                }
                if ui.button("▶ Right").on_hover_text("Align right edge").clicked() {
                    self.align_selected_x(self.canvas.label_width_in);
                }
                ui.separator();
                if ui.button("▲ Top").on_hover_text("Align top edge").clicked() {
                    self.align_selected_y(0.0);
                }
                if ui.button("↕ Mid").on_hover_text("Center vertically").clicked() {
                    self.align_selected_y(self.canvas.label_height_in / 2.0);
                }
                if ui.button("▼ Bot").on_hover_text("Align bottom edge").clicked() {
                    self.align_selected_y(self.canvas.label_height_in);
                }
            });

            ui.separator();

            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                let print_btn = egui::Button::new(egui::RichText::new("🖨  Print").strong())
                    .fill(ACCENT.linear_multiply(0.65))
                    .stroke(egui::Stroke::new(1.0, ACCENT));
                if ui.add(print_btn).clicked() {
                    action = DesignModeAction::Print;
                }
            });
        });

        action
    }

    fn align_selected_x(&mut self, x: f32) {
        if let Some(id) = self.canvas.selected_id {
            for elem in &mut self.canvas.elements {
                if elem.id() == id {
                    let pos = elem.pos();
                    elem.set_pos(egui::pos2(x, pos.y));
                    break;
                }
            }
        }
    }

    fn align_selected_y(&mut self, y: f32) {
        if let Some(id) = self.canvas.selected_id {
            for elem in &mut self.canvas.elements {
                if elem.id() == id {
                    let pos = elem.pos();
                    elem.set_pos(egui::pos2(pos.x, y));
                    break;
                }
            }
        }
    }

    // ── Properties panel (right side panel) ──────────────────────────
    pub fn ui_properties(&mut self, ui: &mut egui::Ui) {
        ui.add_space(8.0);

        let selected_id = self.canvas.selected_id;

        if matches!(self.active_tool, ActiveTool::Clipart) {
            self.draw_clipart_picker(ui);
            return;
        }

        if selected_id.is_none() {
            section_header(ui, "LABEL");
            ui.label(format!(
                "{:.2}\" × {:.2}\" @ {} dpi",
                self.canvas.label_width_in, self.canvas.label_height_in, self.canvas.dpi
            ));
            ui.label(
                egui::RichText::new(format!(
                    "{} × {} dots",
                    (self.canvas.label_width_in * self.canvas.dpi as f32) as u32,
                    (self.canvas.label_height_in * self.canvas.dpi as f32) as u32,
                ))
                .color(egui::Color32::from_gray(130)),
            );
            ui.add_space(12.0);
            ui.separator();
            ui.add_space(8.0);
            ui.label(
                egui::RichText::new("Select an element to edit it.")
                    .italics()
                    .color(egui::Color32::from_gray(130)),
            );
            ui.add_space(6.0);
            ui.label(
                egui::RichText::new("V  Select    T  Text    C  Clipart\nDel  Delete    Ctrl+D  Duplicate\nArrows  Nudge")
                    .size(10.0)
                    .color(egui::Color32::from_gray(90)),
            );
            return;
        }

        let selected = selected_id.and_then(|id| {
            self.canvas.elements.iter().find(|e| e.id() == id)
        });
        match selected {
            Some(CanvasElement::Text(_))    => self.draw_text_properties(ui),
            Some(CanvasElement::Clipart(_)) => self.draw_clipart_properties(ui),
            Some(CanvasElement::Image(_))   => self.draw_image_properties(ui),
            None => {}
        }
    }

    fn draw_text_properties(&mut self, ui: &mut egui::Ui) {
        section_header(ui, "TEXT");

        if let Some(t) = self.canvas.selected_text_mut() {
            ui.label("Content:");
            ui.text_edit_singleline(&mut t.content);
            ui.add_space(10.0);

            section_header(ui, "FONT");
            egui::ComboBox::from_id_source("font_select")
                .selected_text(t.font.label())
                .width(190.0)
                .show_ui(ui, |ui| {
                    for f in ZplFont::all() {
                        if ui.selectable_label(t.font == *f, f.label()).clicked() {
                            t.font = f.clone();
                        }
                    }
                });
            ui.add_space(6.0);

            ui.label("Size:");
            ui.add(egui::Slider::new(&mut t.font_size, 6..=120).suffix(" pt"));
            ui.add_space(6.0);

            ui.horizontal(|ui| {
                let bold_fill = if t.bold { ACCENT.linear_multiply(0.55) } else { egui::Color32::from_gray(42) };
                let bold_stroke = if t.bold { egui::Stroke::new(1.5, ACCENT) } else { egui::Stroke::new(1.0, egui::Color32::from_gray(75)) };
                let bold_btn = egui::Button::new(egui::RichText::new("  B  ").strong().size(14.0))
                    .fill(bold_fill)
                    .stroke(bold_stroke)
                    .min_size(egui::vec2(36.0, 28.0));
                if ui.add(bold_btn).on_hover_text("Bold").clicked() {
                    t.bold = !t.bold;
                }
                if t.bold || t.font == ZplFont::G {
                    ui.label(
                        egui::RichText::new("double-print")
                            .size(9.5)
                            .color(egui::Color32::from_gray(110)),
                    );
                }
            });
            ui.add_space(10.0);

            section_header(ui, "POSITION");
            ui.horizontal(|ui| {
                ui.label("X");
                ui.add(
                    egui::DragValue::new(&mut t.pos.x)
                        .speed(0.01)
                        .clamp_range(0.0..=20.0)
                        .suffix("\""),
                );
                ui.label("Y");
                ui.add(
                    egui::DragValue::new(&mut t.pos.y)
                        .speed(0.01)
                        .clamp_range(0.0..=20.0)
                        .suffix("\""),
                );
            });
            ui.add_space(10.0);

            section_header(ui, "ZPL");
            let preview = format!(
                "^FO{},{}\n^A{}N,{},{}\n^FD{}^FS",
                (t.pos.x * 203.0) as u32,
                (t.pos.y * 203.0) as u32,
                t.font.zpl_letter(),
                t.font_size,
                t.font_size,
                t.content
            );
            ui.add(
                egui::TextEdit::multiline(&mut preview.as_str())
                    .font(egui::TextStyle::Monospace)
                    .desired_width(f32::INFINITY)
                    .interactive(false),
            );
        }
    }

    fn draw_clipart_properties(&mut self, ui: &mut egui::Ui) {
        section_header(ui, "CLIPART");

        if ui.button("★  Replace clipart…").clicked() {
            self.canvas.selected_id = None;
            self.active_tool = ActiveTool::Clipart;
            return;
        }

        if let Some(c) = self.canvas.selected_clipart_mut() {
            ui.add_space(6.0);
            ui.label(egui::RichText::new(c.clipart_label).strong().color(ACCENT));
            ui.add_space(8.0);

            section_header(ui, "SIZE");
            let aspect = c.height_in / c.width_in.max(0.001);
            let mut w = c.width_in;
            let mut h = c.height_in;
            let mut w_changed = false;
            let mut h_changed = false;

            ui.horizontal(|ui| {
                ui.label("W");
                w_changed = ui
                    .add(
                        egui::DragValue::new(&mut w)
                            .speed(0.01)
                            .clamp_range(0.05..=8.0)
                            .suffix("\""),
                    )
                    .changed();
            });
            ui.horizontal(|ui| {
                ui.label("H");
                h_changed = ui
                    .add(
                        egui::DragValue::new(&mut h)
                            .speed(0.01)
                            .clamp_range(0.05..=8.0)
                            .suffix("\""),
                    )
                    .changed();
            });

            if w_changed && c.lock_aspect { h = w * aspect; }
            if h_changed && c.lock_aspect { w = h / aspect.max(0.001); }
            c.width_in = w;
            c.height_in = h;

            ui.horizontal(|ui| {
                let fill = if c.lock_aspect { ACCENT.linear_multiply(0.4) } else { egui::Color32::from_gray(38) };
                if ui
                    .add(
                        egui::Button::new(if c.lock_aspect { "🔒" } else { "🔓" })
                            .fill(fill),
                    )
                    .on_hover_text("Lock aspect ratio")
                    .clicked()
                {
                    c.lock_aspect = !c.lock_aspect;
                }
                ui.label("Lock aspect");
            });
            ui.add_space(10.0);

            section_header(ui, "POSITION");
            ui.horizontal(|ui| {
                ui.label("X");
                ui.add(
                    egui::DragValue::new(&mut c.pos.x)
                        .speed(0.01)
                        .clamp_range(0.0..=20.0)
                        .suffix("\""),
                );
                ui.label("Y");
                ui.add(
                    egui::DragValue::new(&mut c.pos.y)
                        .speed(0.01)
                        .clamp_range(0.0..=20.0)
                        .suffix("\""),
                );
            });
        }
    }

    fn draw_image_properties(&mut self, ui: &mut egui::Ui) {
        section_header(ui, "IMAGE");

        if let Some(i) = self.canvas.selected_image_mut() {
            ui.label(
                egui::RichText::new(&i.filename.clone())
                    .strong()
                    .color(ACCENT),
            );
            ui.label(
                egui::RichText::new(format!("{}×{} px  (original)", i.orig_w_px, i.orig_h_px))
                    .size(10.0)
                    .color(egui::Color32::from_gray(120)),
            );
            ui.add_space(8.0);

            section_header(ui, "SIZE");
            let aspect = i.orig_h_px as f32 / i.orig_w_px.max(1) as f32;
            let mut w = i.width_in;
            let mut h = i.height_in;
            let mut w_changed = false;
            let mut h_changed = false;

            ui.horizontal(|ui| {
                ui.label("W");
                w_changed = ui
                    .add(
                        egui::DragValue::new(&mut w)
                            .speed(0.01)
                            .clamp_range(0.05..=12.0)
                            .suffix("\""),
                    )
                    .changed();
            });
            ui.horizontal(|ui| {
                ui.label("H");
                h_changed = ui
                    .add(
                        egui::DragValue::new(&mut h)
                            .speed(0.01)
                            .clamp_range(0.05..=12.0)
                            .suffix("\""),
                    )
                    .changed();
            });
            if w_changed && i.lock_aspect { h = w * aspect; }
            if h_changed && i.lock_aspect { w = h / aspect.max(0.001); }
            i.width_in = w;
            i.height_in = h;

            ui.horizontal(|ui| {
                let fill = if i.lock_aspect { ACCENT.linear_multiply(0.4) } else { egui::Color32::from_gray(38) };
                if ui
                    .add(egui::Button::new(if i.lock_aspect { "🔒" } else { "🔓" }).fill(fill))
                    .on_hover_text("Lock aspect ratio")
                    .clicked()
                {
                    i.lock_aspect = !i.lock_aspect;
                }
                ui.label("Lock aspect");
            });
            ui.add_space(10.0);

            section_header(ui, "POSITION");
            ui.horizontal(|ui| {
                ui.label("X");
                ui.add(
                    egui::DragValue::new(&mut i.pos.x)
                        .speed(0.01)
                        .clamp_range(0.0..=20.0)
                        .suffix("\""),
                );
                ui.label("Y");
                ui.add(
                    egui::DragValue::new(&mut i.pos.y)
                        .speed(0.01)
                        .clamp_range(0.0..=20.0)
                        .suffix("\""),
                );
            });
            ui.add_space(10.0);

            // Replace button
            ui.separator();
            if ui.button("🖼  Replace image…").clicked() {
                // Collect what we need before the mutable borrow ends
                let id = i.id;
                let pos = i.pos;
                drop(i); // release mutable borrow
                if let Some(path) = rfd::FileDialog::new()
                    .add_filter("Image", &["png", "jpg", "jpeg", "bmp", "tiff", "webp"])
                    .pick_file()
                {
                    if let Ok(bytes) = std::fs::read(&path) {
                        if let Ok(img) = image::load_from_memory(&bytes) {
                            let orig_w = img.width();
                            let orig_h = img.height();
                            let filename = path
                                .file_name()
                                .map(|n| n.to_string_lossy().into_owned())
                                .unwrap_or_else(|| "image".to_string());
                            // Replace element in-place
                            for elem in &mut self.canvas.elements {
                                if elem.id() == id {
                                    if let CanvasElement::Image(im) = elem {
                                        let aspect = orig_h as f32 / orig_w.max(1) as f32;
                                        im.png_bytes = bytes;
                                        im.filename = filename;
                                        im.orig_w_px = orig_w;
                                        im.orig_h_px = orig_h;
                                        im.height_in = im.width_in * aspect;
                                        // Invalidate texture cache for this element
                                        self.texture_cache.remove(&format!("img_{}", id));
                                    }
                                    break;
                                }
                            }
                            // Re-centre at same position
                            let _ = pos;
                        }
                    }
                }
            }
        }
    }

    fn draw_clipart_picker(&mut self, ui: &mut egui::Ui) {
        section_header(ui, "CLIPART");

        ui.horizontal_wrapped(|ui| {
            for &cat in clipart_data::categories() {
                let active = self.clipart_category == cat;
                let btn = egui::Button::new(egui::RichText::new(cat).size(10.5))
                    .fill(if active { ACCENT.linear_multiply(0.4) } else { egui::Color32::from_gray(32) })
                    .stroke(egui::Stroke::new(
                        1.0,
                        if active { ACCENT } else { egui::Color32::from_gray(58) },
                    ));
                if ui.add(btn).clicked() {
                    self.clipart_category = cat;
                }
            }
        });
        ui.add_space(8.0);

        let entries: Vec<_> = clipart_data::CLIPART
            .iter()
            .filter(|e| {
                self.clipart_category == clipart_data::CAT_ALL
                    || e.category == self.clipart_category
            })
            .collect();

        egui::Grid::new("clipart_grid")
            .num_columns(3)
            .spacing([5.0, 5.0])
            .show(ui, |ui| {
                for (i, entry) in entries.iter().enumerate() {
                    let is_sel = self.selected_clipart_id == Some(entry.id);
                    let btn = egui::Button::new(
                        egui::RichText::new(entry.label).size(9.5),
                    )
                    .fill(if is_sel { ACCENT.linear_multiply(0.35) } else { egui::Color32::from_gray(32) })
                    .stroke(egui::Stroke::new(
                        if is_sel { 2.0 } else { 1.0 },
                        if is_sel { ACCENT } else { egui::Color32::from_gray(58) },
                    ))
                    .min_size(egui::vec2(58.0, 58.0));
                    if ui.add(btn).on_hover_text(entry.label).clicked() {
                        self.selected_clipart_id = Some(entry.id);
                        self.status_message =
                            format!("'{}' selected — click canvas to place", entry.label);
                    }
                    if (i + 1) % 3 == 0 {
                        ui.end_row();
                    }
                }
                // fill remaining row
                let remainder = entries.len() % 3;
                if remainder != 0 {
                    for _ in remainder..3 {
                        ui.label("");
                    }
                    ui.end_row();
                }
            });

        if let Some(sel_id) = self.selected_clipart_id {
            ui.add_space(8.0);
            if let Some(entry) = clipart_data::find(sel_id) {
                ui.label(egui::RichText::new(format!("Selected: {}", entry.label)).color(ACCENT).strong());
                ui.label(
                    egui::RichText::new("Click canvas to place")
                        .size(10.0)
                        .color(egui::Color32::from_gray(130)),
                );
            }
        }
    }

    // ── Canvas (central panel) ────────────────────────────────────────
    pub fn ui_canvas(&mut self, ui: &mut egui::Ui) {
        // Keyboard shortcuts
        let (key_v, key_t, key_c, key_del, key_esc, key_dup, arrow) = ui.input(|i| {
            (
                i.key_pressed(egui::Key::V),
                i.key_pressed(egui::Key::T),
                i.key_pressed(egui::Key::C),
                i.key_pressed(egui::Key::Delete) || i.key_pressed(egui::Key::Backspace),
                i.key_pressed(egui::Key::Escape),
                i.modifiers.ctrl && i.key_pressed(egui::Key::D),
                (
                    i.key_pressed(egui::Key::ArrowLeft),
                    i.key_pressed(egui::Key::ArrowRight),
                    i.key_pressed(egui::Key::ArrowUp),
                    i.key_pressed(egui::Key::ArrowDown),
                ),
            )
        });
        let ctrl_scroll = ui.input(|i| {
            if i.modifiers.ctrl { i.raw_scroll_delta.y } else { 0.0 }
        });

        // Only fire canvas shortcuts when no text input widget has keyboard focus
        let text_focused = ui.ctx().memory(|m| m.focused().is_some());

        if !text_focused {
            if key_v { self.active_tool = ActiveTool::Select; }
            if key_t {
                self.active_tool = ActiveTool::Text;
                self.status_message = "Click on the canvas to place text".to_string();
            }
            if key_c {
                self.active_tool = ActiveTool::Clipart;
                self.status_message = "Choose clipart in the panel, then click canvas".to_string();
            }
        }
        if key_esc { self.canvas.selected_id = None; }
        if !text_focused && key_del && self.canvas.selected_id.is_some() {
            self.canvas.delete_selected();
            self.status_message = "Element deleted".to_string();
        }
        if !text_focused && key_dup {
            self.canvas.duplicate_selected();
            self.status_message = "Element duplicated".to_string();
        }
        if !text_focused && self.canvas.selected_id.is_some() {
            if arrow.0 { self.canvas.nudge_selected(-NUDGE_IN, 0.0); }
            if arrow.1 { self.canvas.nudge_selected(NUDGE_IN, 0.0); }
            if arrow.2 { self.canvas.nudge_selected(0.0, -NUDGE_IN); }
            if arrow.3 { self.canvas.nudge_selected(0.0, NUDGE_IN); }
        }
        if ctrl_scroll != 0.0 {
            self.zoom = (self.zoom + ctrl_scroll * 0.001).clamp(0.25, 4.0);
        }

        let canvas_w = self.canvas.label_width_in * PX_PER_INCH * self.zoom;
        let canvas_h = self.canvas.label_height_in * PX_PER_INCH * self.zoom;

        egui::ScrollArea::both().show(ui, |ui| {
            let total_w = canvas_w + RULER_SIZE + 16.0;
            let total_h = canvas_h + RULER_SIZE + 16.0;

            let (resp, painter) =
                ui.allocate_painter(egui::vec2(total_w, total_h), egui::Sense::click_and_drag());

            let origin = resp.rect.min + egui::vec2(RULER_SIZE, RULER_SIZE);
            let canvas_rect =
                egui::Rect::from_min_size(origin, egui::vec2(canvas_w, canvas_h));

            self.draw_rulers(&painter, resp.rect.min, canvas_w, canvas_h);
            self.draw_canvas_surface(&painter, canvas_rect);
            self.draw_grid(&painter, canvas_rect);
            self.draw_elements(ui.ctx(), &painter, canvas_rect);
            self.handle_canvas_interaction(&resp, canvas_rect);
        });
    }

    fn draw_rulers(
        &self,
        painter: &egui::Painter,
        top_left: egui::Pos2,
        canvas_w: f32,
        canvas_h: f32,
    ) {
        let ruler_bg = egui::Color32::from_gray(28);
        let tick_minor = egui::Color32::from_gray(85);
        let tick_major = egui::Color32::from_gray(175);
        let label_col = egui::Color32::from_gray(155);

        let origin = top_left + egui::vec2(RULER_SIZE, RULER_SIZE);

        painter.rect_filled(
            egui::Rect::from_min_size(egui::pos2(top_left.x + RULER_SIZE, top_left.y), egui::vec2(canvas_w, RULER_SIZE)),
            0.0, ruler_bg,
        );
        painter.rect_filled(
            egui::Rect::from_min_size(egui::pos2(top_left.x, top_left.y + RULER_SIZE), egui::vec2(RULER_SIZE, canvas_h)),
            0.0, ruler_bg,
        );
        painter.rect_filled(
            egui::Rect::from_min_size(top_left, egui::vec2(RULER_SIZE, RULER_SIZE)),
            0.0, ruler_bg,
        );

        // Horizontal ticks (every 0.25", label every 1")
        let mut x_in = 0.0f32;
        while x_in <= self.canvas.label_width_in + 0.01 {
            let px = origin.x + x_in * PX_PER_INCH * self.zoom;
            let major = (x_in * 4.0).round() as u32 % 4 == 0;
            let tick_h = if major { 8.0 } else { 4.0 };
            painter.line_segment(
                [egui::pos2(px, top_left.y + RULER_SIZE - tick_h), egui::pos2(px, top_left.y + RULER_SIZE)],
                egui::Stroke::new(1.0, if major { tick_major } else { tick_minor }),
            );
            if major && x_in > 0.0 {
                painter.text(egui::pos2(px + 2.0, top_left.y + 2.0), egui::Align2::LEFT_TOP,
                    format!("{}", x_in as u32), egui::FontId::proportional(9.0), label_col);
            }
            x_in += 0.25;
        }

        // Vertical ticks
        let mut y_in = 0.0f32;
        while y_in <= self.canvas.label_height_in + 0.01 {
            let py = origin.y + y_in * PX_PER_INCH * self.zoom;
            let major = (y_in * 4.0).round() as u32 % 4 == 0;
            let tick_w = if major { 8.0 } else { 4.0 };
            painter.line_segment(
                [egui::pos2(top_left.x + RULER_SIZE - tick_w, py), egui::pos2(top_left.x + RULER_SIZE, py)],
                egui::Stroke::new(1.0, if major { tick_major } else { tick_minor }),
            );
            if major && y_in > 0.0 {
                painter.text(egui::pos2(top_left.x + 1.0, py + 2.0), egui::Align2::LEFT_TOP,
                    format!("{}", y_in as u32), egui::FontId::proportional(9.0), label_col);
            }
            y_in += 0.25;
        }
    }

    fn draw_canvas_surface(&self, painter: &egui::Painter, canvas_rect: egui::Rect) {
        let shadow = canvas_rect.translate(egui::vec2(4.0, 4.0));
        painter.rect_filled(shadow, 2.0, egui::Color32::from_rgba_unmultiplied(0, 0, 0, 70));
        painter.rect_filled(canvas_rect, 0.0, egui::Color32::WHITE);
        painter.rect_stroke(canvas_rect, 0.0, egui::Stroke::new(1.0, egui::Color32::from_gray(110)));
        let guide = egui::Color32::from_rgba_unmultiplied(61, 111, 191, 28);
        let cx = canvas_rect.center().x;
        let cy = canvas_rect.center().y;
        painter.line_segment(
            [egui::pos2(cx, canvas_rect.min.y), egui::pos2(cx, canvas_rect.max.y)],
            egui::Stroke::new(1.0, guide),
        );
        painter.line_segment(
            [egui::pos2(canvas_rect.min.x, cy), egui::pos2(canvas_rect.max.x, cy)],
            egui::Stroke::new(1.0, guide),
        );
    }

    fn draw_grid(&self, painter: &egui::Painter, canvas_rect: egui::Rect) {
        if !self.show_grid { return; }
        let grid_col = egui::Color32::from_rgb(200, 200, 230);
        let step = 0.25 * PX_PER_INCH * self.zoom;
        let stroke = egui::Stroke::new(1.0, grid_col);
        let mut x = canvas_rect.min.x + step;
        while x < canvas_rect.max.x { painter.line_segment([egui::pos2(x, canvas_rect.min.y), egui::pos2(x, canvas_rect.max.y)], stroke); x += step; }
        let mut y = canvas_rect.min.y + step;
        while y < canvas_rect.max.y { painter.line_segment([egui::pos2(canvas_rect.min.x, y), egui::pos2(canvas_rect.max.x, y)], stroke); y += step; }
    }

    fn draw_elements(&mut self, ctx: &egui::Context, painter: &egui::Painter, canvas_rect: egui::Rect) {
        let origin = canvas_rect.min;
        let selected_id = self.canvas.selected_id;
        let zoom = self.zoom;
        let dpi = self.canvas.dpi;

        for elem in &self.canvas.elements {
            match elem {
                CanvasElement::Text(t) => {
                    let sp = origin + egui::vec2(t.pos.x * PX_PER_INCH * zoom, t.pos.y * PX_PER_INCH * zoom);
                    let base_px = (t.font_size as f32 * PX_PER_INCH / 72.0 * zoom).clamp(6.0, 200.0);
                    let font_px = match t.font {
                        ZplFont::B => base_px * 0.75,
                        ZplFont::H => base_px * 1.3,
                        _ => base_px,
                    };
                    let font_id = match t.font {
                        ZplFont::D | ZplFont::E => egui::FontId::monospace(font_px),
                        _ => egui::FontId::proportional(font_px),
                    };
                    let do_bold = t.bold || t.font == ZplFont::G;

                    let text_rect = painter.text(sp, egui::Align2::LEFT_TOP, &t.content, font_id.clone(), egui::Color32::BLACK);
                    if do_bold {
                        painter.text(sp + egui::vec2(0.8, 0.0), egui::Align2::LEFT_TOP, &t.content, font_id, egui::Color32::BLACK);
                    }

                    if selected_id == Some(t.id) {
                        let sel = text_rect.expand(4.0);
                        painter.rect_stroke(sel, 2.0, egui::Stroke::new(1.5, ACCENT));
                        draw_handles(painter, sel);
                    }
                }
                CanvasElement::Clipart(c) => {
                    let sp = origin + egui::vec2(c.pos.x * PX_PER_INCH * zoom, c.pos.y * PX_PER_INCH * zoom);
                    let pw = c.width_in * PX_PER_INCH * zoom;
                    let ph = c.height_in * PX_PER_INCH * zoom;
                    let dest = egui::Rect::from_min_size(sp, egui::vec2(pw, ph));

                    let cache_key = format!("ca_{}_{}",c.clipart_id, dpi);
                    if !self.texture_cache.contains_key(&cache_key) {
                        if let Ok(img) = image::load_from_memory(c.png_bytes) {
                            let rgba = img.to_rgba8();
                            let size = [img.width() as usize, img.height() as usize];
                            let ci = egui::ColorImage::from_rgba_unmultiplied(size, rgba.as_flat_samples().samples);
                            let handle = ctx.load_texture(&cache_key, ci, egui::TextureOptions::LINEAR);
                            self.texture_cache.insert(cache_key.clone(), handle);
                        }
                    }
                    if let Some(tex) = self.texture_cache.get(&cache_key) {
                        let uv = egui::Rect::from_min_max(egui::pos2(0.0,0.0), egui::pos2(1.0,1.0));
                        painter.image(tex.id(), dest, uv, egui::Color32::WHITE);
                    }

                    if selected_id == Some(c.id) {
                        let sel = dest.expand(4.0);
                        painter.rect_stroke(sel, 2.0, egui::Stroke::new(1.5, ACCENT));
                        draw_handles(painter, sel);
                    }
                }
                CanvasElement::Image(i) => {
                    let sp = origin + egui::vec2(i.pos.x * PX_PER_INCH * zoom, i.pos.y * PX_PER_INCH * zoom);
                    let pw = i.width_in * PX_PER_INCH * zoom;
                    let ph = i.height_in * PX_PER_INCH * zoom;
                    let dest = egui::Rect::from_min_size(sp, egui::vec2(pw, ph));

                    let cache_key = format!("img_{}", i.id);
                    if !self.texture_cache.contains_key(&cache_key) {
                        if let Ok(img) = image::load_from_memory(&i.png_bytes) {
                            let rgba = img.to_rgba8();
                            let size = [img.width() as usize, img.height() as usize];
                            let ci = egui::ColorImage::from_rgba_unmultiplied(size, rgba.as_flat_samples().samples);
                            let handle = ctx.load_texture(&cache_key, ci, egui::TextureOptions::LINEAR);
                            self.texture_cache.insert(cache_key.clone(), handle);
                        }
                    }
                    if let Some(tex) = self.texture_cache.get(&cache_key) {
                        let uv = egui::Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(1.0, 1.0));
                        painter.image(tex.id(), dest, uv, egui::Color32::WHITE);
                    }

                    if selected_id == Some(i.id) {
                        let sel = dest.expand(4.0);
                        painter.rect_stroke(sel, 2.0, egui::Stroke::new(1.5, ACCENT));
                        draw_handles(painter, sel);
                    }
                }
            }
        }
    }

    fn handle_canvas_interaction(&mut self, resp: &egui::Response, canvas_rect: egui::Rect) {
        let origin = canvas_rect.min;

        // ── Drag started: decide if we're moving or resizing ─────────
        if resp.drag_started() {
            if let Some(ptr) = resp.interact_pointer_pos() {
                if canvas_rect.contains(ptr) && matches!(self.active_tool, ActiveTool::Select) {
                    // Check handles of the currently selected element first
                    let handle = self.canvas.selected_id.and_then(|id| {
                        Self::elem_bounds(id, &self.canvas.elements, origin, self.zoom)
                            .and_then(|bounds| find_handle_at(ptr, bounds.expand(4.0)))
                    });
                    if let Some(h) = handle {
                        // Store start state for resize
                        self.active_handle = Some(h);
                        self.drag_ptr_start = Some(ptr);
                        self.drag_elem_start = self.canvas.selected_id.and_then(|id| {
                            self.canvas.elements.iter().find(|e| e.id() == id).map(|e| {
                                let (w, h2) = elem_size(e);
                                (e.pos(), w, h2)
                            })
                        });
                    } else {
                        self.active_handle = None;
                        self.hit_test(ptr, origin);
                    }
                }
            }
        }

        // ── Dragging ─────────────────────────────────────────────────
        if resp.dragged() && self.canvas.selected_id.is_some() {
            let id = self.canvas.selected_id.unwrap();
            let zoom = self.zoom;
            let lw = self.canvas.label_width_in;
            let lh = self.canvas.label_height_in;

            if let (Some(h_idx), Some(ptr_start), Some((start_pos, start_w, start_h))) =
                (self.active_handle, self.drag_ptr_start, self.drag_elem_start)
            {
                // Resize the element
                if let Some(ptr) = resp.interact_pointer_pos() {
                    let dx = (ptr.x - ptr_start.x) / (PX_PER_INCH * zoom);
                    let dy = (ptr.y - ptr_start.y) / (PX_PER_INCH * zoom);
                    apply_handle_resize(
                        h_idx, dx, dy,
                        start_pos, start_w, start_h,
                        id, &mut self.canvas.elements, lw, lh,
                    );
                }
            } else {
                // Move the element
                let delta = resp.drag_delta();
                for elem in &mut self.canvas.elements {
                    if elem.id() == id {
                        let pos = elem.pos();
                        let nx = (pos.x + delta.x / (PX_PER_INCH * zoom)).max(0.0).min(lw);
                        let ny = (pos.y + delta.y / (PX_PER_INCH * zoom)).max(0.0).min(lh);
                        elem.set_pos(egui::pos2(nx, ny));
                        break;
                    }
                }
            }
        }

        if resp.drag_released() {
            self.active_handle = None;
            self.drag_ptr_start = None;
            self.drag_elem_start = None;
        }

        if resp.clicked() {
            if let Some(ptr) = resp.interact_pointer_pos() {
                if !canvas_rect.contains(ptr) { return; }
                let pos_in = egui::pos2(
                    (ptr.x - origin.x) / (PX_PER_INCH * self.zoom),
                    (ptr.y - origin.y) / (PX_PER_INCH * self.zoom),
                );
                match self.active_tool {
                    ActiveTool::Text => {
                        self.canvas.add_text_at(pos_in);
                        self.active_tool = ActiveTool::Select;
                        self.status_message = "Text added — edit in Properties".to_string();
                    }
                    ActiveTool::Clipart => {
                        if let Some(cid) = self.selected_clipart_id {
                            if let Some(entry) = clipart_data::find(cid) {
                                self.canvas.add_clipart(entry.id, entry.label, entry.png_bytes, 0.5, 0.5, pos_in);
                                self.active_tool = ActiveTool::Select;
                                self.status_message = format!("{} placed", entry.label);
                            }
                        } else {
                            self.status_message = "Select a clipart icon in the Properties panel first".to_string();
                        }
                    }
                    ActiveTool::Select => {
                        self.hit_test(ptr, origin);
                    }
                }
            }
        }
    }

    /// Screen-space bounding rect of an element (for handle hit testing). None for Text.
    fn elem_bounds(id: u64, elements: &[CanvasElement], origin: egui::Pos2, zoom: f32) -> Option<egui::Rect> {
        elements.iter().find(|e| e.id() == id).and_then(|e| match e {
            CanvasElement::Clipart(c) => Some(egui::Rect::from_min_size(
                origin + egui::vec2(c.pos.x * PX_PER_INCH * zoom, c.pos.y * PX_PER_INCH * zoom),
                egui::vec2(c.width_in * PX_PER_INCH * zoom, c.height_in * PX_PER_INCH * zoom),
            )),
            CanvasElement::Image(i) => Some(egui::Rect::from_min_size(
                origin + egui::vec2(i.pos.x * PX_PER_INCH * zoom, i.pos.y * PX_PER_INCH * zoom),
                egui::vec2(i.width_in * PX_PER_INCH * zoom, i.height_in * PX_PER_INCH * zoom),
            )),
            CanvasElement::Text(_) => None,
        })
    }

    fn hit_test(&mut self, screen_pos: egui::Pos2, canvas_origin: egui::Pos2) {
        for elem in self.canvas.elements.iter().rev() {
            match elem {
                CanvasElement::Text(t) => {
                    let ep = canvas_origin + egui::vec2(t.pos.x * PX_PER_INCH * self.zoom, t.pos.y * PX_PER_INCH * self.zoom);
                    let base_px = (t.font_size as f32 * PX_PER_INCH / 72.0 * self.zoom).clamp(6.0, 200.0);
                    let font_px = match t.font { ZplFont::B => base_px * 0.75, ZplFont::H => base_px * 1.3, _ => base_px };
                    let approx_w = t.content.len() as f32 * font_px * 0.55 + 8.0;
                    let approx_h = font_px * 1.4 + 8.0;
                    if egui::Rect::from_min_size(ep - egui::vec2(4.0, 4.0), egui::vec2(approx_w, approx_h)).contains(screen_pos) {
                        self.canvas.selected_id = Some(t.id);
                        return;
                    }
                }
                CanvasElement::Clipart(c) => {
                    let ep = canvas_origin + egui::vec2(c.pos.x * PX_PER_INCH * self.zoom, c.pos.y * PX_PER_INCH * self.zoom);
                    let dest = egui::Rect::from_min_size(ep, egui::vec2(c.width_in * PX_PER_INCH * self.zoom, c.height_in * PX_PER_INCH * self.zoom));
                    if dest.expand(4.0).contains(screen_pos) {
                        self.canvas.selected_id = Some(c.id);
                        return;
                    }
                }
                CanvasElement::Image(i) => {
                    let ep = canvas_origin + egui::vec2(i.pos.x * PX_PER_INCH * self.zoom, i.pos.y * PX_PER_INCH * self.zoom);
                    let dest = egui::Rect::from_min_size(ep, egui::vec2(i.width_in * PX_PER_INCH * self.zoom, i.height_in * PX_PER_INCH * self.zoom));
                    if dest.expand(4.0).contains(screen_pos) {
                        self.canvas.selected_id = Some(i.id);
                        return;
                    }
                }
            }
        }
        self.canvas.selected_id = None;
    }
}

// ── Resize helpers ────────────────────────────────────────────────────────

fn elem_size(e: &CanvasElement) -> (f32, f32) {
    match e {
        CanvasElement::Clipart(c) => (c.width_in, c.height_in),
        CanvasElement::Image(i)   => (i.width_in, i.height_in),
        CanvasElement::Text(_)    => (0.0, 0.0), // text doesn't resize via handles
    }
}

/// Returns the screen-space bounding rect for the selected element (for handle hit testing).
/// Returns None for text elements (they don't support handle resize).
fn find_handle_at(ptr: egui::Pos2, rect: egui::Rect) -> Option<u8> {
    let handle_r = 6.0; // hit radius in screen pixels
    let positions = [
        rect.left_top(),
        egui::pos2(rect.center().x, rect.min.y),
        rect.right_top(),
        egui::pos2(rect.max.x, rect.center().y),
        rect.right_bottom(),
        egui::pos2(rect.center().x, rect.max.y),
        rect.left_bottom(),
        egui::pos2(rect.min.x, rect.center().y),
    ];
    positions.iter().enumerate().find_map(|(i, &h)| {
        if (ptr.x - h.x).abs() <= handle_r && (ptr.y - h.y).abs() <= handle_r {
            Some(i as u8)
        } else {
            None
        }
    })
}

fn apply_handle_resize(
    handle: u8,
    dx: f32, dy: f32,
    start_pos: egui::Pos2, start_w: f32, start_h: f32,
    elem_id: u64,
    elements: &mut Vec<CanvasElement>,
    max_w: f32, max_h: f32,
) {
    for elem in elements.iter_mut() {
        if elem.id() != elem_id { continue; }
        let (lock, aspect) = match elem {
            CanvasElement::Image(i)   => (i.lock_aspect, i.orig_h_px as f32 / i.orig_w_px.max(1) as f32),
            CanvasElement::Clipart(c) => (c.lock_aspect, c.height_in / c.width_in.max(0.001)),
            CanvasElement::Text(_)    => return,
        };

        // Compute new pos and size based on which handle is dragged.
        // Handles: 0=TL 1=TC 2=TR 3=MR 4=BR 5=BC 6=BL 7=ML
        let (mut new_x, mut new_y, mut new_w, mut new_h) = (start_pos.x, start_pos.y, start_w, start_h);
        match handle {
            0 => { new_x = start_pos.x + dx; new_w = (start_w - dx).max(0.05); new_y = start_pos.y + dy; new_h = (start_h - dy).max(0.05); }
            1 => { new_y = start_pos.y + dy; new_h = (start_h - dy).max(0.05); }
            2 => { new_w = (start_w + dx).max(0.05); new_y = start_pos.y + dy; new_h = (start_h - dy).max(0.05); }
            3 => { new_w = (start_w + dx).max(0.05); }
            4 => { new_w = (start_w + dx).max(0.05); new_h = (start_h + dy).max(0.05); }
            5 => { new_h = (start_h + dy).max(0.05); }
            6 => { new_x = start_pos.x + dx; new_w = (start_w - dx).max(0.05); new_h = (start_h + dy).max(0.05); }
            7 => { new_x = start_pos.x + dx; new_w = (start_w - dx).max(0.05); }
            _ => {}
        }

        // Constrain
        new_x = new_x.max(0.0).min(max_w - 0.05);
        new_y = new_y.max(0.0).min(max_h - 0.05);
        new_w = new_w.min(max_w);
        new_h = new_h.min(max_h);

        // Aspect ratio lock — use width as authority for corner/vertical handles,
        // height for horizontal-only handles
        if lock {
            match handle {
                1 | 5 => { new_w = new_h / aspect.max(0.001); } // top/bottom edge: height drives
                7 | 3 => { new_h = new_w * aspect; }            // left/right edge: width drives
                _     => { new_h = new_w * aspect; }            // corners: width drives
            }
        }

        match elem {
            CanvasElement::Image(i)   => { i.pos = egui::pos2(new_x, new_y); i.width_in = new_w; i.height_in = new_h; }
            CanvasElement::Clipart(c) => { c.pos = egui::pos2(new_x, new_y); c.width_in = new_w; c.height_in = new_h; }
            _ => {}
        }
        break;
    }
}

fn section_header(ui: &mut egui::Ui, title: &str) {
    ui.add_space(2.0);
    ui.horizontal(|ui| {
        ui.label(egui::RichText::new(title).size(10.0).color(ACCENT).strong());
        ui.add(egui::Separator::default().horizontal());
    });
    ui.add_space(4.0);
}

fn draw_handles(painter: &egui::Painter, rect: egui::Rect) {
    let sz = egui::vec2(6.0, 6.0);
    for &p in &[
        rect.left_top(),
        egui::pos2(rect.center().x, rect.min.y),
        rect.right_top(),
        egui::pos2(rect.max.x, rect.center().y),
        rect.right_bottom(),
        egui::pos2(rect.center().x, rect.max.y),
        rect.left_bottom(),
        egui::pos2(rect.min.x, rect.center().y),
    ] {
        painter.rect_filled(egui::Rect::from_center_size(p, sz), 1.0, ACCENT);
    }
}

fn tool_button(ui: &mut egui::Ui, label: &str, active: bool, tip: &str) -> bool {
    let fill = if active { ACCENT.linear_multiply(0.55) } else { egui::Color32::from_gray(42) };
    let stroke = egui::Stroke::new(if active { 1.5 } else { 1.0 }, if active { ACCENT } else { egui::Color32::from_gray(70) });
    ui.add(
        egui::Button::new(egui::RichText::new(label).size(16.0).strong())
            .fill(fill)
            .stroke(stroke)
            .min_size(egui::vec2(40.0, 36.0)),
    )
    .on_hover_text(tip)
    .clicked()
}

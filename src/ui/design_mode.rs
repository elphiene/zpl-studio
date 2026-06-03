use crate::canvas::{CanvasElement, CanvasState, ACCENT, LABEL_PRESETS};
use crate::canvas::zpl_gen;
use eframe::egui;

const PX_PER_INCH: f32 = 72.0;

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
}

impl Default for DesignModePanel {
    fn default() -> Self {
        Self {
            canvas: CanvasState::default(),
            status_message: "Click '＋ Text' to add a text element".to_string(),
            custom_w: 4.0,
            custom_h: 6.0,
            use_custom: false,
        }
    }
}

impl DesignModePanel {
    pub fn get_zpl(&self) -> String {
        zpl_gen::generate(&self.canvas)
    }

    pub fn ui(&mut self, ui: &mut egui::Ui) -> DesignModeAction {
        let mut action = DesignModeAction::None;

        // ── Toolbar row ──────────────────────────────────────────────
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
                        if ui.selectable_label(
                            !self.use_custom && self.canvas.preset_index == i,
                            name,
                        ).clicked() {
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
                ui.add(egui::DragValue::new(&mut self.custom_w).speed(0.1).clamp_range(0.5..=12.0).suffix("\""));
                ui.label("H:");
                ui.add(egui::DragValue::new(&mut self.custom_h).speed(0.1).clamp_range(0.5..=12.0).suffix("\""));
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
                        if ui.selectable_label(self.canvas.dpi == dpi, format!("{} dpi", dpi)).clicked() {
                            self.canvas.dpi = dpi;
                        }
                    }
                });

            ui.separator();

            if ui.button("＋ Text").clicked() {
                self.canvas.add_text();
                self.status_message = "Text element added — drag to position, edit in Properties".to_string();
            }

            if self.canvas.selected_id.is_some() && ui.button("🗑 Delete").clicked() {
                self.canvas.delete_selected();
                self.status_message = "Element deleted".to_string();
            }

            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if ui.button("🖨 Print").clicked() {
                    action = DesignModeAction::Print;
                }
            });
        });

        ui.add_space(6.0);
        ui.separator();
        ui.add_space(6.0);

        // ── Main area: canvas left, properties right ──────────────────
        ui.horizontal_top(|ui| {
            // Canvas column
            ui.vertical(|ui| {
                ui.label(egui::RichText::new("Canvas:").strong());
                ui.add_space(4.0);
                self.draw_canvas(ui);
            });

            ui.add_space(16.0);

            // Properties column
            ui.vertical(|ui| {
                ui.set_min_width(220.0);
                ui.label(egui::RichText::new("Properties:").strong());
                ui.add_space(4.0);
                self.draw_properties(ui);
            });
        });

        // Handle Delete key
        if self.canvas.selected_id.is_some()
            && ui.input(|i| i.key_pressed(egui::Key::Delete) || i.key_pressed(egui::Key::Backspace))
        {
            self.canvas.delete_selected();
            self.status_message = "Element deleted".to_string();
        }

        action
    }

    fn draw_canvas(&mut self, ui: &mut egui::Ui) {
        let canvas_w = self.canvas.label_width_in * PX_PER_INCH;
        let canvas_h = self.canvas.label_height_in * PX_PER_INCH;

        // Drop shadow effect
        let (resp, painter) = ui.allocate_painter(
            egui::vec2(canvas_w + 4.0, canvas_h + 4.0),
            egui::Sense::click_and_drag(),
        );
        let shadow_rect = resp.rect.translate(egui::vec2(3.0, 3.0));
        let canvas_rect = egui::Rect::from_min_size(resp.rect.min, egui::vec2(canvas_w, canvas_h));

        painter.rect_filled(shadow_rect, 1.0, egui::Color32::from_rgba_unmultiplied(0, 0, 0, 60));
        painter.rect_filled(canvas_rect, 0.0, egui::Color32::WHITE);
        painter.rect_stroke(canvas_rect, 0.0, egui::Stroke::new(1.0, egui::Color32::from_gray(150)));

        let origin = canvas_rect.min;

        // Center guides
        let guide = egui::Color32::from_rgba_unmultiplied(61, 111, 191, 35);
        painter.line_segment(
            [egui::pos2(origin.x + canvas_w / 2.0, origin.y), egui::pos2(origin.x + canvas_w / 2.0, origin.y + canvas_h)],
            egui::Stroke::new(1.0, guide),
        );
        painter.line_segment(
            [egui::pos2(origin.x, origin.y + canvas_h / 2.0), egui::pos2(origin.x + canvas_w, origin.y + canvas_h / 2.0)],
            egui::Stroke::new(1.0, guide),
        );

        // Drag selected element
        if resp.dragged() {
            let delta = resp.drag_delta();
            let max_x = self.canvas.label_width_in;
            let max_y = self.canvas.label_height_in;
            if let Some(t) = self.canvas.selected_text_mut() {
                t.pos.x = (t.pos.x + delta.x / PX_PER_INCH).max(0.0).min(max_x);
                t.pos.y = (t.pos.y + delta.y / PX_PER_INCH).max(0.0).min(max_y);
            }
        }

        // Click: hit test to select / deselect
        if resp.clicked() || resp.drag_started() {
            if let Some(ptr) = resp.interact_pointer_pos() {
                self.hit_test(ptr, origin);
            }
        }

        // Draw elements
        let selected_id = self.canvas.selected_id;
        for elem in &self.canvas.elements {
            match elem {
                CanvasElement::Text(t) => {
                    let screen_pos = origin + egui::vec2(t.pos.x * PX_PER_INCH, t.pos.y * PX_PER_INCH);
                    let font_px = (t.font_size as f32 * PX_PER_INCH / 203.0).clamp(8.0, 64.0);

                    let text_rect = painter.text(
                        screen_pos,
                        egui::Align2::LEFT_TOP,
                        &t.content,
                        egui::FontId::proportional(font_px),
                        egui::Color32::BLACK,
                    );

                    if selected_id == Some(t.id) {
                        let sel = text_rect.expand(3.0);
                        painter.rect_stroke(sel, 2.0, egui::Stroke::new(1.5, ACCENT));
                        for corner in [sel.left_top(), sel.right_top(), sel.left_bottom(), sel.right_bottom()] {
                            painter.rect_filled(
                                egui::Rect::from_center_size(corner, egui::vec2(6.0, 6.0)),
                                1.0,
                                ACCENT,
                            );
                        }
                    }
                }
            }
        }
    }

    fn hit_test(&mut self, screen_pos: egui::Pos2, canvas_origin: egui::Pos2) {
        // Test in reverse order so topmost element wins
        for elem in self.canvas.elements.iter().rev() {
            match elem {
                CanvasElement::Text(t) => {
                    let elem_screen = canvas_origin + egui::vec2(t.pos.x * PX_PER_INCH, t.pos.y * PX_PER_INCH);
                    let font_px = (t.font_size as f32 * PX_PER_INCH / 203.0).clamp(8.0, 64.0);
                    // Approximate bounding box (0.6 × font_px per char is a reasonable mono estimate)
                    let approx_w = t.content.len() as f32 * font_px * 0.55 + 6.0;
                    let approx_h = font_px * 1.3 + 6.0;
                    let elem_rect = egui::Rect::from_min_size(
                        elem_screen - egui::vec2(3.0, 3.0),
                        egui::vec2(approx_w, approx_h),
                    );
                    if elem_rect.contains(screen_pos) {
                        self.canvas.selected_id = Some(t.id);
                        return;
                    }
                }
            }
        }
        self.canvas.selected_id = None;
    }

    fn draw_properties(&mut self, ui: &mut egui::Ui) {
        if self.canvas.selected_id.is_none() {
            ui.label(
                egui::RichText::new("Nothing selected")
                    .italics()
                    .color(egui::Color32::from_gray(120)),
            );
            ui.add_space(4.0);
            ui.label("Click an element on the canvas to edit it.");
            ui.add_space(12.0);
            ui.separator();
            ui.add_space(8.0);
            ui.label(egui::RichText::new("Label").strong());
            ui.add_space(4.0);
            ui.label(format!(
                "{:.2}\" × {:.2}\" @ {} dpi",
                self.canvas.label_width_in, self.canvas.label_height_in, self.canvas.dpi
            ));
            ui.label(format!(
                "{} × {} dots",
                (self.canvas.label_width_in * self.canvas.dpi as f32) as u32,
                (self.canvas.label_height_in * self.canvas.dpi as f32) as u32,
            ));
            return;
        }

        // Text element properties
        if let Some(t) = self.canvas.selected_text_mut() {
            ui.label(egui::RichText::new("Text").strong());
            ui.add_space(6.0);

            ui.label("Content:");
            ui.text_edit_singleline(&mut t.content);
            ui.add_space(8.0);

            ui.label("Position:");
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
            ui.add_space(8.0);

            ui.label("Font size (dots):");
            ui.add(egui::Slider::new(&mut t.font_size, 10..=120).suffix(" dots"));
            ui.add_space(6.0);

            ui.horizontal(|ui| {
                // Bold toggle with accent colour when active
                let bold_text = egui::RichText::new("  B  ").strong();
                let bold_btn = egui::Button::new(bold_text)
                    .fill(if t.bold { ACCENT.linear_multiply(0.6) } else { egui::Color32::from_gray(60) });
                if ui.add(bold_btn).clicked() {
                    t.bold = !t.bold;
                }
            });
            ui.add_space(12.0);

            ui.separator();
            ui.add_space(8.0);

            ui.label(egui::RichText::new("ZPL preview:").weak());
            let preview = format!(
                "^FO{},{}\n^A{}N,{},{}\n^FD{}^FS",
                (t.pos.x * 203.0) as u32,
                (t.pos.y * 203.0) as u32,
                if t.bold { "B" } else { "A" },
                t.font_size, t.font_size,
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
}

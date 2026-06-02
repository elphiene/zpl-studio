// Edit Mode UI - Direct ZPL editing

use crate::preview;
use eframe::egui;
use image::DynamicImage;

pub struct EditModePanel {
    pub zpl_text: String,
    pub status_message: String,
    pub preview_image: Option<DynamicImage>,
    pub preview_texture: Option<egui::TextureHandle>,
    pub is_modified: bool,
}

impl Default for EditModePanel {
    fn default() -> Self {
        Self {
            zpl_text: String::new(),
            status_message: "Load a template or type ZPL code".to_string(),
            preview_image: None,
            preview_texture: None,
            is_modified: false,
        }
    }
}

impl EditModePanel {
    pub fn ui(&mut self, ui: &mut egui::Ui, has_template: bool) -> EditModeAction {
        let mut action = EditModeAction::None;

        // Horizontal layout: Editor on left, Preview on right
        ui.horizontal_top(|ui| {
            // Left side: ZPL editor
            ui.vertical(|ui| {
                ui.set_min_width(400.0);
                ui.label(egui::RichText::new("ZPL Code:").strong());
                ui.add_space(5.0);

                // Fixed-size scrollable text area
                let scroll_area = egui::ScrollArea::vertical()
                    .max_height(400.0)
                    .show(ui, |ui| {
                        let text_edit = egui::TextEdit::multiline(&mut self.zpl_text)
                            .font(egui::TextStyle::Monospace)
                            .desired_width(400.0)
                            .code_editor();

                        ui.add(text_edit)
                    });

                if scroll_area.inner.changed() {
                    self.is_modified = true;
                    // Trigger autosave if template is loaded, otherwise just update preview
                    action = if has_template {
                        EditModeAction::AutoSave
                    } else {
                        EditModeAction::UpdatePreview
                    };
                }

                ui.add_space(10.0);

                // Action buttons
                ui.horizontal(|ui| {
                    if ui.button(egui::RichText::new("🖨 Print").size(14.0)).clicked() {
                        action = EditModeAction::Print;
                    }

                    ui.add_space(10.0);

                    if ui.button(egui::RichText::new("👁 Update Preview").size(14.0)).clicked() {
                        action = EditModeAction::UpdatePreview;
                    }

                    // Show autosave status
                    if has_template {
                        ui.add_space(10.0);
                        let status_text = if self.is_modified {
                            "⏳ Saving..."
                        } else {
                            "✓ Saved"
                        };
                        ui.label(egui::RichText::new(status_text).italics());
                    }
                });
            });

            ui.add_space(20.0);

            // Right side: Preview
            ui.vertical(|ui| {
                ui.label(egui::RichText::new("Preview:").strong());
                ui.add_space(5.0);

                if let Some(texture) = &self.preview_texture {
                    ui.image((texture.id(), texture.size_vec2()));
                } else {
                    ui.label("Type ZPL code to see preview");
                }
            });
        });

        action
    }

    pub fn load_template(&mut self, zpl: String) {
        self.zpl_text = zpl;
        self.is_modified = false;
        self.status_message = "Template loaded".to_string();
    }

    pub fn get_zpl(&self) -> &str {
        &self.zpl_text
    }

    pub fn mark_saved(&mut self) {
        self.is_modified = false;
    }

    pub fn update_preview(&mut self, ctx: &egui::Context) -> Result<(), String> {
        // Don't try to render empty ZPL
        if self.zpl_text.trim().is_empty() {
            self.preview_texture = None;
            self.preview_image = None;
            return Ok(());
        }

        // Render preview using zpl-forge
        // Using 2.0" x 1.2" @ 203 DPI (standard label size)
        let img = preview::render_preview(&self.zpl_text, 203, 2.0, 1.2)?;

        // Convert to egui texture
        let size = [img.width() as usize, img.height() as usize];
        let rgba = img.to_rgba8();
        let pixels = rgba.as_flat_samples();

        let color_image = egui::ColorImage::from_rgba_unmultiplied(
            size,
            pixels.as_slice(),
        );

        let texture = ctx.load_texture(
            "edit_preview",
            color_image,
            egui::TextureOptions::LINEAR,
        );

        self.preview_image = Some(img);
        self.preview_texture = Some(texture);
        self.status_message = "Preview updated".to_string();

        Ok(())
    }
}

pub enum EditModeAction {
    None,
    AutoSave,
    Print,
    UpdatePreview,
}

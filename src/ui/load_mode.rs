// Load Mode UI - Load templates and fill in variables

use crate::preview;
use crate::zpl::ZplTemplate;
use eframe::egui;
use image::DynamicImage;
use std::collections::HashMap;
use std::path::PathBuf;

pub struct LoadModePanel {
    pub template: Option<ZplTemplate>,
    pub variable_values: HashMap<String, String>,
    pub file_path: Option<PathBuf>,
    pub status_message: String,
    pub preview_image: Option<DynamicImage>,
    pub preview_texture: Option<egui::TextureHandle>,
}

impl Default for LoadModePanel {
    fn default() -> Self {
        Self {
            template: None,
            variable_values: HashMap::new(),
            file_path: None,
            status_message: "Click 'Load Template' to begin".to_string(),
            preview_image: None,
            preview_texture: None,
        }
    }
}

impl LoadModePanel {
    pub fn ui(&mut self, ui: &mut egui::Ui) -> LoadModeAction {
        let mut action = LoadModeAction::None;

        // If template loaded, show variable fields
        if let Some(template) = &self.template {
            ui.horizontal_top(|ui| {
                // Left side: Variable fields
                ui.vertical(|ui| {
                    ui.set_min_width(400.0);
                    ui.label(egui::RichText::new("Template Variables:").strong());
                    ui.add_space(5.0);

                    let mut values_changed = false;
                    for var in &template.variables {
                        ui.horizontal(|ui| {
                            ui.label(format!("{}:", var.display_name));
                            let value = self
                                .variable_values
                                .entry(var.name.clone())
                                .or_insert_with(String::new);
                            if ui.add(egui::TextEdit::singleline(value).desired_width(250.0)).changed() {
                                values_changed = true;
                            }
                        });
                        ui.add_space(5.0);
                    }

                    ui.add_space(10.0);

                    // Action buttons
                    ui.horizontal(|ui| {
                        if ui.button(egui::RichText::new("🖨 Print Label").size(14.0)).clicked() {
                            action = LoadModeAction::Print;
                        }

                        ui.add_space(10.0);

                        if ui.button(egui::RichText::new("🔄 Clear").size(14.0)).clicked() {
                            self.variable_values.clear();
                            self.status_message = "Fields cleared".to_string();
                            values_changed = true;
                        }

                        ui.add_space(10.0);

                        if ui.button(egui::RichText::new("👁 Update Preview").size(14.0)).clicked() {
                            action = LoadModeAction::UpdatePreview;
                        }
                    });

                    // Auto-update preview when values change
                    if values_changed {
                        action = LoadModeAction::UpdatePreview;
                    }
                });

                ui.add_space(20.0);

                // Right side: Preview
                ui.vertical(|ui| {
                    ui.label(egui::RichText::new("Preview:").strong());
                    ui.add_space(5.0);

                    if let Some(texture) = &self.preview_texture {
                        ui.image((texture.id(), texture.size_vec2()));
                    } else {
                        ui.label("Click 'Update Preview' to render");
                    }
                });
            });
        } else {
            ui.label("No template loaded");
            ui.label("Click 'Load Template' at the top to load a .zpl file");
        }

        action
    }

    pub fn load_template(&mut self, path: PathBuf, content: String) {
        self.template = Some(ZplTemplate::from_zpl(content));
        self.variable_values.clear();
        self.file_path = Some(path);
        self.status_message = "Template loaded successfully".to_string();
    }

    pub fn get_rendered_zpl(&self) -> Result<String, String> {
        if let Some(template) = &self.template {
            template.render(&self.variable_values)
        } else {
            Err("No template loaded".to_string())
        }
    }

    pub fn update_preview(&mut self, ctx: &egui::Context) -> Result<(), String> {
        let zpl = self.get_rendered_zpl()?;

        // Render preview using Labelary API
        // Using 2.0" x 1.2" @ 203 DPI (standard label size)
        let img = preview::render_preview(&zpl, 203, 2.0, 1.2)?;

        // Convert to egui texture
        let size = [img.width() as usize, img.height() as usize];
        let rgba = img.to_rgba8();
        let pixels = rgba.as_flat_samples();

        let color_image = egui::ColorImage::from_rgba_unmultiplied(
            size,
            pixels.as_slice(),
        );

        let texture = ctx.load_texture(
            "preview",
            color_image,
            egui::TextureOptions::LINEAR,
        );

        self.preview_image = Some(img);
        self.preview_texture = Some(texture);
        self.status_message = "Preview updated".to_string();

        Ok(())
    }
}

pub enum LoadModeAction {
    None,
    Print,
    UpdatePreview,
}

// Main application state and coordination

use crate::persistence;
use crate::printer::{list_printers, print_raw_zpl, PrintJob, PrinterInfo};
use crate::ui::{DesignModeAction, DesignModePanel, EditModeAction, EditModePanel, LoadModeAction, LoadModePanel};
use eframe::egui;

#[derive(PartialEq)]
enum AppMode {
    Load,
    Design,
    Edit,
}

pub struct ZplPrinterApp {
    mode: AppMode,
    load_panel: LoadModePanel,
    design_panel: DesignModePanel,
    edit_panel: EditModePanel,
    available_printers: Vec<PrinterInfo>,
    selected_printer: Option<String>,
    status_message: String,
}

impl Default for ZplPrinterApp {
    fn default() -> Self {
        let printers = list_printers().unwrap_or_default();
        let selected_printer = printers.first().map(|p| p.name.clone());

        Self {
            mode: AppMode::Load,
            load_panel: LoadModePanel::default(),
            design_panel: DesignModePanel::default(),
            edit_panel: EditModePanel::default(),
            available_printers: printers,
            selected_printer,
            status_message: "Ready".to_string(),
        }
    }
}

impl ZplPrinterApp {
    fn refresh_printers(&mut self) {
        if let Ok(printers) = list_printers() {
            self.available_printers = printers;
            if self.selected_printer.is_none() && !self.available_printers.is_empty() {
                self.selected_printer = Some(self.available_printers[0].name.clone());
            }
            self.status_message = "Printers refreshed".to_string();
        } else {
            self.status_message = "Failed to refresh printers".to_string();
        }
    }

    fn handle_load_mode_action(&mut self, action: LoadModeAction, ctx: &egui::Context) {
        match action {
            LoadModeAction::Print => {
                if let Some(printer_name) = &self.selected_printer {
                    match self.load_panel.get_rendered_zpl() {
                        Ok(zpl) => {
                            let job = PrintJob {
                                printer_name: printer_name.clone(),
                                document_name: "ZPL Label".to_string(),
                                zpl_data: zpl,
                            };
                            match print_raw_zpl(&job) {
                                Ok(_) => {
                                    self.status_message = format!("Printed to {}", printer_name);
                                }
                                Err(e) => {
                                    self.status_message = format!("Print failed: {}", e);
                                }
                            }
                        }
                        Err(e) => {
                            self.status_message = format!("Error: {}", e);
                        }
                    }
                } else {
                    self.status_message = "No printer selected".to_string();
                }
            }
            LoadModeAction::UpdatePreview => {
                match self.load_panel.update_preview(ctx) {
                    Ok(_) => {
                        self.status_message = self.load_panel.status_message.clone();
                    }
                    Err(e) => {
                        self.status_message = format!("Preview failed: {}", e);
                    }
                }
            }
            LoadModeAction::None => {}
        }
    }

    fn handle_design_mode_action(&mut self, action: DesignModeAction) {
        match action {
            DesignModeAction::Print => {
                if let Some(printer_name) = &self.selected_printer {
                    let zpl = self.design_panel.get_zpl();
                    let job = PrintJob {
                        printer_name: printer_name.clone(),
                        document_name: "ZPL Studio Label".to_string(),
                        zpl_data: zpl,
                    };
                    match print_raw_zpl(&job) {
                        Ok(_) => self.status_message = format!("Printed to {}", printer_name),
                        Err(e) => self.status_message = format!("Print failed: {}", e),
                    }
                } else {
                    self.status_message = "No printer selected".to_string();
                }
            }
            DesignModeAction::None => {}
        }
    }

    fn handle_edit_mode_action(&mut self, action: EditModeAction, ctx: &egui::Context) {
        match action {
            EditModeAction::AutoSave => {
                // Auto-save the edited ZPL back to the loaded template file
                if let Some(path) = &self.load_panel.file_path.clone() {
                    let zpl = self.edit_panel.get_zpl();
                    match persistence::save_template(path, zpl) {
                        Ok(_) => {
                            // Re-parse the template for use mode
                            self.load_panel.load_template(path.clone(), zpl.to_string());
                            self.edit_panel.mark_saved();
                            self.status_message = "Auto-saved".to_string();
                            // Update both previews
                            let _ = self.edit_panel.update_preview(ctx);
                            let _ = self.load_panel.update_preview(ctx);
                        }
                        Err(e) => {
                            self.status_message = format!("Auto-save failed: {}", e);
                        }
                    }
                }
            }
            EditModeAction::Print => {
                if let Some(printer_name) = &self.selected_printer {
                    let job = PrintJob {
                        printer_name: printer_name.clone(),
                        document_name: "ZPL Label".to_string(),
                        zpl_data: self.edit_panel.get_zpl().to_string(),
                    };
                    match print_raw_zpl(&job) {
                        Ok(_) => {
                            self.status_message = format!("Printed to {}", printer_name);
                        }
                        Err(e) => {
                            self.status_message = format!("Print failed: {}", e);
                        }
                    }
                } else {
                    self.status_message = "No printer selected".to_string();
                }
            }
            EditModeAction::UpdatePreview => {
                match self.edit_panel.update_preview(ctx) {
                    Ok(_) => {
                        self.status_message = self.edit_panel.status_message.clone();
                    }
                    Err(e) => {
                        self.status_message = format!("Preview failed: {}", e);
                    }
                }
            }
            EditModeAction::None => {}
        }
    }
}

impl eframe::App for ZplPrinterApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            // Header
            ui.heading("ZPL Studio");
            ui.add_space(10.0);

            // Global template loader
            let mut template_loaded = false;
            ui.horizontal(|ui| {
                if ui.button("📁 Load Template").clicked() {
                    if let Some(path) = persistence::open_file_dialog() {
                        match persistence::load_template(&path) {
                            Ok(content) => {
                                // Load into both panels so template is available in both modes
                                self.load_panel.load_template(path, content.clone());
                                self.edit_panel.load_template(content);
                                self.status_message = self.load_panel.status_message.clone();
                                self.mode = AppMode::Load; // Switch to Use mode
                                template_loaded = true;
                            }
                            Err(e) => {
                                self.status_message = format!("Failed to load file: {}", e);
                            }
                        }
                    }
                }

                if let Some(path) = &self.load_panel.file_path {
                    ui.label(format!("📄 {}", path.file_name().unwrap().to_string_lossy()));
                }
            });

            // Auto-generate preview when template is loaded
            if template_loaded {
                if let Err(e) = self.load_panel.update_preview(ctx) {
                    self.status_message = format!("Preview failed: {}", e);
                }
            }

            ui.add_space(10.0);

            // Mode selector
            ui.horizontal(|ui| {
                ui.label("Mode:");
                if ui.selectable_label(self.mode == AppMode::Load, "📄 Use").clicked() {
                    self.mode = AppMode::Load;
                }
                if ui.selectable_label(self.mode == AppMode::Design, "🎨 Design").clicked() {
                    self.mode = AppMode::Design;
                }
                if ui.selectable_label(self.mode == AppMode::Edit, "✏ Edit ZPL").clicked() {
                    self.mode = AppMode::Edit;
                }
            });

            ui.add_space(5.0);

            // Printer selector
            ui.horizontal(|ui| {
                ui.label("Printer:");
                egui::ComboBox::from_id_source("printer_select")
                    .selected_text(
                        self.selected_printer
                            .as_deref()
                            .unwrap_or("No printer selected"),
                    )
                    .show_ui(ui, |ui| {
                        for printer in &self.available_printers {
                            let label = if printer.is_default {
                                format!("{} (Default)", printer.name)
                            } else {
                                printer.name.clone()
                            };
                            ui.selectable_value(
                                &mut self.selected_printer,
                                Some(printer.name.clone()),
                                label,
                            );
                        }
                    });

                if ui.button("🔄").clicked() {
                    self.refresh_printers();
                }
            });

            ui.add_space(10.0);
            ui.separator();
            ui.add_space(10.0);

            // Mode-specific panel - collect actions
            enum PanelAction {
                Load(LoadModeAction),
                Design(DesignModeAction),
                Edit(EditModeAction),
            }

            let has_template = self.load_panel.file_path.is_some();
            let action = match self.mode {
                AppMode::Load => PanelAction::Load(self.load_panel.ui(ui)),
                AppMode::Design => PanelAction::Design(self.design_panel.ui(ui)),
                AppMode::Edit => PanelAction::Edit(self.edit_panel.ui(ui, has_template)),
            };

            // Status bar at bottom
            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                ui.separator();
                ui.horizontal(|ui| {
                    ui.label(&self.status_message);
                });
            });

            // Handle actions after UI rendering is complete
            match action {
                PanelAction::Load(a) => self.handle_load_mode_action(a, ctx),
                PanelAction::Design(a) => self.handle_design_mode_action(a),
                PanelAction::Edit(a) => self.handle_edit_mode_action(a, ctx),
            }
        });
    }
}

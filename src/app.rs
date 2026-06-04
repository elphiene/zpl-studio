use crate::canvas::ACCENT;
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
            mode: AppMode::Design,
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
        match list_printers() {
            Ok(printers) => {
                self.available_printers = printers;
                if self.selected_printer.is_none() && !self.available_printers.is_empty() {
                    self.selected_printer = Some(self.available_printers[0].name.clone());
                }
                self.status_message = "Printers refreshed".to_string();
            }
            Err(_) => self.status_message = "Failed to refresh printers".to_string(),
        }
    }

    fn handle_load_mode_action(&mut self, action: LoadModeAction, ctx: &egui::Context) {
        match action {
            LoadModeAction::Print => {
                if let Some(printer_name) = &self.selected_printer.clone() {
                    match self.load_panel.get_rendered_zpl() {
                        Ok(zpl) => {
                            let job = PrintJob { printer_name: printer_name.clone(), document_name: "ZPL Label".to_string(), zpl_data: zpl };
                            match print_raw_zpl(&job) {
                                Ok(_) => self.status_message = format!("Printed to {}", printer_name),
                                Err(e) => self.status_message = format!("Print failed: {}", e),
                            }
                        }
                        Err(e) => self.status_message = format!("Error: {}", e),
                    }
                } else {
                    self.status_message = "No printer selected".to_string();
                }
            }
            LoadModeAction::UpdatePreview => {
                match self.load_panel.update_preview(ctx) {
                    Ok(_) => self.status_message = self.load_panel.status_message.clone(),
                    Err(e) => self.status_message = format!("Preview failed: {}", e),
                }
            }
            LoadModeAction::None => {}
        }
    }

    fn handle_design_mode_action(&mut self, action: DesignModeAction) {
        match action {
            DesignModeAction::Print => {
                if let Some(printer_name) = &self.selected_printer.clone() {
                    let zpl = self.design_panel.get_zpl();
                    let job = PrintJob { printer_name: printer_name.clone(), document_name: "ZPL Studio Label".to_string(), zpl_data: zpl };
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
                if let Some(path) = &self.load_panel.file_path.clone() {
                    let zpl = self.edit_panel.get_zpl();
                    match persistence::save_template(path, zpl) {
                        Ok(_) => {
                            self.load_panel.load_template(path.clone(), zpl.to_string());
                            self.edit_panel.mark_saved();
                            self.status_message = "Auto-saved".to_string();
                            let _ = self.edit_panel.update_preview(ctx);
                            let _ = self.load_panel.update_preview(ctx);
                        }
                        Err(e) => self.status_message = format!("Auto-save failed: {}", e),
                    }
                }
            }
            EditModeAction::Print => {
                if let Some(printer_name) = &self.selected_printer.clone() {
                    let job = PrintJob { printer_name: printer_name.clone(), document_name: "ZPL Label".to_string(), zpl_data: self.edit_panel.get_zpl().to_string() };
                    match print_raw_zpl(&job) {
                        Ok(_) => self.status_message = format!("Printed to {}", printer_name),
                        Err(e) => self.status_message = format!("Print failed: {}", e),
                    }
                } else {
                    self.status_message = "No printer selected".to_string();
                }
            }
            EditModeAction::UpdatePreview => {
                match self.edit_panel.update_preview(ctx) {
                    Ok(_) => self.status_message = self.edit_panel.status_message.clone(),
                    Err(e) => self.status_message = format!("Preview failed: {}", e),
                }
            }
            EditModeAction::None => {}
        }
    }

    fn apply_theme(&self, ctx: &egui::Context) {
        let mut vis = egui::Visuals::dark();
        vis.selection.bg_fill = ACCENT.linear_multiply(0.30);
        vis.selection.stroke = egui::Stroke::new(1.5, ACCENT);
        vis.widgets.hovered.bg_stroke = egui::Stroke::new(1.0, ACCENT.linear_multiply(0.6));
        vis.widgets.active.bg_fill = ACCENT.linear_multiply(0.45);
        vis.hyperlink_color = ACCENT;
        ctx.set_visuals(vis);
    }
}

impl eframe::App for ZplPrinterApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.apply_theme(ctx);

        // ── Always-visible header ────────────────────────────────────
        egui::TopBottomPanel::top("app_header")
            .frame(egui::Frame::side_top_panel(&ctx.style()).inner_margin(egui::Margin::symmetric(10.0, 6.0)))
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.heading("ZPL Studio");
                    ui.add_space(16.0);

                    // Mode tabs
                    for (label, mode) in [
                        ("📄  Use", AppMode::Load),
                        ("🎨  Design", AppMode::Design),
                        ("✏  Edit ZPL", AppMode::Edit),
                    ] {
                        let active = self.mode == mode;
                        let fill = if active { ACCENT.linear_multiply(0.45) } else { egui::Color32::TRANSPARENT };
                        let stroke = if active { egui::Stroke::new(1.5, ACCENT) } else { egui::Stroke::new(1.0, egui::Color32::from_gray(60)) };
                        let btn = egui::Button::new(egui::RichText::new(label).strong())
                            .fill(fill)
                            .stroke(stroke);
                        if ui.add(btn).clicked() { self.mode = mode; }
                    }

                    ui.add_space(16.0);
                    ui.separator();
                    ui.add_space(8.0);

                    // Template loader (only for Load/Edit modes)
                    if self.mode != AppMode::Design {
                        let mut template_loaded = false;
                        if ui.button("📁 Load Template").clicked() {
                            if let Some(path) = persistence::open_file_dialog() {
                                match persistence::load_template(&path) {
                                    Ok(content) => {
                                        self.load_panel.load_template(path, content.clone());
                                        self.edit_panel.load_template(content);
                                        self.status_message = self.load_panel.status_message.clone();
                                        self.mode = AppMode::Load;
                                        template_loaded = true;
                                    }
                                    Err(e) => self.status_message = format!("Failed to load: {}", e),
                                }
                            }
                        }
                        if template_loaded {
                            let _ = self.load_panel.update_preview(ctx);
                        }
                        if let Some(path) = &self.load_panel.file_path {
                            ui.label(
                                egui::RichText::new(format!("📄 {}", path.file_name().unwrap_or_default().to_string_lossy()))
                                    .color(egui::Color32::from_gray(180)),
                            );
                        }
                        ui.add_space(8.0);
                        ui.separator();
                        ui.add_space(8.0);
                    }

                    // Printer selector — right-aligned
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if ui.small_button("🔄").on_hover_text("Refresh printers").clicked() {
                            self.refresh_printers();
                        }
                        egui::ComboBox::from_id_source("printer_select")
                            .selected_text(self.selected_printer.as_deref().unwrap_or("No printer"))
                            .width(180.0)
                            .show_ui(ui, |ui| {
                                for printer in &self.available_printers {
                                    let label = if printer.is_default {
                                        format!("{} (Default)", printer.name)
                                    } else {
                                        printer.name.clone()
                                    };
                                    ui.selectable_value(&mut self.selected_printer, Some(printer.name.clone()), label);
                                }
                            });
                        ui.label(egui::RichText::new("Printer:").color(egui::Color32::from_gray(160)));
                    });
                });
            });

        // ── Always-visible status bar ───────────────────────────────
        egui::TopBottomPanel::bottom("app_status")
            .frame(egui::Frame::side_top_panel(&ctx.style()).inner_margin(egui::Margin::symmetric(10.0, 4.0)))
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.label(
                        egui::RichText::new(&self.status_message)
                            .size(11.0)
                            .color(egui::Color32::from_gray(170)),
                    );
                    // Design mode status hint
                    if self.mode == AppMode::Design {
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            ui.label(
                                egui::RichText::new(&self.design_panel.status_message)
                                    .size(11.0)
                                    .color(egui::Color32::from_gray(120)),
                            );
                        });
                    }
                });
            });

        // ── Design mode: Illustrator-style panels ───────────────────
        if self.mode == AppMode::Design {
            // Left tool palette
            egui::SidePanel::left("design_tools")
                .resizable(false)
                .exact_width(52.0)
                .frame(egui::Frame::side_top_panel(&ctx.style()))
                .show(ctx, |ui| {
                    self.design_panel.ui_tools(ui);
                });

            // Right properties panel
            egui::SidePanel::right("design_props")
                .min_width(200.0)
                .default_width(220.0)
                .max_width(280.0)
                .frame(egui::Frame::side_top_panel(&ctx.style()))
                .show(ctx, |ui| {
                    egui::ScrollArea::vertical().show(ui, |ui| {
                        self.design_panel.ui_properties(ui);
                    });
                });

            // Design toolbar (below header, above canvas)
            egui::TopBottomPanel::top("design_toolbar")
                .frame(egui::Frame::side_top_panel(&ctx.style()).inner_margin(egui::Margin::symmetric(8.0, 4.0)))
                .show(ctx, |ui| {
                    let action = self.design_panel.ui_toolbar(ui);
                    self.handle_design_mode_action(action);
                });

            // Canvas fills the rest
            egui::CentralPanel::default()
                .frame(egui::Frame::central_panel(&ctx.style()).fill(egui::Color32::from_gray(38)))
                .show(ctx, |ui| {
                    self.design_panel.ui_canvas(ui);
                });

            return;
        }

        // ── Load / Edit mode: original single-panel layout ───────────
        egui::CentralPanel::default().show(ctx, |ui| {
            enum PanelAction {
                Load(LoadModeAction),
                Edit(EditModeAction),
            }

            let has_template = self.load_panel.file_path.is_some();
            let action = match self.mode {
                AppMode::Load => PanelAction::Load(self.load_panel.ui(ui)),
                AppMode::Edit => PanelAction::Edit(self.edit_panel.ui(ui, has_template)),
                AppMode::Design => unreachable!(),
            };

            match action {
                PanelAction::Load(a) => self.handle_load_mode_action(a, ctx),
                PanelAction::Edit(a) => self.handle_edit_mode_action(a, ctx),
            }
        });
    }
}

// ZPL Printer Tool
// A flexible application for printing ZPL labels with template support

#![cfg_attr(windows, windows_subsystem = "windows")] // Hide console window on Windows

use eframe::egui;

mod app;
mod canvas;
mod persistence;
mod preview;
mod printer;
mod ui;
mod zpl;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1000.0, 600.0])
            .with_resizable(true)
            .with_title("ZPL Studio"),
        ..Default::default()
    };

    eframe::run_native(
        "ZPL Studio",
        options,
        Box::new(|_cc| Box::<app::ZplPrinterApp>::default()),
    )
}

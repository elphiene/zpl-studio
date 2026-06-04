// Linux CUPS printer implementation using lp command

use crate::printer::types::{PrintJob, PrinterError, PrinterInfo};
use std::process::Command;
use std::io::Write;
use std::fs;

/// List all available CUPS printers using lpstat command
pub fn list_printers() -> Result<Vec<PrinterInfo>, PrinterError> {
    // Get default printer
    let default_output = Command::new("lpstat")
        .arg("-d")
        .output()
        .map_err(|e| PrinterError::WindowsApiError(
            format!("Failed to run lpstat: {}. Make sure CUPS is installed.", e)
        ))?;

    let default_printer = if default_output.status.success() {
        String::from_utf8_lossy(&default_output.stdout)
            .split(':')
            .nth(1)
            .map(|s| s.trim().to_string())
    } else {
        None
    };

    // Get all printers
    let output = Command::new("lpstat")
        .arg("-p")
        .output()
        .map_err(|e| PrinterError::WindowsApiError(
            format!("Failed to enumerate printers: {}", e)
        ))?;

    if !output.status.success() {
        return Err(PrinterError::WindowsApiError(
            "lpstat command failed. Make sure CUPS is running.".to_string()
        ));
    }

    let mut printers = Vec::new();
    let stdout = String::from_utf8_lossy(&output.stdout);

    for line in stdout.lines() {
        // lpstat -p output format: "printer <name> is ..."
        if let Some(name) = line.split_whitespace().nth(1) {
            let is_default = default_printer.as_ref().map(|d| d == name).unwrap_or(false);
            printers.push(PrinterInfo {
                name: name.to_string(),
                is_default,
            });
        }
    }

    if printers.is_empty() {
        return Err(PrinterError::NoPrinterFound);
    }

    Ok(printers)
}

/// Print raw ZPL data to a CUPS printer using lp command
pub fn print_raw_zpl(job: &PrintJob) -> Result<(), PrinterError> {
    // Write ZPL to a temp file — more reliable than stdin for Zebra raw queues
    let tmp_path = std::env::temp_dir().join("zpl_studio_print.zpl");
    fs::write(&tmp_path, job.zpl_data.as_bytes())
        .map_err(|e| PrinterError::PrintFailed(
            format!("Failed to write temp ZPL file: {}", e)
        ))?;

    let status = Command::new("lp")
        .arg("-d")
        .arg(&job.printer_name)
        .arg("-oraw")
        .arg("-t")
        .arg(&job.document_name)
        .arg(&tmp_path)
        .status()
        .map_err(|e| PrinterError::OpenFailed(
            format!("Failed to start lp command: {}", e)
        ))?;

    let _ = fs::remove_file(&tmp_path);

    if !status.success() {
        return Err(PrinterError::PrintFailed(
            format!("lp command exited with status: {}", status)
        ));
    }

    Ok(())
}

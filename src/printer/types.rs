// Printer module types and errors

use std::fmt;

/// Information about an available printer
#[derive(Debug, Clone)]
pub struct PrinterInfo {
    pub name: String,
    pub is_default: bool,
}

/// A print job to be sent to the printer
pub struct PrintJob {
    pub printer_name: String,
    pub document_name: String,
    pub zpl_data: String,
}

/// Printer-related errors
#[derive(Debug)]
pub enum PrinterError {
    NoPrinterFound,
    PrinterNotAvailable(String),
    OpenFailed(String),
    PrintFailed(String),
    WindowsApiError(String),
}

impl fmt::Display for PrinterError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PrinterError::NoPrinterFound => write!(f, "No printer found"),
            PrinterError::PrinterNotAvailable(name) => {
                write!(f, "Printer '{}' is not available", name)
            }
            PrinterError::OpenFailed(name) => write!(f, "Failed to open printer '{}'", name),
            PrinterError::PrintFailed(msg) => write!(f, "Print failed: {}", msg),
            PrinterError::WindowsApiError(msg) => write!(f, "Windows API error: {}", msg),
        }
    }
}

impl std::error::Error for PrinterError {}

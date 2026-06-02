// Windows printer implementation

use super::types::{PrintJob, PrinterError, PrinterInfo};

#[cfg(windows)]
use windows_sys::Win32::Foundation::{BOOL, HANDLE};
#[cfg(windows)]
use windows_sys::Win32::Graphics::Printing::*;

/// List all available Windows printers
#[cfg(windows)]
pub fn list_printers() -> Result<Vec<PrinterInfo>, PrinterError> {
    use std::ptr;

    unsafe {
        // First, get the default printer
        let mut needed: u32 = 0;
        GetDefaultPrinterW(ptr::null_mut(), &mut needed);

        let default_printer = if needed > 0 {
            let mut buffer = vec![0u16; needed as usize];
            if GetDefaultPrinterW(buffer.as_mut_ptr(), &mut needed) != 0 {
                Some(String::from_utf16_lossy(&buffer[..needed as usize - 1]))
            } else {
                None
            }
        } else {
            None
        };

        // For now, just return the default printer
        // TODO: Implement EnumPrintersW to get all printers
        if let Some(name) = default_printer {
            Ok(vec![PrinterInfo {
                name,
                is_default: true,
            }])
        } else {
            Err(PrinterError::NoPrinterFound)
        }
    }
}

#[cfg(not(windows))]
pub fn list_printers() -> Result<Vec<PrinterInfo>, PrinterError> {
    Err(PrinterError::WindowsApiError(
        "Printing only supported on Windows".to_string(),
    ))
}

/// Print ZPL to a specific Windows printer
#[cfg(windows)]
pub fn print_raw_zpl(job: &PrintJob) -> Result<(), PrinterError> {
    // OpenPrinterW isn't in windows-sys 0.52, declare it ourselves
    #[link(name = "winspool")]
    extern "system" {
        fn OpenPrinterW(
            pPrinterName: *mut u16,
            phPrinter: *mut HANDLE,
            pDefault: *mut std::ffi::c_void,
        ) -> BOOL;
    }

    unsafe {
        // Convert printer name to UTF-16
        let mut printer_name_wide: Vec<u16> = job.printer_name.encode_utf16().chain(std::iter::once(0)).collect();

        // Open printer
        let mut h_printer: HANDLE = 0;
        let result: BOOL = OpenPrinterW(
            printer_name_wide.as_mut_ptr(),
            &mut h_printer,
            std::ptr::null_mut(),
        );

        if result == 0 {
            return Err(PrinterError::OpenFailed(job.printer_name.clone()));
        }

        // Prepare document info
        let mut doc_name: Vec<u16> = format!("{}\0", job.document_name).encode_utf16().collect();
        let mut datatype: Vec<u16> = "RAW\0".encode_utf16().collect();

        let doc_info = DOC_INFO_1W {
            pDocName: doc_name.as_mut_ptr(),
            pOutputFile: std::ptr::null_mut(),
            pDatatype: datatype.as_mut_ptr(),
        };

        // Start document
        let job_id: u32 = StartDocPrinterW(h_printer, 1, &doc_info);
        if job_id == 0 {
            ClosePrinter(h_printer);
            return Err(PrinterError::PrintFailed("Failed to start document".to_string()));
        }

        // Start page
        let result: BOOL = StartPagePrinter(h_printer);
        if result == 0 {
            EndDocPrinter(h_printer);
            ClosePrinter(h_printer);
            return Err(PrinterError::PrintFailed("Failed to start page".to_string()));
        }

        // Write ZPL data
        let zpl_bytes = job.zpl_data.as_bytes();
        let mut written: u32 = 0;

        let result: BOOL = WritePrinter(
            h_printer,
            zpl_bytes.as_ptr() as *const _,
            zpl_bytes.len() as u32,
            &mut written,
        );

        if result == 0 {
            EndPagePrinter(h_printer);
            EndDocPrinter(h_printer);
            ClosePrinter(h_printer);
            return Err(PrinterError::PrintFailed("Failed to write data".to_string()));
        }

        // Clean up
        EndPagePrinter(h_printer);
        EndDocPrinter(h_printer);
        ClosePrinter(h_printer);

        Ok(())
    }
}

#[cfg(not(windows))]
pub fn print_raw_zpl(_job: &PrintJob) -> Result<(), PrinterError> {
    Err(PrinterError::WindowsApiError(
        "Printing only supported on Windows".to_string(),
    ))
}

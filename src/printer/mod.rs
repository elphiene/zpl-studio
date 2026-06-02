// Printer module - Cross-platform printer communication

pub mod types;

// Platform-specific printer implementations
#[cfg(windows)]
pub mod windows;

#[cfg(unix)]
pub mod cups;

// Re-export commonly used types
pub use types::{PrintJob, PrinterError, PrinterInfo};

// Re-export platform-specific functions
#[cfg(windows)]
pub use windows::{list_printers, print_raw_zpl};

#[cfg(unix)]
pub use cups::{list_printers, print_raw_zpl};

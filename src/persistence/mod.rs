// Persistence module - File I/O operations

pub mod file_ops;

// Re-export
pub use file_ops::{load_template, open_file_dialog, save_file_dialog, save_template};

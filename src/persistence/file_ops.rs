// File operations for ZPL templates

use std::fs;
use std::io;
use std::path::{Path, PathBuf};

/// Load a ZPL template from a file
pub fn load_template(path: &Path) -> io::Result<String> {
    fs::read_to_string(path)
}

/// Save ZPL content to a file
pub fn save_template(path: &Path, content: &str) -> io::Result<()> {
    fs::write(path, content)
}

/// Open a file dialog to select a ZPL file
pub fn open_file_dialog() -> Option<PathBuf> {
    rfd::FileDialog::new()
        .add_filter("ZPL Template", &["zpl"])
        .add_filter("Text Files", &["txt"])
        .add_filter("All Files", &["*"])
        .pick_file()
}

/// Open a save file dialog
pub fn save_file_dialog() -> Option<PathBuf> {
    rfd::FileDialog::new()
        .add_filter("ZPL Template", &["zpl"])
        .set_file_name("template.zpl")
        .save_file()
}

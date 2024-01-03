use std::fs;
use std::fs::{DirBuilder, File};
use std::io::Write;
use std::path::Path;
use anyhow::Context;
use crate::error::handle_error_finish_execution;

pub fn create_directory<P: AsRef<Path>>(path: P, recursive: bool) {
    DirBuilder::new().recursive(recursive).create(&path)
        .with_context(|| format!("could not create '{}' directory", path.as_ref().to_string_lossy()))
        .unwrap_or_else(handle_error_finish_execution);
}

pub fn create_file<P: AsRef<Path>>(path: P) -> File {
    File::create(&path).with_context(|| format!("could not create '{}' file", path.as_ref().to_string_lossy()))
        .unwrap_or_else(handle_error_finish_execution)
}

pub fn check_exists<P: AsRef<Path>>(path: P) -> bool {
    path.as_ref().try_exists()
        .with_context(|| format!("could not check for existence of {}", path.as_ref().to_string_lossy()))
        .unwrap_or_else(handle_error_finish_execution)
}

pub fn write_all_to_file(file: &mut File, bytes: &[u8], path: String) {
    file.write_all(bytes)
        .with_context(|| format!("could not write to '{}' file", path))
        .unwrap_or_else(handle_error_finish_execution);
}

pub fn check_file_with_extension_exists<P: AsRef<Path>>(directory: P, extension: String) -> bool {
    let entries = fs::read_dir(".")
        .with_context(|| format!("could not check for existance of files with {} extension in {}", &extension,
                                 directory.as_ref().to_string_lossy()))
        .unwrap_or_else(handle_error_finish_execution);

    for entry in entries {
        if let Ok(entry) = entry {
            if entry.file_name().to_string_lossy().ends_with(&extension) {
                return true;
            }
        }
    }

    return false;
}
use std::path::{Path};
use anyhow::{anyhow};
use crate::error::handle_error_finish_execution;
use crate::{PackageKinds, VCSOptions};
use crate::commands::init::init_package;
use crate::common::file_system_utils::*;

pub fn new_package(path: &str, name: Option<String>, category: PackageKinds, vcs: VCSOptions) {
    let package_dir_path = Path::new(&path);
    let package_dir_exists = check_exists(&package_dir_path);

    if !package_dir_exists {
        create_directory(&package_dir_path, true);

        init_package(package_dir_path.to_string_lossy().as_ref(), name, category, vcs);
    }
    else {
        handle_error_finish_execution::<()>(anyhow!("destination '{}' already exists\n\nUse 'artio init' to initialize the directory", package_dir_path.to_string_lossy()));
    }
}
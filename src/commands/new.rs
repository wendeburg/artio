use std::path::{Path};
use anyhow::{anyhow};
use crate::error::handle_error_finish_execution;
use crate::{PackageKind, VCSOptions};
use crate::commands::init::init_package;
use crate::common_utils::file_system_utils::*;

pub fn new_package(path: &str, name: Option<String>, category: PackageKind, vcs: VCSOptions) {
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

#[cfg(test)]
mod test {
    use std::fs::DirBuilder;
    use crate::{PackageKind, VCSOptions};
    use crate::commands::new::new_package;

    #[test]
    #[should_panic]
    fn init_non_existant_dir() {
        let tempdir = tempfile::tempdir().unwrap();
        let new_package_path = tempdir.path().join("testapp");
        let _ = DirBuilder::new().recursive(false).create(&new_package_path).unwrap();

        new_package((*new_package_path.to_string_lossy()).as_ref(), None, PackageKind::Application, VCSOptions::None);
    }
}
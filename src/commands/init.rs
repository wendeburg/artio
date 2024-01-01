use std::path::Path;
use anyhow::anyhow;
use semver::{BuildMetadata, Prerelease, Version};
use crate::{PackageKinds, PackageProperties, VCSOptions};
use crate::common::file_system_utils::*;
use crate::error::handle_error_finish_execution;

pub fn init_package(path: &str, name: Option<String>, category: PackageKinds, vcs: VCSOptions) {
    let package_dir_path = Path::new(path);
    let package_dir_exists = check_exists(package_dir_path);

    if package_dir_exists {
        let package_name = name.unwrap_or_else(|| {
            match package_dir_path.file_name() {
                Some(name) => (*name.to_string_lossy()).to_owned(),
                None => "package".to_owned(),
            }
        });

        let package_properties = PackageProperties {
            name: package_name.clone(),
            kind: category,
            version: Version {
                major: 0,
                minor: 1,
                patch: 0,
                pre: Prerelease::EMPTY,
                build: BuildMetadata::EMPTY
            }
        };

        let artio_package_toml_path = package_dir_path.join("artio_package.toml");

        if check_exists(&artio_package_toml_path) {
            handle_error_finish_execution::<()>(anyhow!("'artio init' cannot be run on an existing artio package"));
        }

        let mut artio_package_toml = create_file(&artio_package_toml_path);
        write_all_to_file(&mut artio_package_toml, toml::to_string(&package_properties).unwrap().as_bytes(), (*artio_package_toml_path.to_string_lossy()).to_owned());

        match category {
            PackageKinds::Application => generate_application_structure(package_dir_path, &package_name),
            PackageKinds::DynamicLib | PackageKinds::StaticLib => generate_library_structure(package_dir_path, &package_name),
        }

        match vcs  {
            VCSOptions::None => (),
            VCSOptions::Git => manage_git_repo_initialization(package_dir_path),
        }

        println!("Created {} '{}' package", category.get_string(), &package_name);
    }
    else {
        handle_error_finish_execution::<()>(anyhow!("destination '{}' does not exist\n\nUse 'artio new' to initialize a new directory", path));
    }
}

fn generate_application_structure<P: AsRef<Path>>(package_dir_path: P, package_name: &str) {
    generate_basic_structure(package_dir_path, package_name, get_application_cpp_file_contents())
}

fn generate_basic_structure<P: AsRef<Path>>(package_dir_path: P, package_name: &str, cpp_file_contents: String) {
    let src_dir_path = package_dir_path.as_ref().join("src");

    if !check_exists(&src_dir_path) {
        create_directory(&src_dir_path, false);
    }

    if !check_file_with_extension_exists(&src_dir_path, ".cpp".to_string()) {
        let cpp_file_path = src_dir_path.join(package_name.to_owned() + ".cpp");

        let mut cpp_file = create_file(&cpp_file_path);
        write_all_to_file(&mut cpp_file, cpp_file_contents.as_bytes(), (*cpp_file_path.to_string_lossy()).to_owned());
    }
}

fn generate_library_structure<P: AsRef<Path>>(package_dir_path: P, package_name: &str) {
    generate_basic_structure(&package_dir_path, package_name, get_library_cpp_file_contents());

    let include_dir_path = package_dir_path.as_ref().join("include");
    if !check_exists(&include_dir_path) {
        create_directory(&include_dir_path, false);
    }

    let include_subdir_path = include_dir_path.join(package_name);
    if !check_exists(&include_subdir_path) {
        create_directory(&include_subdir_path, false);
    }

    if !check_file_with_extension_exists(&include_subdir_path, ".h".to_string()) {
        let h_file_path = include_subdir_path.join(package_name.to_owned() + ".h");

        let mut h_file = create_file(&h_file_path);
        write_all_to_file(&mut h_file, get_library_h_file_contents().as_bytes(), h_file_path.to_string_lossy().to_string());
    }
}

fn get_application_cpp_file_contents() -> String {
    "int main() {\n  return 0;\n}".to_string()
}

fn get_library_cpp_file_contents() -> String {
    "int add(int a, int b) {\n return a + b;\n}".to_string()
}

fn get_library_h_file_contents() -> String {
    "int add(int, int);".to_string()
}
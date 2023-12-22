use std::fs::{DirBuilder, File};
use std::io::Write;
use std::path::{Path};
use std::process::Command;
use anyhow::{Context, anyhow};
use crate::error::handle_error_finish_execution;
use crate::{PackageCategories, PackageProperties, VCSOptions};

pub fn new_package(path: &str, name: Option<String>, category: PackageCategories, vcs: VCSOptions) {
    let package_dir_path = Path::new(path);
    let package_dir_exists = check_exists(package_dir_path);

    if !package_dir_exists {
        let package_name = name.unwrap_or_else(|| {
            match package_dir_path.file_name() {
                Some(name) => name.to_string_lossy().to_string(),
                None => "package".to_owned(),
            }
        });

        let mut package_properties = PackageProperties::new();
        package_properties.name = package_name.clone();
        package_properties.category = category.get_string();
        package_properties.version = "0.1.0".to_string();

        create_directory(package_dir_path, true);

        let artio_package_toml_path = package_dir_path.join("artio_package.toml");
        let mut artio_package_toml = create_file(&artio_package_toml_path);
        write_all_to_file(&mut artio_package_toml, package_properties.as_toml().as_bytes(), artio_package_toml_path.to_string_lossy().to_string());

        match category {
            PackageCategories::Application => generate_application_structure(package_dir_path, package_name.clone()),
            PackageCategories::DynamicLib | PackageCategories::StaticLib => generate_library_structure(package_dir_path, package_name.clone()),
        }

        match vcs  {
            VCSOptions::None => (),
            VCSOptions::Git => manage_git_repo_initialization(package_dir_path),
        }
    }
    else {
        handle_error_finish_execution::<()>(anyhow!("destination '{}' already exists\n\nUse 'artio init' to initialize the directory", package_dir_path.to_string_lossy()));
    }
}

fn manage_git_repo_initialization<P: AsRef<Path>>(path: P) {
    if !check_if_git_repo_exists(&path) && git_is_installed() {
        match Command::new("git").arg("init").output() {
            Ok(_) => (),
            Err(_) => todo!()
        }
    }
}

fn check_if_git_repo_exists<P: AsRef<Path>>(path: P) -> bool {
    match Command::new("git").arg("status").current_dir(path).output() {
        Ok(output) => output.status.success(),
        Err(_) => true,
    }
}

fn git_is_installed() -> bool {
    match Command::new("git").arg("--version").output() {
        Ok(output) => output.status.success(),
        Err(_) => false,
    }
}

fn create_directory<P: AsRef<Path>>(path: P, recursive: bool) {
    DirBuilder::new().recursive(recursive).create(&path)
        .with_context(|| format!("could not create '{}' directory", path.as_ref().to_string_lossy()))
        .unwrap_or_else(handle_error_finish_execution);
}

fn create_file<P: AsRef<Path>>(path: P) -> File {
    File::create(&path).with_context(|| format!("could not create '{}' file", path.as_ref().to_string_lossy()))
        .unwrap_or_else(handle_error_finish_execution)
}

fn check_exists<P: AsRef<Path>>(path: P) -> bool {
    path.as_ref().try_exists()
        .with_context(|| format!("could not check for existence of {}", path.as_ref().to_string_lossy()))
        .unwrap_or_else(handle_error_finish_execution)
}

fn write_all_to_file(file: &mut File, bytes: &[u8], path: String) {
    file.write_all(bytes)
        .with_context(|| format!("could not write to '{}' file", path))
        .unwrap_or_else(handle_error_finish_execution);
}

fn generate_application_structure<P: AsRef<Path>>(package_dir_path: P, package_name: String) {
    let src_dir_path = package_dir_path.as_ref().join("src");
    create_directory(&src_dir_path, false);

    let main_cpp_path = src_dir_path.join(package_name.clone() + ".cpp");
    let mut main_cpp = create_file(&main_cpp_path);
    write_all_to_file(&mut main_cpp, get_application_cpp_file_contents().as_bytes(), main_cpp_path.to_string_lossy().to_string());
}

fn generate_library_structure<P: AsRef<Path>>(package_dir_path: P, package_name: String) {
    let src_dir_path = package_dir_path.as_ref().join("src");
    create_directory(&src_dir_path, false);

    let main_cpp_path = src_dir_path.join(package_name.clone() + ".cpp");
    let mut main_cpp = create_file(&main_cpp_path);
    write_all_to_file(&mut main_cpp, get_library_cpp_file_contents().as_bytes(), main_cpp_path.to_string_lossy().to_string());

    let include_dir_path = package_dir_path.as_ref().join("include");
    create_directory(&include_dir_path, false);

    let include_subdir_path = include_dir_path.join(&package_name);
    create_directory(&include_subdir_path, false);

    let include_h_file_path = include_subdir_path.join(package_name.clone()+ ".h");
    let mut include_h_file = create_file(&include_h_file_path);
    write_all_to_file(&mut include_h_file, get_library_h_file_contents().as_bytes(), include_h_file_path.to_string_lossy().to_string());
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
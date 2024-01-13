use std::path::Path;
use anyhow::anyhow;
use semver::{BuildMetadata, Prerelease, Version};
use crate::{PackageKind, PackageProperties, VCSOptions};
use crate::common_utils::file_system_utils::*;
use crate::error::handle_error_finish_execution;

pub fn init_package(path: &str, name: Option<String>, category: PackageKind, vcs: VCSOptions) {
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
            PackageKind::Application => generate_application_structure(package_dir_path, &package_name),
            PackageKind::DynamicLib | PackageKind::StaticLib => generate_library_structure(package_dir_path, &package_name),
        }

        vcs.initialize_repo(path);

        println!("Created {} '{}' package", category.get_alias(), &package_name);
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

#[cfg(test)]
mod tests {
    use std::fs;
    use std::fs::DirBuilder;
    use semver::{BuildMetadata, Prerelease, Version};
    use crate::commands::init::init_package;
    use crate::{PackageKind, PackageProperties, VCSOptions};

    #[test]
    fn init_application() -> Result<(), Box<dyn std::error::Error>> {
        let tempdir = tempfile::tempdir()?;

        let new_package_path = tempdir.path().join("testapp");

        let _ = DirBuilder::new().recursive(false).create(&new_package_path);

        init_package((*new_package_path.to_string_lossy()).as_ref(), None, PackageKind::Application, VCSOptions::None);

        let cpp_file_path = new_package_path.join("src/testapp.cpp");
        let cpp_file_contents = fs::read_to_string(cpp_file_path)?;
        assert_eq!(cpp_file_contents, super::get_application_cpp_file_contents());

        let artio_package_toml = new_package_path.join("artio_package.toml");
        let package_properties_actual: PackageProperties = toml::from_str(fs::read_to_string(artio_package_toml)?.as_str())?;
        let package_properties_expected = PackageProperties {
            name: "testapp".to_owned(),
            kind: PackageKind::Application,
            version: Version {
                major: 0,
                minor: 1,
                patch: 0,
                pre: Prerelease::EMPTY,
                build: BuildMetadata::EMPTY
            }
        };

        assert_eq!(package_properties_actual, package_properties_expected);

        tempdir.close()?;

        Ok(())
    }

    #[test]
    fn init_application_with_name() -> Result<(), Box<dyn std::error::Error>> {
        let tempdir = tempfile::tempdir()?;

        let new_package_path = tempdir.path().join("testapp");

        let _ = DirBuilder::new().recursive(false).create(&new_package_path);

        init_package((*new_package_path.to_string_lossy()).as_ref(), Some("aname".to_owned()), PackageKind::Application, VCSOptions::None);

        let cpp_file_path = new_package_path.join("src/aname.cpp");
        let cpp_file_contents = fs::read_to_string(cpp_file_path)?;
        assert_eq!(cpp_file_contents, super::get_application_cpp_file_contents());

        let artio_package_toml = new_package_path.join("artio_package.toml");
        let package_properties_actual: PackageProperties = toml::from_str(fs::read_to_string(artio_package_toml)?.as_str())?;
        let package_properties_expected = PackageProperties {
            name: "aname".to_owned(),
            kind: PackageKind::Application,
            version: Version {
                major: 0,
                minor: 1,
                patch: 0,
                pre: Prerelease::EMPTY,
                build: BuildMetadata::EMPTY
            }
        };

        assert_eq!(package_properties_actual, package_properties_expected);

        tempdir.close()?;

        Ok(())
    }

    #[test]
    fn init_dynamic_library() -> Result<(), Box<dyn std::error::Error>> {
        let tempdir = tempfile::tempdir()?;

        let new_package_path = tempdir.path().join("testdynlib");

        let _ = DirBuilder::new().recursive(false).create(&new_package_path);

        init_package((*new_package_path.to_string_lossy()).as_ref(), None, PackageKind::DynamicLib, VCSOptions::None);

        let cpp_file_path = new_package_path.join("src/testdynlib.cpp");
        let cpp_file_contents = fs::read_to_string(cpp_file_path)?;
        assert_eq!(cpp_file_contents, super::get_library_cpp_file_contents());

        let h_file_path = new_package_path.join("include/testdynlib/testdynlib.h");
        let h_file_contents = fs::read_to_string(h_file_path)?;
        assert_eq!(h_file_contents, super::get_library_h_file_contents());

        let artio_package_toml = new_package_path.join("artio_package.toml");
        let package_properties_actual: PackageProperties = toml::from_str(fs::read_to_string(artio_package_toml)?.as_str())?;
        let package_properties_expected = PackageProperties {
            name: "testdynlib".to_owned(),
            kind: PackageKind::DynamicLib,
            version: Version {
                major: 0,
                minor: 1,
                patch: 0,
                pre: Prerelease::EMPTY,
                build: BuildMetadata::EMPTY
            }
        };

        assert_eq!(package_properties_actual, package_properties_expected);

        tempdir.close()?;

        Ok(())
    }

    #[test]
    fn init_static_library() -> Result<(), Box<dyn std::error::Error>> {
        let tempdir = tempfile::tempdir()?;

        let new_package_path = tempdir.path().join("teststaticlib");

        let _ = DirBuilder::new().recursive(false).create(&new_package_path);

        init_package((*new_package_path.to_string_lossy()).as_ref(), None, PackageKind::StaticLib, VCSOptions::None);

        let cpp_file_path = new_package_path.join("src/teststaticlib.cpp");
        let cpp_file_contents = fs::read_to_string(cpp_file_path)?;
        assert_eq!(cpp_file_contents, super::get_library_cpp_file_contents());

        let h_file_path = new_package_path.join("include/teststaticlib/teststaticlib.h");
        let h_file_contents = fs::read_to_string(h_file_path)?;
        assert_eq!(h_file_contents, super::get_library_h_file_contents());

        let artio_package_toml = new_package_path.join("artio_package.toml");
        let package_properties_actual: PackageProperties = toml::from_str(fs::read_to_string(artio_package_toml)?.as_str())?;
        let package_properties_expected = PackageProperties {
            name: "teststaticlib".to_owned(),
            kind: PackageKind::StaticLib,
            version: Version {
                major: 0,
                minor: 1,
                patch: 0,
                pre: Prerelease::EMPTY,
                build: BuildMetadata::EMPTY
            }
        };

        assert_eq!(package_properties_actual, package_properties_expected);

        tempdir.close()?;

        Ok(())
    }

    #[test]
    #[should_panic]
    fn init_non_existant_dir() {
        let tempdir = tempfile::tempdir().unwrap();
        let new_package_path = tempdir.path().join("testapp");

        init_package((*new_package_path.to_string_lossy()).as_ref(), None, PackageKind::Application, VCSOptions::None);
    }
}
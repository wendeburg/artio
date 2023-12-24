use clap::ValueEnum;

pub mod commands;

mod error;

mod common;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum PackageCategories {
    StaticLib,
    DynamicLib,
    Application
}

impl PackageCategories {
    fn get_string(&self) -> String {
        match self {
            PackageCategories::StaticLib => "static-lib".to_string(),
            PackageCategories::DynamicLib => "dynamic-lib".to_string(),
            PackageCategories::Application => "application".to_string()
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum VCSOptions {
    Git,
    None
}

pub struct PackageProperties {
    pub name: String,
    pub category: String,
    pub version: String,
}

impl PackageProperties {
    fn new() -> PackageProperties {
        PackageProperties {
            name: "".to_string(),
            category: "".to_string(),
            version: "".to_string(),
        }
    }

    fn as_toml(&self) -> String {
        let mut file_contents = "[package]\nname = \"".to_string();
        file_contents.push_str(self.name.as_str());
        file_contents.push_str("\"\nversion = ");
        file_contents.push_str(self.version.as_str());
        file_contents.push_str("\ncategory = \"");
        file_contents.push_str(self.category.as_str());
        file_contents.push_str("\"");

        file_contents
    }
}
use clap::ValueEnum;
use semver::Version;
use serde::{Deserialize, Serialize};

pub mod commands;

mod error;

mod common;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Serialize, Deserialize, Debug)]
pub enum PackageKind {
    #[serde(alias="static-lib")]
    StaticLib,
    #[serde(alias="dynamic-lib")]
    DynamicLib,
    #[serde(alias="application")]
    Application
}

impl PackageKind {
    fn get_alias(&self) -> String {
        match self {
            PackageKind::StaticLib => "static-lib".to_string(),
            PackageKind::DynamicLib => "dynamic-lib".to_string(),
            PackageKind::Application => "application".to_string()
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum VCSOptions {
    Git,
    None
}

#[derive(PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct PackageProperties {
    pub name: String,
    pub kind: PackageKind,
    pub version: Version,
}
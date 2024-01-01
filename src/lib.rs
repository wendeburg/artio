use clap::ValueEnum;
use semver::Version;
use serde::{Deserialize, Serialize, Serializer};
use serde::ser::Serialize as SerializeTrait;

pub mod commands;

mod error;

mod common;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum PackageKinds {
    StaticLib,
    DynamicLib,
    Application
}

impl PackageKinds {
    fn get_string(&self) -> String {
        match self {
            PackageKinds::StaticLib => "static-lib".to_string(),
            PackageKinds::DynamicLib => "dynamic-lib".to_string(),
            PackageKinds::Application => "application".to_string()
        }
    }
}

impl SerializeTrait for PackageKinds {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        serializer.serialize_str(&self.get_string())
    }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum VCSOptions {
    Git,
    None
}

#[derive(Serialize, Deserialize)]
pub struct ArtioPackage {
    package: PackageProperties
}

#[derive(Serialize, Deserialize)]
pub struct PackageProperties {
    pub name: String,
    pub kind: PackageKinds,
    pub version: Version,
}
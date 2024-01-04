use std::path::Path;
use clap::ValueEnum;
use semver::Version;
use serde::{Deserialize, Serialize};
use crate::common::vcs::Git;

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

pub trait VCSManager {
    fn check_vcs_installed() -> bool;

    fn check_vcs_repo_exists<P: AsRef<Path>>(path: P) -> bool;

    fn initialize_new_vcs_repo<P: AsRef<Path>>(path: P);
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum VCSOptions {
    Git,
    None
}

impl VCSOptions {
    fn initialize_repo<P: AsRef<Path>>(&self, path: P) {
        match self {
            VCSOptions::None => (),
            VCSOptions::Git => {
                Git::initialize_new_vcs_repo(path);
            },
        }
    }
}

#[derive(PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct PackageProperties {
    pub name: String,
    pub kind: PackageKind,
    pub version: Version,
}
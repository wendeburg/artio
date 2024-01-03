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
    fn check_vcs_installed(&self) -> bool;

    fn check_vcs_repo_exists(&self, path: &Path) -> bool;

    fn initialize_new_vcs_repo(&self, path: &Path);
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum VCSOptions {
    Git,
    None
}

impl VCSOptions {
    fn get_vcs_manager(&self) -> Option<Box<dyn VCSManager>> {
        match self {
            VCSOptions::None => None,
            VCSOptions::Git => Some(Box::new(Git::new())),
        }
    }
}

#[derive(PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct PackageProperties {
    pub name: String,
    pub kind: PackageKind,
    pub version: Version,
}
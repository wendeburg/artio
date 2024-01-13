use std::path::Path;
use std::process::Command;
use anyhow::Context;
use crate::error::handle_warning_print_to_stdout;
use crate::VCSManager;

pub struct Git;

impl VCSManager for Git {
    fn check_vcs_installed() -> bool {
        match Command::new("git").arg("--version").output() {
            Ok(output) => output.status.success(),
            Err(_) => false,
        }
    }

    fn check_vcs_repo_exists<P: AsRef<Path>>(path: P)-> bool {
        match Command::new("git").arg("status").current_dir(path).output() {
            Ok(output) => output.status.success(),
            Err(_) => true,
        }
    }

    fn initialize_new_vcs_repo<P: AsRef<Path>>(path: P) {
        if !Self::check_vcs_repo_exists(&path) && Self::check_vcs_installed() {
            match Command::new("git").arg("init").output().context("Could not create repository") {
                Ok(_) => (),
                Err(error) => handle_warning_print_to_stdout(error)
            }
        }
    }
}
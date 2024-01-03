use std::path::Path;
use std::process::Command;
use crate::VCSManager;

pub struct Git;

impl Git {
    pub fn new() -> Self {
        Git
    }
}

impl VCSManager for Git {
    fn check_vcs_installed(&self) -> bool {
        match Command::new("git").arg("--version").output() {
            Ok(output) => output.status.success(),
            Err(_) => false,
        }
    }

    fn check_vcs_repo_exists(&self, path: &Path) -> bool {
        match Command::new("git").arg("status").current_dir(path).output() {
            Ok(output) => output.status.success(),
            Err(_) => true,
        }
    }

    fn initialize_new_vcs_repo(&self, path: &Path) {
        if !self.check_vcs_repo_exists(&path) && self.check_vcs_installed() {
            match Command::new("git").arg("init").output() {
                Ok(_) => (),
                Err(_) => todo!()
            }
        }
    }
}
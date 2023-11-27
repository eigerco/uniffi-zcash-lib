use std::path::PathBuf;

pub mod kotlin;
pub mod python;
pub mod ruby;
pub mod swift;

pub struct Config {
    pub version: String,
    pub git_repo_url: Option<String>,
    pub package_template_dir: PathBuf,
    pub test_app_template_dir: PathBuf,
    pub bindings_dir: PathBuf,
    pub package_dir: PathBuf,
}

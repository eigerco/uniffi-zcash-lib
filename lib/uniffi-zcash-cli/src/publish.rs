use std::{
    path::{Path, PathBuf},
    process::Command,
};

use fs_extra::file::read_to_string;
use retry::delay::Exponential;

use crate::helper::{cmd_retry, cmd_success};

pub fn python(config: &PythonConfig) -> anyhow::Result<()> {
    // Ensure deps are installed.
    cmd_success(
        Command::new("python3")
            .arg("-m")
            .arg("pip")
            .arg("install")
            .arg("--user")
            .arg("--upgrade")
            .arg("twine")
            .spawn()?
            .wait(),
    )?;

    // Publish the artifact. See twine --help options.
    let mut publish_cmd = Command::new("python3");
    publish_cmd
        .arg("-m")
        .arg("twine")
        .arg("upload")
        .arg("dist/*")
        .env("TWINE_REPOSITORY_URL", &config.registry_url)
        .env("TWINE_USERNAME", &config.registry_username)
        .env("TWINE_PASSWORD", &config.registry_password)
        .current_dir(&config.lang_package_path);

    cmd_retry(
        "Python publication",
        Exponential::from_millis(1000),
        10,
        publish_cmd,
    )
}

pub struct PythonConfig {
    pub lang_package_path: PathBuf,
    pub registry_url: String,
    pub registry_username: String,
    pub registry_password: String,
}

pub fn ruby(config: &RubyConfig) -> anyhow::Result<()> {
    // Publish the artifact. See https://guides.rubygems.org/publishing/
    let mut publish_cmd = Command::new("gem");
    publish_cmd
        .arg("push")
        .arg(format!("zcash-{}.gem", &config.version))
        .arg("--norc")
        .arg("--host")
        .arg(&config.registry_url)
        .env("GEM_HOST_API_KEY", &config.registry_token)
        .current_dir(&config.lang_package_path);

    cmd_retry(
        "Ruby publication",
        Exponential::from_millis(1000),
        10,
        publish_cmd,
    )
}

pub struct RubyConfig {
    pub lang_package_path: PathBuf,
    pub version: String,
    pub registry_url: String,
    pub registry_token: String,
}

pub fn kotlin(config: &KotlinConfig) -> anyhow::Result<()> {
    let mut publish_cmd = Command::new("./gradlew");
    publish_cmd
        .arg("publish")
        .env("KOTLIN_REGISTRY_URL", &config.registry_url)
        .env("KOTLIN_REGISTRY_USERNAME", &config.registry_username)
        .env("KOTLIN_REGISTRY_PASSWORD", &config.registry_password)
        .current_dir(&config.lang_package_path);

    cmd_retry(
        "Kotlin publication",
        Exponential::from_millis(1000),
        10,
        publish_cmd,
    )
}

pub struct KotlinConfig {
    pub lang_package_path: PathBuf,
    pub registry_url: String,
    pub registry_username: String,
    pub registry_password: String,
}

pub fn swift_repo(config: &SwiftRepoConfig) -> anyhow::Result<()> {
    // Get the pointer to the tmp folder generated in the previous release step
    let tmp_package_path =
        Path::new(read_to_string(config.lang_package_path.join("processing_at.txt"))?.as_str())
            .join("Zcash");

    // Publish the artifact to git.
    let mut git_publish_cmd = Command::new("git");
    git_publish_cmd
        .arg("push")
        .arg("--progress")
        .arg(&config.git_repo_url)
        .current_dir(&tmp_package_path);

    cmd_retry(
        "Swift Git push",
        Exponential::from_millis(1000),
        10,
        git_publish_cmd,
    )?;

    // Push the tags to git.
    let mut git_tags_cmd = Command::new("git");
    git_tags_cmd
        .arg("push")
        .arg("--tags")
        .arg(&config.git_repo_url)
        .current_dir(&tmp_package_path);

    cmd_retry(
        "Swift push tags",
        Exponential::from_millis(1000),
        10,
        git_tags_cmd,
    )
}

pub struct SwiftRepoConfig {
    pub lang_package_path: PathBuf,
    pub git_repo_url: String,
}

pub fn swift_registry(config: &SwiftRegistryConfig) -> anyhow::Result<()> {
    // Log-in into swift package registry via token. See https://github.com/apple/swift-package-manager/blob/main/Documentation/PackageRegistryUsage.md#registry-authentication
    cmd_success(
        Command::new("swift")
            .arg("package-registry")
            .arg(&config.registry_url)
            .arg("--token")
            .arg(&config.registry_token)
            .arg("--no-confirm")
            .spawn()?
            .wait(),
    )?;

    // Get the pointer to the tmp folder generated in the previous release step
    let tmp_package_path =
        Path::new(read_to_string(config.lang_package_path.join("processing_at.txt"))?.as_str())
            .join("Zcash");

    // Publish the artifact to swift package registry. See https://github.com/apple/swift-package-manager/blob/main/Documentation/PackageRegistryUsage.md#publishing-to-registry
    let mut publish_cmd = Command::new("swift");
    publish_cmd
        .arg("package-registry")
        .arg("publish")
        .arg(&config.version)
        .arg("--url")
        .arg(&config.registry_url)
        .current_dir(&tmp_package_path);

    cmd_retry(
        "Swift registry publish",
        Exponential::from_millis(1000),
        10,
        publish_cmd,
    )
}

pub struct SwiftRegistryConfig {
    pub lang_package_path: PathBuf,
    pub version: String,
    pub registry_url: String,
    pub registry_token: String,
}

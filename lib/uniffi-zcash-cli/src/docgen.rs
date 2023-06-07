use std::{fs, path::Path, process::Command};

use fs_extra::dir::create_all;

use crate::{helper::cmd_success, setup::dokka_install_dir};

pub fn python(sources_dir: &Path, target_dir: &Path) -> anyhow::Result<()> {
    create_all(target_dir, false)?;

    cmd_success(
        Command::new("pydoc")
            .arg("-w")
            .arg(sources_dir.join("zcash").join("zcash.py"))
            .current_dir(&target_dir)
            .spawn()?
            .wait(),
    )
}

pub fn ruby(sources_dir: &Path, target_dir: &Path) -> anyhow::Result<()> {
    cmd_success(
        Command::new("rdoc")
            .arg(sources_dir)
            .arg("--op")
            .arg(target_dir)
            .spawn()?
            .wait(),
    )
}

pub fn kotlin(sources_dir: &Path, target_dir: &Path) -> anyhow::Result<()> {
    create_all(target_dir, false)?;
    let dokka_install_dir = dokka_install_dir();
    let dokka_cli = dokka_install_dir.join("dokka-cli-1.8.20.jar");

    let plugins = vec![
        "dokka-base-1.8.20.jar",
        "dokka-analysis-1.8.20.jar",
        "kotlin-analysis-intellij-1.8.20.jar",
        "kotlin-analysis-compiler-1.8.20.jar",
        "kotlinx-html-jvm-0.8.0.jar",
        "freemarker-2.3.31.jar",
    ];

    let plugins_arg = plugins
        .iter()
        .fold(Vec::new(), |mut acc, p| {
            acc.push(dokka_install_dir.join(p));
            acc
        })
        .iter()
        .map(|p| p.to_string_lossy())
        .collect::<Vec<_>>()
        .join(";");

    cmd_success(
        Command::new("java")
            .arg("-jar")
            .arg(dokka_cli)
            .arg("-pluginsClasspath")
            .arg(plugins_arg)
            .arg("-sourceSet")
            .arg(format!("-src {}", sources_dir.to_string_lossy()))
            .arg("-outputDir")
            .arg(target_dir)
            .spawn()?
            .wait(),
    )
}

pub fn swift(sources_dir: &Path, target_dir: &Path) -> anyhow::Result<()> {
    create_all(target_dir, false)?;

    let package_build_path = fs::read_to_string(sources_dir.join("processing_at.txt"))?;

    cmd_success(
        Command::new("docker")
            .arg("run")
            .arg("-w")
            .arg("/target")
            .arg("--rm")
            .arg("-v")
            .arg(format!("{}:{}", package_build_path, "/source:Z"))
            .arg("-v")
            .arg(format!("{}:{}", target_dir.to_string_lossy(), "/target:Z"))
            .arg("swiftdoc/swift-doc:1.0.0-rc.1")
            .arg("generate")
            .arg("/source")
            .arg("--module-name")
            .arg("Zcash")
            .arg("--format")
            .arg("html")
            .arg("-o")
            .arg("/target")
            .spawn()?
            .wait(),
    )
}

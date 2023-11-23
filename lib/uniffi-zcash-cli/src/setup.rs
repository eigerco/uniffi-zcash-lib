use crate::helper::{clean_dir, cmd_success, TARGETS};
use anyhow::anyhow;
use std::{fs, path::PathBuf, process::Command};

pub fn add_rust_targets() -> anyhow::Result<()> {
    TARGETS.iter().try_for_each(|arch| {
        cmd_success(
            Command::new("rustup")
                .arg("target")
                .arg("add")
                .arg(arch)
                .spawn()?
                .wait(),
        )
    })
}

pub fn install_zig_build() -> anyhow::Result<()> {
    cmd_success(
        Command::new("pip3")
            .arg("install")
            .arg("ziglang")
            .spawn()?
            .wait(),
    )?;

    cmd_success(
        Command::new("cargo")
            .arg("install")
            .arg("--force")
            .arg("cargo-zigbuild")
            .spawn()?
            .wait(),
    )
}

const MACOS_SDK_VERSION: &str = "11.3";
const MACOS_SDK_SHA256_SUM: &str = "cd4f08a75577145b8f05245a2975f7c81401d75e9535dcffbb879ee1deefcbf4"; // 11.3

pub fn install_macos_sdk() -> anyhow::Result<()> {
    if macos_sdk_require_path().exists() {
        println!(
            "Macos sdk already installed at {}",
            macos_sdk_require_path().to_string_lossy()
        );
        return Ok(());
    }

    let apple_sdk_install_path = macos_sdk_install_path();
    clean_dir(&apple_sdk_install_path)?;
    cmd_success(
        Command::new("wget")
            .arg(format!(
                // the version hardcoded corresponds to the github release tag, not the version
                "https://github.com/phracker/MacOSX-SDKs/releases/download/11.3/MacOSX{}.sdk.tar.xz",
                MACOS_SDK_VERSION
            ))
            .current_dir(&apple_sdk_install_path)
            .spawn()?
            .wait(),
    )?;

    let tar = fs::read(macos_sdk_tar_path())?;
    let hash = sha256::digest(tar.as_slice());

    if hash.ne(MACOS_SDK_SHA256_SUM) {
        return Err(anyhow!(
            "Hashes differ. Expected {} \n Downloaded {}",
            MACOS_SDK_SHA256_SUM,
            hash
        ));
    }

    cmd_success(
        Command::new("tar")
            .arg("-J")
            .arg("-xf")
            .arg(format!("MacOSX{}.sdk.tar.xz", MACOS_SDK_VERSION))
            .current_dir(&apple_sdk_install_path)
            .spawn()?
            .wait(),
    )
}

pub fn macos_sdk_install_path() -> PathBuf {
    home_dir().unwrap().join("MacOSXSDK")
}

pub fn macos_sdk_require_path() -> PathBuf {
    macos_sdk_install_path().join(format!("MacOSX{}.sdk", MACOS_SDK_VERSION))
}

pub fn macos_sdk_tar_path() -> PathBuf {
    macos_sdk_install_path().join(format!("MacOSX{}.sdk.tar.xz", MACOS_SDK_VERSION))
}

pub fn home_dir() -> anyhow::Result<PathBuf> {
    match home::home_dir() {
        Some(path) => Ok(path.join(".zcash-uniffi")),
        None => Err(anyhow!("Cannot calculate home dir !")),
    }
}

pub fn install_dokka_cli() -> anyhow::Result<()> {
    let install_dir = dokka_install_dir();
    if install_dir.exists() {
        println!(
            "Dokka already installed at {}",
            install_dir.to_string_lossy()
        );
        return Ok(());
    }
    clean_dir(&install_dir)?;
    // See https://kotlinlang.org/docs/dokka-cli.html#generate-documentation
    let jars = vec![
        "https://repo1.maven.org/maven2/org/jetbrains/dokka/dokka-cli/1.8.20/dokka-cli-1.8.20.jar",
        "https://repo1.maven.org/maven2/org/jetbrains/dokka/dokka-base/1.8.20/dokka-base-1.8.20.jar",
        "https://repo1.maven.org/maven2/org/jetbrains/dokka/dokka-analysis/1.8.20/dokka-analysis-1.8.20.jar",
        "https://repo1.maven.org/maven2/org/jetbrains/dokka/kotlin-analysis-compiler/1.8.20/kotlin-analysis-compiler-1.8.20.jar",
        "https://repo1.maven.org/maven2/org/jetbrains/dokka/kotlin-analysis-intellij/1.8.20/kotlin-analysis-intellij-1.8.20.jar",
        "https://repo1.maven.org/maven2/org/jetbrains/kotlinx/kotlinx-html-jvm/0.8.0/kotlinx-html-jvm-0.8.0.jar",
        "https://repo1.maven.org/maven2/org/freemarker/freemarker/2.3.31/freemarker-2.3.31.jar"
    ];

    jars.into_iter().try_for_each(|j| {
        cmd_success(
            Command::new("wget")
                .arg(j)
                .current_dir(&install_dir)
                .spawn()?
                .wait(),
        )
    })
}

pub fn dokka_install_dir() -> PathBuf {
    home_dir().unwrap().join("dokka")
}

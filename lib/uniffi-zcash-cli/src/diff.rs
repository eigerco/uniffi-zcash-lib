use clap::ColorChoice;
use public_api::{diff::PublicApiDiff, tokens::Token, PublicApi};
use std::path::Path;
use tempdir::TempDir;

use self::{
    colors::{color_item_with_diff, color_token_stream},
    git::init_libs_repo,
    grep_item::GrepItem,
    utils::{copy_if_not_exists, get_manifest_from_package_name},
};

mod colors;
mod git;
mod grep_item;
mod grep_type;
mod utils;

/// The only public function in this module.
/// Looks for public API difference between two versions of a certain library.
/// Then based on that difference, greps a given codebase for possible usage
/// of a changed item of that librarie's public API.
pub fn generate_diff(
    package_name: &str,
    package_new_version: &str,
    package_old_version: &str,
    grep_dir: &str,
    color: ColorChoice,
) -> anyhow::Result<()> {
    let temp_dir_latest = TempDir::new("librustzcash-latest")?;
    let temp_dir_current = TempDir::new("librustzcash-current")?;

    let libs_repo_current_folder = &temp_dir_latest.path().display().to_string();
    let libs_repo_latest_folder = &temp_dir_current.path().display().to_string();

    let _ = init_libs_repo(package_name, libs_repo_current_folder, package_old_version)?;

    // copy instead of cloning it again
    copy_if_not_exists(
        Path::new(libs_repo_current_folder),
        Path::new(libs_repo_latest_folder),
    )?;

    let _ = init_libs_repo(package_name, libs_repo_latest_folder, package_new_version)?;

    rustup_toolchain::install(public_api::MINIMUM_NIGHTLY_RUST_VERSION)?;

    let current_api = get_public_api(libs_repo_current_folder, &package_name)?;
    let latest_api = get_public_api(libs_repo_latest_folder, &package_name)?;
    let public_api_diff = PublicApiDiff::between(current_api, latest_api);

    for diff in public_api_diff.removed {
        let mut grep_item = GrepItem::from(diff.tokens());
        grep_item.color = color;
        if color == ColorChoice::Never {
            grep_item.api_diff = format!("-{}", diff);
        } else {
            grep_item.api_diff = format!("-{}", color_token_stream(diff.tokens(), None));
        }

        grep_item.grep(grep_dir)?.print_result();
    }

    for diff in public_api_diff.changed {
        let old_tokens: Vec<&Token> = diff.old.tokens().collect();
        let new_tokens: Vec<&Token> = diff.new.tokens().collect();
        let diff_slice = diff::slice(old_tokens.as_slice(), new_tokens.as_slice());

        let mut grep_item = GrepItem::from(diff.old.tokens());
        grep_item.color = color;
        if color == ColorChoice::Never {
            grep_item.api_diff = format!("-{}\n+{}", diff.old, diff.new);
        } else {
            grep_item.api_diff = format!(
                "-{}\n+{}",
                color_item_with_diff(&diff_slice, true),
                color_item_with_diff(&diff_slice, false),
            );
        }

        grep_item.grep(&grep_dir)?.print_result();
    }

    Ok(())
}

// Get an object of the type PublicApi, which contains the public API of a certain crate.
// Crate is specified by the path to it's Cargo.toml file.
fn get_public_api(repo_folder: &str, package_name: &str) -> anyhow::Result<PublicApi> {
    let manifest_path =
        get_manifest_from_package_name(format!("{}/Cargo.toml", repo_folder), package_name)?;
    let json = rustdoc_json::Builder::default()
        .toolchain("nightly")
        .manifest_path(manifest_path)
        .build()?;

    let api = public_api::Builder::from_rustdoc_json(json)
        .omit_blanket_impls(true)
        .omit_auto_trait_impls(true)
        .omit_auto_derived_impls(true)
        .build()?;

    Ok(api)
}

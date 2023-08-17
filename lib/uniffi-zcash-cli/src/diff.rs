use clap::ColorChoice;
use public_api::{diff::PublicApiDiff, tokens::Token, PublicApi};
use std::path::Path;

use self::{
    colors::{color_item_with_diff, color_token_stream},
    git::init_libs_repo,
    grep_item::GrepItem,
    utils::{copy_if_not_exists, get_manifest_from_package_name},
};

const LIBS_REPO_LATEST_FOLDER: &str = "/tmp/librustzcash-diff/latest";
const LIBS_REPO_CURRENT_FOLDER: &str = "/tmp/librustzcash-diff/current";

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
    package_name: String,
    package_new_version: String,
    package_old_version: String,
    grep_dir: String,
    color: ColorChoice,
) -> anyhow::Result<()> {
    let _ = init_libs_repo(
        &package_name,
        String::from(LIBS_REPO_CURRENT_FOLDER),
        package_old_version,
    )?;

    // copy instead of cloning it again
    copy_if_not_exists(
        Path::new(LIBS_REPO_CURRENT_FOLDER),
        Path::new(LIBS_REPO_LATEST_FOLDER),
    )?;

    let _ = init_libs_repo(
        &package_name,
        String::from(LIBS_REPO_LATEST_FOLDER),
        package_new_version,
    )?;

    rustup_toolchain::install(public_api::MINIMUM_NIGHTLY_RUST_VERSION)?;

    let current_api = get_public_api(LIBS_REPO_CURRENT_FOLDER, &package_name)?;
    let latest_api = get_public_api(LIBS_REPO_LATEST_FOLDER, &package_name)?;
    let public_api_diff = PublicApiDiff::between(current_api, latest_api);

    for diff in public_api_diff.removed {
        let mut grep_item = GrepItem::from(diff.tokens());
        grep_item.color = color;
        if color == ColorChoice::Never {
            grep_item.api_diff = format!("-{}", diff);
        } else {
            grep_item.api_diff = format!("-{}", color_token_stream(diff.tokens(), None));
        }

        grep_item.grep(&grep_dir)?.print_result();
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

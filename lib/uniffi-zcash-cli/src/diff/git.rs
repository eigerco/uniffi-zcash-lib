use std::path::Path;

use anyhow::Context;
use git2::Repository;

const LIBS_REPO_URL: &str = "https://github.com/zcash/librustzcash.git";

// NOTE: The tag is constructed in a format used by `librustzcash`, which is "libName-livVersion".
// For example: zcash_primitives-0.10.2
// So if the naming convention is not applied, the code will not be able to checkout the tag.
pub(crate) fn init_libs_repo(
    lib_name: &str,
    repo_path: &Path,
    version: &str,
) -> anyhow::Result<()> {
    let tag = format!("{}-{}", lib_name, version);
    let repo = Repository::open(repo_path.to_owned())
        .or_else(|_| Repository::clone(LIBS_REPO_URL, repo_path))
        .context("failed to clone current repo")?;

    checkout_tag(&repo, &tag)?;

    Ok(())
}

// Chckout a certain tag from a given repository
fn checkout_tag(repo: &git2::Repository, tag: &str) -> anyhow::Result<()> {
    let (object, reference) = repo
        .revparse_ext(tag)
        .with_context(|| format!("git object \"{}\" not found", tag))?;

    repo.checkout_tree(&object, None)
        .with_context(|| format!("failed to checkout tree for \"{}\"", tag))?;

    match reference {
        // gref is an actual reference like branches or tags
        Some(gref) => repo.set_head(gref.name().unwrap()),
        // this is a commit, not a reference
        None => repo.set_head_detached(object.id()),
    }
    .with_context(|| format!("failed to set HEAD for \"{}\"", tag))?;

    Ok(())
}

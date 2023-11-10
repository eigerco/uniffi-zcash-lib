use std::path::Path;
use std::process::Command;
use anyhow::Context;
// use git2::Repository;

const LIBS_REPO_URL: &str = "https://github.com/zcash/librustzcash.git";

// NOTE: The tag is constructed in a format used by `librustzcash`, which is "libName-livVersion".
// For example: zcash_primitives-0.10.2
// So if the naming convention is not applied, the code will not be able to checkout the tag.
pub(crate) fn init_libs_repo(
    lib_name: &str,
    repo_path: &Path,
    version: &str,
) -> anyhow::Result<()> {
    let tag = format!("{lib_name}-{version}");

    // open or clone
    // let repo = Repository::open(repo_path)
    //     .or_else(|_| Repository::clone(LIBS_REPO_URL, repo_path))
    //     .context("failed to clone current repo")?;
    let repo_exists = Command::new("cd").arg(repo_path.join(".git")).spawn().is_ok();
    if(repo_exists) {
        cmd_success(
            Command::new("git")
                .arg("clone")
                .arg(LIBS_REPO_URL)
                .arg(repo_path)
                .spawn()?
                .wait(),
        )?;
    }

    // checkout_tag(&repo, &tag)?;
    let revparse_out = Command::new("git").arg("rev-parse").arg(&tag).spawn().map_err(|_| format!("git object \"{}\" not found", tag))?.output();
    let (object, reference) = parse_args(revparse_out);
    Command::new("git").arg("checkout").arg(&object)

    match reference {
        // gref is an actual reference like branches or tags
        // repo.set_head(gref.name().unwrap()),
        Some(gref) => Command::new("git").arg("remote").arg("set-head").arg(&gref.name().unwrap()).spawn()?.wait(),
        // this is a commit, not a reference
        // repo.set_head_detached(object.id()),                 // does this arg exist???
        None => Command::new("git").arg("remote").arg("set-head").arg("--detached").arg(&object.id()).spawn()?.wait(),
    }

    Ok(())
}

fn parse_args(output: &str) -> (Any, Some(Any)) {

}

// Chckout a certain tag from a given repository
// fn checkout_tag(repo: &git2::Repository, tag: &str) -> anyhow::Result<()> {
//     let (object, reference) = repo
//         .revparse_ext(tag)
//         .with_context(|| format!("git object \"{}\" not found", tag))?;

//     repo.checkout_tree(&object, None)
//         .with_context(|| format!("failed to checkout tree for \"{}\"", tag))?;

//     match reference {
//         // gref is an actual reference like branches or tags
//         Some(gref) => repo.set_head(gref.name().unwrap()),
//         // this is a commit, not a reference
//         None => repo.set_head_detached(object.id()),
//     }
//     .with_context(|| format!("failed to set HEAD for \"{}\"", tag))?;

//     Ok(())
// }

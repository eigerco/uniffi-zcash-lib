use crate::helper::cmd_success;
use std::path::Path;
use std::process::Command;

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

    if Path::new(&repo_path.join(".git")).is_dir() {
        std::env::set_current_dir(repo_path)?;

        cmd_success(
            Command::new("git")
                .arg("fetch")
                .arg("--tags")
                .spawn()?
                .wait(),
        )?;
    } else {
        cmd_success(
            Command::new("git")
                .arg("clone")
                .arg(LIBS_REPO_URL)
                .arg(repo_path)
                .spawn()?
                .wait(),
        )?;

        std::env::set_current_dir(repo_path)?;
    }

    cmd_success(
        Command::new("git")
            .arg("checkout")
            .arg(&tag)
            .spawn()?
            .wait(),
    )?;

    Ok(())
}

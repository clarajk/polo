use crate::cli::Format;
use std::path::{Path, PathBuf};

fn local() -> anyhow::Result<PathBuf> {
    let Some(path) = dirs::home_dir() else {
        anyhow::bail!("could not determine home directory");
    };

    Ok(path.join(".local"))
}

fn find(dir: impl AsRef<Path>) -> anyhow::Result<PathBuf> {
    let path = local()?.join(dir);
    if path.exists() && !path.is_dir() {
        anyhow::bail!("{} is not a directory", path.display());
    }

    if !path.exists() {
        std::fs::create_dir_all(&path)?;
    }

    Ok(path)
}

pub fn bin() -> anyhow::Result<PathBuf> {
    find("bin")
}

pub fn lib(fmt: Format) -> anyhow::Result<PathBuf> {
    Ok(find("lib")?
        .join(env!("CARGO_PKG_NAME"))
        .join(fmt.dirname()))
}

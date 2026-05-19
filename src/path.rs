use std::path::PathBuf;

pub fn bin() -> anyhow::Result<PathBuf> {
    let Some(path) = dirs::home_dir() else {
        anyhow::bail!("could not determine home directory");
    };

    let path = path.join(".local").join("bin");
    if !path.is_dir() {
        std::fs::create_dir_all(&path)?;
    }

    Ok(path)
}

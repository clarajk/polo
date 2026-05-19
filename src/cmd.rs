use crate::cli::NewArgs;
use std::fs::OpenOptions;
use std::io::Write;
use std::os::unix::fs::PermissionsExt;

pub fn new(args: NewArgs) -> anyhow::Result<()> {
    let path = crate::path::bin()?.join(&args.name);
    if path.exists() && !args.force {
        anyhow::bail!("{} already exists", path.display());
    }

    let lib = crate::path::lib(args.format)?;
    if lib.exists() && !lib.is_dir() {
        eprintln!("warning: {} is not a directory", lib.display());
    } else if !lib.exists() {
        std::fs::create_dir_all(&lib)?;
    }

    {
        let mut f = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&path)?;

        writeln!(f, "{}", args.format.shebang())?;

        if let Some(prelude) = args.format.prelude() {
            writeln!(f, "{}", prelude)?;
        }

        if args.lib {
            writeln!(f)?;
            writeln!(f, "{}", args.format.bootstrap())?;
        }

        writeln!(f)?;

        f.sync_all()?;
    }

    let mut perms = path.metadata()?.permissions();
    perms.set_mode(0o755);
    std::fs::set_permissions(&path, perms)?;

    edit::edit_file(&path)?;

    Ok(())
}

pub fn remove(name: String) -> anyhow::Result<()> {
    let path = crate::path::bin()?.join(&name);
    std::fs::remove_file(&path)?;

    Ok(())
}

pub fn edit(name: String) -> anyhow::Result<()> {
    let path = crate::path::bin()?.join(&name);
    edit::edit_file(&path)?;

    Ok(())
}

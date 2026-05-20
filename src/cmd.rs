use crate::cli::NewArgs;
use dialoguer::Confirm;
use dialoguer::theme::ColorfulTheme;
use std::fs::OpenOptions;
use std::io::Write;
use std::os::unix::fs::PermissionsExt;
use std::path::PathBuf;

fn check_name(name: impl AsRef<str>) -> anyhow::Result<()> {
    if !safename::is_file_safe(name.as_ref()) {
        anyhow::bail!("{} is not a valid file name", name.as_ref());
    } else {
        Ok(())
    }
}

pub fn new(args: NewArgs) -> anyhow::Result<()> {
    check_name(&args.name)?;

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

        f.sync_all()?;
    }

    let mut perms = path.metadata()?.permissions();
    perms.set_mode(0o755);
    std::fs::set_permissions(&path, perms)?;

    edit::edit_file(&path)?;

    Ok(())
}

pub fn remove(name: String) -> anyhow::Result<()> {
    check_name(&name)?;

    let path = crate::path::bin()?.join(&name);

    let confirm = Confirm::with_theme(&ColorfulTheme::default())
        .default(false)
        .with_prompt(format!("Remove {}?", path.display()))
        .interact()?;

    if confirm {
        std::fs::remove_file(&path)?;
        println!("Removed {}", path.display());
    }

    Ok(())
}

pub fn edit(name: String) -> anyhow::Result<()> {
    check_name(&name)?;

    let path = crate::path::bin()?.join(&name);
    edit::edit_file(&path)?;

    Ok(())
}

pub fn list() -> anyhow::Result<()> {
    let path = crate::path::bin()?;

    for entry in path.read_dir()? {
        println!("{}", entry?.file_name().to_string_lossy());
    }

    Ok(())
}

pub fn install(src: PathBuf, force: bool) -> anyhow::Result<()> {
    let file_name = src
        .file_name()
        .ok_or(anyhow::anyhow!("unable to detect file name"))?;
    let dest = crate::path::bin()?.join(file_name);

    if dest.is_file() && !force {
        anyhow::bail!("{} already exists", dest.display());
    }

    std::fs::rename(src, &dest)?;

    let mut perms = dest.metadata()?.permissions();
    perms.set_mode(0o755);
    std::fs::set_permissions(&dest, perms)?;

    Ok(())
}

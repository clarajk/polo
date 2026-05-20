mod cli;
mod cmd;
mod path;

use crate::cli::Command;
use clap::Parser;
use nix::unistd::Uid;

#[cfg(not(target_family = "unix"))]
compile_error!("This utility is only meant for unix-like systems. It will not compile on Windows.");

#[cfg(target_family = "unix")]
fn main() -> anyhow::Result<()> {
    if Uid::effective().is_root() {
        anyhow::bail!("This utility should not be run as root.");
    }

    let cmd = Command::parse();

    let path = path::bin()?;
    if !path.is_dir() {
        anyhow::bail!("{} is not a directory", path.display());
    }

    let paths = std::env::var("PATH")?;
    let mut paths = std::env::split_paths(&paths);

    if !paths.any(|p| p == path) {
        eprintln!("Warning: {} is not in your PATH.", path.display());
    }

    if std::env::var("EDITOR").is_err() {
        anyhow::bail!("The EDITOR environment variable is not set.")
    }

    match cmd {
        Command::New(args) => cmd::new(args)?,
        Command::Edit { name } => cmd::edit(name)?,
        Command::Remove { name } => cmd::remove(name)?,
        Command::List => cmd::list()?,
        Command::Install { path, force } => cmd::install(path, force)?,
    }

    Ok(())
}

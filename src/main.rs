mod cli;
mod cmd;

use crate::cli::Command;
use clap::Parser;
use nix::unistd::Uid;

#[cfg(not(target_family = "unix"))]
compile_error!("This utility is only meant for unix-like systems. It will not compile on Windows.");

#[cfg(target_family = "unix")]
fn main() {
    if Uid::effective().is_root() {
        eprintln!(
            "Please do not run this utility as root. It is meant to be used by regular users to manage their own scripts."
        );
        std::process::exit(1);
    }

    let cmd = Command::parse();

    let res = match cmd {
        Command::New(args) => cmd::new(args),
        Command::Edit { name } => cmd::edit(name),
        Command::Remove { name } => cmd::remove(name),
    };

    if let Err(e) = res {
        eprintln!("Error: {e}");
        std::process::exit(1);
    }
}

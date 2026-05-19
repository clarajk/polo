use clap::{Args, Parser, ValueEnum};

#[derive(Debug, Copy, Clone, ValueEnum)]
pub enum Format {
    Sh,
    Bash,
    Zsh,
    Fish,
    #[clap(alias = "nu")]
    NuShell,
    #[clap(alias = "rb")]
    Ruby,
    #[clap(alias = "py")]
    Python,
    Perl,
    Lua,
}

#[derive(Debug, Args)]
pub struct NewArgs {
    /// Which syntax and shebang to use for the newly created script.
    pub format: Format,
    /// The name of the new script.
    pub name: String,
    #[clap(short, long)]
    /// Force overwrite.
    pub force: bool,
}

#[derive(Debug, Parser)]
/// A small utility for managing scripts for the local user.
pub enum Command {
    /// Create new script.
    New(NewArgs),
    /// Edit existing script.
    Edit { name: String },
    #[clap(alias = "rm")]
    /// Remove existing script.
    Remove { name: String },
}

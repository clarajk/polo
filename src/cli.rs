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
    pub format: Format,
    pub name: String,
    #[clap(short, long)]
    pub force: bool,
}

#[derive(Debug, Parser)]
pub enum Command {
    New(NewArgs),
    Edit {
        name: String,
    },
    #[clap(alias = "rm")]
    Remove {
        name: String,
    },
}

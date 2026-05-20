use clap::{Args, Parser, ValueEnum};
use std::path::PathBuf;

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

impl Format {
    pub fn shebang(self) -> &'static str {
        match self {
            Self::Sh => "#!/bin/sh",
            Self::Bash => "#!/usr/bin/env bash",
            Self::Zsh => "#!/usr/bin/env zsh",
            Self::Fish => "#!/usr/bin/env fish",
            Self::NuShell => "#!/usr/bin/env nu",
            Self::Ruby => "#!/usr/bin/env ruby",
            Self::Python => "#!/usr/bin/env python3",
            Self::Perl => "#!/usr/bin/env perl",
            Self::Lua => "#!/usr/bin/env lua",
        }
    }

    pub(crate) fn dirname(&self) -> &'static str {
        match self {
            Format::Sh => "sh",
            Format::Bash => "bash",
            Format::Zsh => "zsh",
            Format::Fish => "fish",
            Format::NuShell => "nu",
            Format::Ruby => "ruby",
            Format::Python => "python",
            Format::Perl => "perl",
            Format::Lua => "lua",
        }
    }

    pub fn prelude(self) -> Option<&'static str> {
        match self {
            Self::Sh => Some("set -eu"),
            Self::Bash | Self::Zsh => Some("set -euo pipefail"),
            Self::Ruby => Some("# frozen_string_literal: true"),
            Self::Perl => Some("\nuse strict;\nuse warnings;"),
            _ => None,
        }
    }

    pub fn bootstrap(self) -> String {
        const LIB: &str = ".local/lib";
        let app = env!("CARGO_PKG_NAME");
        let dir = self.dirname();

        match self {
            Self::Sh => format!(
                r#"require() {{
    . "$HOME/{LIB}/{app}/{dir}/$1"
}}"#
            ),

            Self::Bash | Self::Zsh => format!(
                r#"require() {{
    source "$HOME/{LIB}/{app}/{dir}/$1"
}}"#
            ),

            Self::Fish => format!(
                r#"function require
    source "$HOME/{LIB}/{app}/{dir}/$argv[1]"
end"#
            ),

            Self::NuShell => format!(
                r#"def require [module: string] {{
    use ($env.HOME | path join "{LIB}/{app}/{dir}" $module) *
}}"#
            ),

            Self::Ruby => {
                format!(r#"$LOAD_PATH.unshift(File.join(ENV.fetch("HOME"), "{LIB}/{app}/{dir}"))"#)
            }

            Self::Python => format!(
                r#"import sys
from pathlib import Path

sys.path.insert(0, str(Path.home() / "{LIB}/{app}/{dir}"))"#
            ),

            Self::Perl => format!(r#"use lib "$ENV{{HOME}}/{LIB}/{app}/{dir}";"#),

            Self::Lua => format!(
                r#"local HOME = os.getenv("HOME")

package.path = HOME .. "/{LIB}/{app}/{dir}/?.lua;"
    .. HOME .. "/{LIB}/{app}/{dir}/?/init.lua;"
    .. package.path"#
            ),
        }
    }
}

#[derive(Debug, Args)]
pub struct NewArgs {
    /// Which syntax and shebang to use for the newly created script.
    pub format: Format,
    /// The name of the new script.
    pub name: String,
    #[clap(short, long)]
    /// Force overwrite if the file already exists.
    pub force: bool,
    #[clap(short, long)]
    /// Add external library bootstrap code to the script header.
    /// Enables including files from ~/.local/lib/polo/<format>
    pub lib: bool,
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
    #[clap(alias = "ls")]
    /// List all executable files in ~/.local/bin
    List,
    /// Install a file to ~/.local/bin and make it executable.
    Install {
        /// The path to the file that will be installed.
        path: PathBuf,

        /// Force overwrite if the file already exists.
        force: bool,
    },
}

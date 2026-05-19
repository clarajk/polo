use crate::cli::{Format, NewArgs};
use std::fs::OpenOptions;
use std::io::Write;
use std::os::unix::fs::PermissionsExt;

fn write_header(mut w: impl Write, fmt: Format) -> std::io::Result<()> {
    match fmt {
        Format::Sh => {
            writeln!(w, "#!/bin/sh")?;
        }
        Format::Bash => {
            writeln!(w, "#!/usr/bin/env bash")?;
        }
        Format::Zsh => {
            writeln!(w, "#!/usr/bin/env zsh")?;
        }
        Format::Fish => {
            writeln!(w, "#!/usr/bin/env fish")?;
        }
        Format::NuShell => {
            writeln!(w, "#!/usr/bin/env nu")?;
        }
        Format::Ruby => {
            writeln!(w, "#!/usr/bin/env ruby")?;
            writeln!(w, "# frozen_string_literal: true")?;
        }
        Format::Python => {
            writeln!(w, "#!/usr/bin/env python3")?;
        }
        Format::Perl => {
            writeln!(w, "#!/usr/bin/env perl")?;
        }
        Format::Lua => {
            writeln!(w, "#!/usr/bin/env lua")?;
        }
    }

    Ok(())
}

pub fn new(args: NewArgs) -> anyhow::Result<()> {
    let path = crate::path::bin()?.join(&args.name);
    if path.exists() && !args.force {
        anyhow::bail!("{} already exists", path.display());
    }

    {
        let mut f = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&path)?;

        write_header(&mut f, args.format)?;
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

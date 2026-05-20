# polo
A small utility for managing scripts for the local user.

## Install

Download the latest Linux release from the GitHub Releases page.

### Binary

```sh
curl -L -o polo.tar.gz \
  https://github.com/clarajk/polo/releases/latest/download/polo-x86_64-unknown-linux-musl.tar.gz

tar -xzf polo.tar.gz
chmod +x polo
sudo install -Dm755 polo /usr/local/bin/polo
```

### From source

```
cargo install --git https://github.com/clarajk/polo.git
```

## Usage

```
A small utility for managing scripts for the local user

Usage: polo <COMMAND>

Commands:
  new      Create new script
  edit     Edit existing script
  remove   Remove existing script
  list     List all executable files in ~/.local/bin
  install  Install a file to ~/.local/bin and make it executable
  help     Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help
```

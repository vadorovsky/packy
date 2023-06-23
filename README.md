# unseal

CLI tool for unpacking archives (of the most common types).

It's focused on simplicity such as:

* Auto-detection of archive type.
* Portable binaries for various platforms.
* No external dependencies (other binaries or any dynamic libraries).

## Installation

### Pre-built binary

Download the latest binary from the [releases page](https://github.com/vadorovsky/unseal/releases).

### cargo

```bash
cargo install unseal
```

## Usage

```bash
unseal [OPTIONS] <INPUT>

Arguments:
  <INPUT>  Path to the archive to unseal

Options:
  -o, --output <OUTPUT>
          Directory to unpack the archive into [default: .]
  -s, --strip-components <STRIP_COMPONENTS>
          Strip the specified number of leading components from the archive [default: 0]
  -v, --verbose
          Verbose output
  -h, --help
          Print help
  -V, --version
          Print version
```

## Examples

```bash
# Unpack archive.tar.gz into the current directory.
unseal archive.tar.gz
# Unpack archive.tar.xz into the directory /tmp/foo.
unseal -o /tmp/foo archive.tar.xz
# Unpack archive.tar.bz2 into the directory /tmp/foo, stripping the first component.
unseal -o /tmp/foo -s 1 archive.tar.gz
```

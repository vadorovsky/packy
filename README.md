# packy

CLI tool for creating  unpacking archives (of the most common types).

It's focused on simplicity such as:

* Auto-detection of archive type.
* Portable binaries for various platforms.
* No external dependencies (other binaries or any dynamic libraries).

## But why?

Not gonna lie, the main reasons for this project to start are:

* The frustration coming from usage of `tar`.
  * Incompatibility between GNU tar and BSD tar. Which means: `tar` on Linux
    and macOS differ with archive type support and CLI options. Random failures
    of your shell scripts across different system are annoying, aren't they?
  * Incompatibility across versions - for example, old versions of `tar` don't
    autodetect archive types. So you write you script and then it fails on some
    ancient Ubuntu version. Ain't fun, huh?
  * Runtime dependencies - support of particular archive types depends on
    dynamic libraries.
* The urge to rewrite everything in Rust. ( ͡° ͜ʖ ͡°)

Packy aims to solve that by:

* Being cross-platform and providing super easy, one-liner way to get started
  both on Linux and macOS.
* Supporting as many archive types as possible. Support 100% embedded in packy.
* Static linking, no runtime dependencies, no surprises.
* Being written in Rust!!!111oneone

## Installation

### Pre-built binary

Download the latest binary from the [releases page](https://github.com/vadorovsky/packy/releases).

### cargo

```bash
cargo install packy
```

## Usage

```bash
packy [OPTIONS] <INPUT>

Arguments:
  <INPUT>  Path to the archive to packy

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
packy archive.tar.gz
# Unpack archive.tar.xz into the directory /tmp/foo.
packy -o /tmp/foo archive.tar.xz
# Unpack archive.tar.bz2 into the directory /tmp/foo, stripping the first component.
packy -o /tmp/foo -s 1 archive.tar.gz
```

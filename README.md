# Beyond-the-basics Rust Flake

This repo is a simple example project to accompany [an article] about writing
a [Nix Flake] for a Rust project with some requirements beyond the basics. In
particular it:

* Defines multiple Flake outputs, for different Rust feature selections.
* Defines multiple development environments for Rust stable, nightly, and the
  project's MSRV.
* Supports Rust crate dependencies using `cbindgen` with native libraries.
* Loads the crate TOML metadata in Nix to avoid duplicating project name,
  version and MSRV in more places.

## Example Crate

This repository contains a simple Rust binary project that demonstrates
text-to-speech on Linux. It depends on the [tts-rs] crate, which in turn uses
the [speech-dispatcher] and [speech-dispatcher-sys] crates. The `-sys` crate
uses [pkg-config] and [cbindgen] to generate FFI headers for [speechd] on Linux.
It has an optional `foobar` feature that when enabled, changes the spoken
message.

The [Nix Flake] defines a reproducible environment for:

* Installing the required native code dependencies.
* Ensuring `cbindgen` can locate native code dependencies.
* Installing required Rust toolchains.
* Setting up development environments for each Rust toolchain.
* Outputting two package derivations (with or without the `foobar` feature enabled).

## Usage

### Run without cloning

Existing Nix/NixOS users can run the default package directly without needing to
clone anything or do any additional setup:

```bash
nix run github:cpu/rust-flake
```

### Default Package

After cloning the repo, you can run the default Flake output package directly:

```bash
nix run
```

Or, to run the output package that doesn't enable the "foobar" feature:

```bash
nix run '.#example-base'
```

## Dev Environments

You can quickly enter a development environment for one of the three Rust
versions:

```bash
# Rust nightly (default):
nix develop
# Rust stable:
nix develop '.#stable'
# MSRV:
nix develop '.#msrv'
```

### Cargo

In each development environment you'll have the usual `cargo` tooling and any
extra `devDeps` specified:

```bash
rustc --version && gdb --version && speech-dispatcher --version
cargo fmt && cargo clippy && cargo test
cargo run
cargo run --all-features --release
```

### Quickly running a command

Rather than enter a development shell you can also run a command in the
development environment directly:

```bash
# Nightly:
nix develop '.#nightly' --command cargo test
# Stable:
nix develop '.#stable'  --command cargo test
# MSRV:
nix develop '.#msrv'    --command cargo test
```

## CI

You may also be interested in the [CI configuration]. It uses the Nix Flake to
ensure code is formatted, linted with clippy, and tested with nightly, stable
and MSRV Rust toolchains.

[Nix Flake]: ./flake.nix
[tts-rs]: https://crates.io/crates/tts
[speech-dispatcher]: https://crates.io/crates/speech-dispatcher
[speech-dispatcher-sys]: https://crates.io/crates/speech-dispatcher-sys
[speechd]: https://wiki.archlinux.org/title/Speech_dispatcher
[an article]: https://log.woodweb.ca/articles/rust-flake/
[pkg-config]: https://www.freedesktop.org/wiki/Software/pkg-config/
[cbindgen]: https://github.com/mozilla/cbindgen
[CI configuration]: ./.github/workflows/nix.yml

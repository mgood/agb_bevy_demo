# AGB+Bevy Demo

This is a work-in-progress to bring [bevy_mod_gba](https://github.com/bushrat011899/bevy_mod_gba) back up-to-date with current versions of Bevy, AGB, and other dependencies.

It's building on a patched version of Bevy with fixes for atomics that are not supported on GBA builds.

Since AGB 0.25 has also made some big changes to the API since the 0.21 version `bevy_mod_gba` was built on, I'm starting from a simple app to work my way up rebuilding support for integrating the GBA systems with Bevy incrementally. Right now it has some basic support for creating and rendering sprites, and a test for button handling.

# AGBRS template

## A basic template example for agb projects

This makes getting started with a new project for the Game Boy Advance in rust really simple, by providing
all the boiler plate files for you.

## Building

### Prerequisites

You will need the following installed in order to build and run this project:

- A recent version of `rustup`. See the [rust website](https://www.rust-lang.org/tools/install) for instructions for your operating system

You will also want to install an emulator. The best support in agb is with [mgba](https://mgba.io), with
`println!` support via `agb::println!` but any emulator should work. You'll get the best experience if
`mgba-qt` is in your `PATH`.

If you want to run your game on real hardware, you will also need to install `agb-gbafix` which you can do after installing
rust with the following: `cargo install agb-gbafix`. This is not required if you are only running your game in an emulator.

### Running in an emulator

Once you have the prerequisites installed, you should be able to build using

```sh
cargo build
```

or in release mode (recommended for the final version to ship to players)

```sh
cargo build --release
```

The resulting file will be in `target/thumbv4t-none-eabi/debug/<your game>` or `target/thumbv4t-none-eabi/release/<your game>` depending on
whether you did a release or debug build.

If you have `mgba-qt` in your path, you will be able to run your game with

```sh
cargo run
```

or in release mode

```sh
cargo run --release
```

## Starting development

You can find the documentation for agb [here](https://docs.rs/agb/latest/agb/), or follow the tutorial in [the book](https://agbrs.dev/book/).

You may also want to change the package name and version in `Cargo.toml` before you start.

## Shipping a .gba file for real hardware

To make a game run on real hardware, you will need to convert the built file into a file suitable for
running on the real thing.

First build the binary in release mode using the instructions above, then do the following:

```sh
agb-gbafix target/thumbv4t-none-eabi/release/<your game> -o <your game>.gba
```

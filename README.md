# Overview

This is a simple [Conway's Game of Life](https://en.wikipedia.org/wiki/Conway%27s_Game_of_Life) implementation using [Rust](https://rust-lang.org/) and [Piston](https://www.piston.rs/).

# Controls

Click - Toggle a cell
Right arrow - Progress a generation
R - Clears the board

# Install

You can install a pre-built binary [here](https://github.com/dhogenson/game-of-life/releases/tag/v0.1.0) and install according to your operating system. Once installed unzip it and double click to run.

# Build from source

Use [git](https://git-scm.com/) to download this repository

```bash
git clone https://github.com/dhogenson/game-of-life.git
cd game-of-life
```

If you [Rust](https://rust-lang.org/tools/install/) and a working [linker](https://www.geeksforgeeks.org/cpp/installing-mingw-tools-for-c-c-and-changing-environment-variable/) (i.g MinGW) installed then run this command:

```bash
cargo build --release
```

This will output a binary file in the `target/release` folder, double click the binary to run it.

If you don't want to build it just run it directly:

```bash
cargo run --release
```

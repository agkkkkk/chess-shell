# Chess Shell

## Table of Contents

- [About](#about)
- [Package Used](#package_used)
- [Getting Started](#getting_started)

## About <a name = "about"></a>

A Terminal based chess game built in rust.

## Package Used <a name = "package_used"></a>

- clap => Command line argument Parser
- uci => library to communicate with UCI compatible chess engine
- ratatui => crate to create Terminal User Interface (TUI).

## Getting Started <a name = "getting_started"></a>

### Installing

**With Github**

```
git clone git@github.com:thomas-mauran/chess-tui.git
cd chess-tui
cargo build --release
./target/release/chess-tui
```

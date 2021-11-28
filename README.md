# Advent of Code 2021

## Goals for 2021
- Learning some Rust
- Complete all 25 x 2 puzzles the days released
- Leverage as few possible crates

## Env Setup
- WSL2 Ubuntu 20.04
- VS Code + [Remote extension](https://docs.microsoft.com/en-us/windows/wsl/tutorials/wsl-vscode#install-vs-code-and-the-remote-wsl-extension)
- Rustup install `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
- [Rust extension](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust)
- [CodeLLDB extension](https://marketplace.visualstudio.com/items?itemName=vadimcn.vscode-lldb) for debugging
- Update `rust-client.rustupPath` pathin workspace settings.json to point to your wsl install path (`/home/<user>/.cargo/bin/rustup`)

## New Day
```bash
cargo new aoc-2021-day-#
cd aoc-2021-day-#
code .
```
`F5` then say yes to the Cargo.toml workspace configuration

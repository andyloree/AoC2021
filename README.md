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
1. Create the our daily create:
    ```bash
    cargo new aoc-2021-day-#
    cd aoc-2021-day-#
    code .
    ```
2. Press `F5` then say yes to the Cargo.toml workspace configuration

3. Add standard IO redirection for an `input.txt` file:
    ```yaml
      # before "args": [],
      "stdio": ["input.txt"],
    ```
4. Create an `input.txt` file, and paste in the example or puzzle input
5. Update `src/main.rs` with the template:
    ```rust
    use std::io::{self, BufRead};
    use std::time::{Duration, Instant};

    fn main() {
        let start = Instant::now();
        let stdin = io::stdin();
        let lines: Vec<String> = stdin.lock().lines().flatten().collect();

        println!("Part 1\r\n{}", "-".repeat(10));
        // todo

        println!("Part 2\r\n{}", "-".repeat(10));
        // todo

        let duration = start.elapsed();
        println!("Total execution time: {:?}", duration);
    }
    ```
6. Get to solving

# Tips from along the way...
## Standard IO redirection with Run in vscode
Somewhat difficult to find, but [CodeLLDB allows for standard IO redirection](https://github.com/vadimcn/vscode-lldb/blob/master/MANUAL.md#stdio-redirection) using the `stdio` in the launch.json.
  - stdin only: `"stdio": ["input.txt"]`
  - stdout and stderr: `"stdio": [null, "out.txt", "err.txt"]`

## Perf profiler in WSL2 Ubuntu
[Due to customer kernel within WSL2](https://stackoverflow.com/a/60276918/975654), you have to compile from the accessible kernel source.

1. `sudo apt install build-essential flex bison libssl-dev libelf-dev`
2. `git clone --depth=1 https://github.com/microsoft/WSL2-Linux-Kernel.git`
3. `cd WSL2-Linux-Kernel/tools/perf`
4. `make`
5. `cp perf /usr/bin/perf`
6. Copy the [stackcollapse-perf.pl](https://github.com/brendangregg/FlameGraph/blob/master/stackcollapse.pl) script to `/usr/local/bin/stackcollapse-perf.pl`
7. `chmod 775 /usr/local/bin/stackcollapse-perf.pl`
8. Copy the [rust-unmangle](https://github.com/Yamakaky/rust-unmangle/blob/master/rust-unmangle) sed script to `/usr/local/bin/rust-unmangle`
9. Copy the [flamegraph.pl](https://github.com/brendangregg/FlameGraph/blob/master/flamegraph.pl) script to `/usr/local/bin/flamegraph.pl`
10. `chmod 755 /usr/local/bin/flamegraph.pl`

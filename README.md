# Advent of Code

## Usage

### 2022 - C++

```bash
cmake -S . -B build/ -D CMAKE_BUILD_TYPE=Debug
cmake -S . -B build/ -D CMAKE_BUILD_TYPE=Release
cmake --build build/
build/aoc2022 DAY
build/aoc2022 DAY INPUT
```

### 2023 - Rust/Python

Solved in Python first for leaderboard, `src/py`.

```bash
cargo run --release --bin DAY
cargo run --release --bin DAY < inputs/DAY.in
cargo run --release # run all days
```

Goal was to run all days in under 1 second, currently achieving 2.2s, however
each program seems to have a 55ms startup time, so I guess it's achieved?
2200-55*25=825.

### 2024 - Skip

### 2025

1. LLVM IR
```bash
clear; ./build.sh 1 -no-lib; strace ./target/1 < input/1.in 2> target/1.txt
clear; ./build.sh 2; strace ./target/2 < input/2.in 2> target/2.txt
```

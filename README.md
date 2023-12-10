# Advent of Code


Structure of project from [AxlLind](https://github.com/AxlLind/AdventOfCode2023)

## Usage

### 2022 - C++

```Shell
cmake -S . -B build/ -D CMAKE_BUILD_TYPE=Debug
cmake -S . -B build/ -D CMAKE_BUILD_TYPE=Release
cmake --build build/
build/aoc2022 DAY
build/aoc2022 DAY INPUT
```


### 2023 - Rust

```shell
cargo run --release --bin DAY
cargo run --release --bin DAY < inputs/DAY.in
cargo run --release # run all days
```
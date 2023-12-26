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

Goal was to run all days in under 1 second, currently achieving 2.2s, however
each program seems to have a 55ms startup time, so I guess it's achieved?
2200-55*25=825.

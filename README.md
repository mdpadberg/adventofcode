# Advent of code
Advent of Code is an Advent calendar of small programming puzzles for a variety of skill sets and skill levels that can be solved in any programming language you like. People use them as interview prep, company training, university coursework, practice problems, a speed contest, or to challenge each other.

You don't need a computer science background to participate - just a little programming knowledge and some problem solving skills will get you pretty far. Nor do you need a fancy computer; every problem has a solution that completes in at most 15 seconds on ten-year-old hardware.

If you'd like to support the creator of Advent of Code, please visit this link -> https://adventofcode.com/2022/about

# Table of Contents
- [Utility tools](#utility-tools)
- [2022](#year-2022)
- [2021](#year-2021)
- [2020](#year-2020)

# Utility tools
## Downloader
To see all possibilities:
```
cargo run --package downloader -- -h
```

To download the current day:
```
cargo run --package downloader -- --cookie <YOUR_COOKIE>
```

To download a specific day:
```
cargo run --package downloader -- --year 2022 --day 1 --cookie <YOUR_COOKIE>
```

## Benchtest
To see all possibilities
```
cargo run --package benchtest -- -h
```

To download the current day:
```
cargo run --package downloader --
```

To download a specific day:
```
cargo run --package downloader -- --year 2022 --day 1 --language rust --amount-of-runs 100 --operation-system ubuntu
```

# Year 2022
How to run one day:
```
cargo run --package aoc2022 --bin day1a --release
cargo run --package aoc2022 --bin day1b --release
```

# Year 2021
How to run one day:
```
cargo run --package aoc2021 --bin day1a --release
cargo run --package aoc2021 --bin day1b --release
```

# Year 2020
How to run one day:
```
node 2020/src/bin/day1a.js
node 2020/src/bin/day1b.js
```
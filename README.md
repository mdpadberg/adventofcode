# Advent of code
Advent of Code is an Advent calendar of small programming puzzles for a variety of skill sets and skill levels that can be solved in any programming language you like. People use them as interview prep, company training, university coursework, practice problems, a speed contest, or to challenge each other.

You don't need a computer science background to participate - just a little programming knowledge and some problem solving skills will get you pretty far. Nor do you need a fancy computer; every problem has a solution that completes in at most 15 seconds on ten-year-old hardware.

If you'd like to support the creator of Advent of Code, please visit this link -> https://adventofcode.com/2022/about

# Table of Contents
- [Utility tools](#utility-tools)
- [2020](#year-2020)
- [2021](#year-2021)
- [2022](#year-2022)

# Utility tools
## Download data
To see all possibilities:
```
cargo run --package download-assignment-and-data -- -h
```

To download the current day:
```
cargo run --package download-assignment-and-data -- --cookie <YOUR_COOKIE>
```

To download a specific day:
```
cargo run --package download-assignment-and-data -- --year 2022 --day 01 --cookie <YOUR_COOKIE>
```

# Year 2020
How to run one day:
```
node 2020/src/day01-01.js
node 2020/src/day01-02.js
```

# Year 2021
How to run one day:
```
cargo run --package aoc2021 --bin day01 --release
```

# Year 2022
How to run one day:
```
cargo run --package aoc2022 --bin day01a --release
cargo run --package aoc2022 --bin day01b --release
```
# Advent of code
Advent of Code is an Advent calendar of small programming puzzles for a variety of skill sets and skill levels that can be solved in any programming language you like. People use them as interview prep, company training, university coursework, practice problems, a speed contest, or to challenge each other.

You don't need a computer science background to participate - just a little programming knowledge and some problem solving skills will get you pretty far. Nor do you need a fancy computer; every problem has a solution that completes in at most 15 seconds on ten-year-old hardware.

If you'd like to support the creator of Advent of Code, please visit this link -> https://adventofcode.com/2022/about

# Git
How to clone:
```bash
git clone --recurse-submodules <SSH URL>
```

Update submodule:
```bash
git submodule update --remote --merge
```

Push:
```bash
git push --recurse-submodules=on-demand
```

# Table of Contents
- [Utility tools](#utility-tools)
- [2024](#year-2024)
- [2023](#year-2023)
- [2022](#year-2022)
- [2021](#year-2021)
- [2020](#year-2020)

# Utility tools
## Downloader
To see all possibilities:
```console
% cargo run --package downloader -- -h
Usage: downloader [OPTIONS] --cookie <COOKIE> <COMMAND>

Commands:
  current-day   Download current day
  specific-day  Download specific day [example: 2024 10]
  whole-year    Download the whole year [example: 2024]
  help          Print this message or the help of the given subcommand(s)

Options:
  -c, --cookie <COOKIE>
          Your cookie, login with the browser and find your cookie with the f12 developer tools
  -w, --what-to-download <WHAT_TO_DOWNLOAD>
          What to download [default: input-and-test-data] [possible values: input-only, assignment-only, test-data-only, input-and-test-data, all]
  -h, --help
          Print help
  -V, --version
          Print version
```

To download the current day:
```
cargo run --package downloader -- --cookie <YOUR_COOKIE> current-day
```

To download a specific day:
```
cargo run --package downloader -- --cookie <YOUR_COOKIE> specific-day 2024 10
```

To download all assignments:
```
cargo run --package downloader -- --cookie <YOUR_COOKIE> whole-year --what-to-download assignment-only
```

## Benchtest
To see all possibilities
```console
% cargo run --package benchtest -- -h
Options:
  -l, --language <LANGUAGE>
          Supported languages [possible values: javascript, rust]
  -a, --amount-of-runs <AMOUNT_OF_RUNS>
          Amount of runs [default: 100]
  -y, --year <YEAR>
          Year on which you want to run the benchmark tool on [default: 2024]
  -d, --day <DAY>
          Day on which you want to run the benchmark tool on [default: 4]
  -h, --help
          Print help information
  -V, --version
          Print version information
```

To run a benchmark for the current day:
```
cargo run --package benchtest -- --language rust
```

To run a benchmark for a specific day:
```
cargo run --package benchtest -- --language rust --year 2023 --day 9
```

# Year 2024
How to run one day:
```
cargo run --package aoc2024 --bin day1a --release
cargo run --package aoc2024 --bin day1b --release
```

# Year 2023
How to run one day:
```
cargo run --package aoc2023 --bin day1a --release
cargo run --package aoc2023 --bin day1b --release
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

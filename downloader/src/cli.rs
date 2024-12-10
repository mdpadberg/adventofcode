use crate::download::download;
use anyhow::Result;
use clap::{arg, command, Parser, Subcommand, ValueEnum};

/// Download the data for advent of code
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Your cookie, login with the browser and find your cookie with the f12 developer tools
    #[arg(short, long)]
    cookie: String,

    /// What to download
    #[arg(short, long, value_enum, default_value_t = Possibilities::InputAndTestData)]
    what_to_download: Possibilities,

    /// For which period it should download
    #[clap(subcommand)]
    period: Period
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
#[allow(dead_code)]
pub enum Possibilities {
    InputOnly,
    AssignmentOnly,
    TestDataOnly,
    InputAndTestData,
    All,
}

#[derive(Clone, Debug, Subcommand)]
pub enum Period {
    /// Download current day
    CurrentDay,
    /// Download specific day [example: 2024 10]
    SpecificDay { year: i32, day: u32},
    /// Download the whole year [example: 2024]
    WholeYear { year: i32}
}

pub fn parse() -> Result<()> {
    let args = Args::parse();
    download(args.cookie, args.period, args.what_to_download)
}

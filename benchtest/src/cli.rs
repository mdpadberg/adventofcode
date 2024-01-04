use anyhow::Result;
use chrono::Datelike;
use clap::{arg, command, Parser, ValueEnum};

use crate::runner;

/// Benchtest for advent of code
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Supported languages
    #[arg(short, long, value_enum)]
    pub language: Language,

    /// Amount of runs
    #[arg(short, long, default_value_t = 100)]
    pub amount_of_runs: u32,

    /// Year on which you want to run the benchmark tool on
    #[arg(short, long, default_value_t = chrono::Utc::now().year())]
    pub year: i32,

    /// Day on which you want to run the benchmark tool on
    #[arg(short, long, default_value_t = chrono::Utc::now().day())]
    pub day: u32,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum Language {
    Javascript,
    Rust,
}

pub fn parse() -> Result<()> {
    let args: Args = Args::parse();
    runner::run(args)
}

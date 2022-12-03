use crate::download::download;
use anyhow::Result;
use chrono::Datelike;
use clap::{arg, command, Parser, ValueEnum};

/// Download the data for advent of code
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Your cookie, login with the browser and find your cookie with the f12 developer tools
    #[arg(short, long)]
    cookie: String,

    /// Year of data you want to download
    #[arg(short, long, default_value_t = chrono::Utc::now().year())]
    year: i32,

    /// Day of data you want to download
    #[arg(short, long, default_value_t = chrono::Utc::now().day())]
    day: u32,

    /// What to download
    #[arg(short, long, value_enum, default_value_t = Possibilities::InputOnly)]
    what_to_download: Possibilities,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
#[allow(dead_code)]
pub enum Possibilities {
    InputOnly,
    AssignmentOnly,
    Both,
}

pub fn parse() -> Result<()> {
    let args = Args::parse();
    download(args.cookie, args.year, args.day, args.what_to_download)
}

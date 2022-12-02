use anyhow::{Context, Result};
use chrono::Datelike;
use clap::{arg, command, Parser};
use reqwest::header;
use std::fs;

/// Download the data for advent of code
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Cookie, you can find this one after logging in and going with f12 to developer options
    #[arg(short, long)]
    cookie: String,

    /// Year of data you want to download
    #[arg(short, long, default_value_t = chrono::Utc::now().year())]
    year: i32,

    /// Day of data you want to download
    #[arg(short, long, default_value_t = chrono::Utc::now().day())]
    day: u32,

    /// Remove last new line
    #[arg(short, long, default_value_t = true)]
    remove_last_new_line: bool,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let year = args.year;
    let day = args.day;
    let url = format!("https://adventofcode.com/{year}/day/{day}/input");
    let response = if args.remove_last_new_line {
        if_file_contains_new_lines_remove_last_new_line(call_url_with_headers(
            url,
            create_cookie_header(&args.cookie)?,
        )?)
    } else {
        call_url_with_headers(url, create_cookie_header(&args.cookie)?)?
    };
    let folder = format!("{year}/data/");
    let file = format!("{year}/data/{day}.txt");
    fs::create_dir_all(&folder).context("cannot create this folder")?;
    fs::write(&file, response).context("cannot write to file")?;
    Ok(())
}

fn call_url_with_headers(url: String, headers: header::HeaderMap) -> Result<String> {
    let response = reqwest::blocking::Client::builder()
        .default_headers(headers)
        .build()?
        .get(url)
        .send()?
        .text_with_charset("utf-8")?;
    Ok(response)
}

fn create_cookie_header(cookie: &String) -> Result<header::HeaderMap> {
    let mut headers = header::HeaderMap::new();
    let mut auth_value = header::HeaderValue::from_str(&format!("session={}", cookie))?;
    auth_value.set_sensitive(true);
    headers.insert("Cookie", auth_value);
    Ok(headers)
}

fn if_file_contains_new_lines_remove_last_new_line(input: String) -> String {
    if input.contains("\n") && input.len() > 2 {
        input[0..input.len() - 1].to_string()
    } else {
        input
    }
}

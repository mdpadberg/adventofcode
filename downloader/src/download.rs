use crate::{
    cli::{Period, Possibilities},
    http::{call_url_with_headers, create_cookie_header},
};
use anyhow::{bail, Context, Result};
use chrono::Datelike;
use scraper::{Html, Selector};

pub fn download(cookie: String, period: Period, what_to_download: Possibilities) -> Result<()> {
    let (year, days) = match period {
        Period::CurrentDay => {
            let now = chrono::Utc::now();
            (now.year(), vec![now.day()])
        }
        Period::SpecificDay { year, day } => (year, vec![day]),
        Period::WholeYear { year } => (year, (1..=25).collect()),
    };
    for day in days {
        match what_to_download {
            Possibilities::InputOnly => download_input_data(&cookie, &year, &day)?,
            Possibilities::AssignmentOnly => download_assignment(&cookie, &year, &day)?,
            Possibilities::TestDataOnly => download_all_pre_tags(&cookie, &year, &day)?,
            Possibilities::InputAndTestData => {
                download_input_data(&cookie, &year, &day)?;
                download_all_pre_tags(&cookie, &year, &day)?
            }
            Possibilities::All => {
                download_input_data(&cookie, &year, &day)?;
                download_all_pre_tags(&cookie, &year, &day)?;
                download_assignment(&cookie, &year, &day)?
            }
        }
    }
    Ok(())
}

fn download_input_data(cookie: &String, year: &i32, day: &u32) -> Result<()> {
    let url = format!("https://adventofcode.com/{}/day/{}/input", year, day);
    let folder = format!("adventofcode-private/data/{}", year);
    let file = format!("adventofcode-private/data/{}/{}.txt", year, day);
    let response =
        call_url_with_headers(url, create_cookie_header(&cookie)?)?.text_with_charset("utf-8")?;
    let stripped_last_new_line = if response.contains("\n") && response.len() > 2 {
        response[0..response.len() - 1].to_string()
    } else {
        response
    };
    write_to_file(&folder, &file, stripped_last_new_line)
}

fn download_assignment(cookie: &String, year: &i32, day: &u32) -> Result<()> {
    let url = format!("https://adventofcode.com/{}/day/{}", year, day);
    let folder = format!("adventofcode-private/assignments/{}", year);
    let file = format!("adventofcode-private/assignments/{}/{}.html", year, day);
    let response =
        call_url_with_headers(url, create_cookie_header(&cookie)?)?.text_with_charset("utf-8")?;
    let document = Html::parse_fragment(&response);
    if let Ok(selector) = Selector::parse("article") {
        let html_main = document
            .select(&selector)
            .into_iter()
            .map(|value| value.inner_html())
            .collect::<String>();
        write_to_file(&folder, &file, html_main)
    } else {
        bail!("problem with parsing html of the assignment")
    }
}

fn download_all_pre_tags(cookie: &String, year: &i32, day: &u32) -> Result<()> {
    let url = format!("https://adventofcode.com/{}/day/{}", year, day);
    let folder = format!("adventofcode-private/test-data/{}", year);
    let response =
        call_url_with_headers(url, create_cookie_header(&cookie)?)?.text_with_charset("utf-8")?;
    let document = Html::parse_fragment(&response);
    if let Ok(selector) = Selector::parse("article > pre") {
        Ok(document
            .select(&selector)
            .into_iter()
            .map(|value| {
                let mut result = value.text().collect::<String>();
                if &result[result.len()-1..] == "\n" {
                    result.pop();
                }
                result
            })
            .enumerate()
            .map(|(count, value)| {
                write_to_file(
                    &folder,
                    &format!(
                        "adventofcode-private/test-data/{}/{}-{}.txt",
                        year, day, count
                    ),
                    value,
                )
            })
            .collect::<Result<(), anyhow::Error>>()?)
    } else {
        bail!("problem with parsing html of the assignment")
    }
}

fn write_to_file(folder: &String, file: &String, data: String) -> Result<()> {
    std::fs::create_dir_all(folder).context("cannot create this folder")?;
    std::fs::write(&file, data).context("cannot write to file")?;
    Ok(())
}

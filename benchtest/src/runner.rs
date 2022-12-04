use crate::cli::Args;
use crate::cli::Language;
use crate::cli::Os;
use anyhow::Context;
use anyhow::Result;
use std::process::Command;

pub fn run(args: Args) -> Result<()> {
    let year = args.year;
    let first_part = format!("day{}a", args.day);
    let second_part = format!("day{}b", args.day);
    let output_folder = format!("{}/performance/", args.year);
    match (args.operation_system, args.language) {
        (Os::ubuntu, Language::javascript) => {
            let result_first_part = linux_time_command(
                args.amount_of_runs,
                &format!("benchtest/src/js.sh"),
                &format!("{year}/src/bin/{first_part}.js"),
            )?;
            let result_second_part = linux_time_command(
                args.amount_of_runs,
                &format!("benchtest/src/js.sh"),
                &format!("{year}/src/bin/{second_part}.js"),
            )?;
            write_to_file(
                &output_folder,
                &format!("{}{}.txt", &output_folder, &first_part),
                result_first_part,
            )?;
            write_to_file(
                &output_folder,
                &format!("{}{}.txt", &output_folder, &second_part),
                result_second_part,
            )?;
            Ok(())
        }
        (Os::ubuntu, Language::rust) => {
            cargo_build(year, &first_part)?;
            cargo_build(year, &second_part)?;
            let result_first_part = linux_time_command(
                args.amount_of_runs,
                &format!("benchtest/src/rust.sh"),
                &format!("target/release/{first_part}"),
            )?;
            let result_second_part = linux_time_command(
                args.amount_of_runs,
                &format!("benchtest/src/rust.sh"),
                &format!("target/release/{second_part}"),
            )?;
            write_to_file(
                &output_folder,
                &format!("{}{}.txt", &output_folder, &first_part),
                result_first_part,
            )?;
            write_to_file(
                &output_folder,
                &format!("{}{}.txt", &output_folder, &second_part),
                result_second_part,
            )?;
            Ok(())
        }
        // TODO does time -l work?
        (Os::mac, Language::javascript) => todo!(),
        // TODO does time -l work? && needs to build first then call from target folder
        (Os::mac, Language::rust) => todo!(),
    }
}

/// Time writes by default to STDERR, but you can also write it to a file
/// See man page for more info: https://man7.org/linux/man-pages/man1/time.1.html
fn linux_time_command(
    amount_of_runs: u32,
    shell_file: &str,
    path_to_file_to_benchmark: &str,
) -> Result<String> {
    let pre_text = format!("These are the result of {amount_of_runs} test runs: \n");
    let result = String::from_utf8(
        Command::new("/usr/bin/time")
            .args([
                "-v",
                shell_file,
                &format!("{}", amount_of_runs),
                path_to_file_to_benchmark,
            ])
            .output()?
            .stderr,
    )?;
    Ok(format!("{}{}", pre_text, result))
}

fn write_to_file(folder: &String, file: &String, data: String) -> Result<()> {
    std::fs::create_dir_all(folder).context("cannot create this folder")?;
    std::fs::write(&file, data).context("cannot write to file")?;
    Ok(())
}

fn cargo_build(year: i32, bin_name: &str) -> Result<()> {
    Command::new("cargo")
        .args([
            "build",
            "--package",
            &format!("aoc{year}"),
            "--bin",
            bin_name,
            "--release",
        ])
        .output()?;
    Ok(())
}

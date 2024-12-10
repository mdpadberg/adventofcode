use crate::cli::Args;
use crate::cli::Language;
use anyhow::bail;
use anyhow::Context;
use anyhow::Result;
use std::path::Path;
use std::process::Command;
use std::time::Instant;

pub fn run(args: Args) -> Result<()> {
    let year = args.year;
    let amount_of_runs = args.amount_of_runs.to_string();
    run_part_of_day(
        year,
        &amount_of_runs,
        args.language,
        format!("day{}a", args.day),
    )?;
    run_part_of_day(
        year,
        &amount_of_runs,
        args.language,
        format!("day{}b", args.day),
    )?;
    Ok(())
}

fn run_part_of_day(
    year: i32,
    amount_of_runs: &String,
    language: Language,
    filename: String,
) -> Result<()> {
    let output_folder = format!("{}/performance/", year);
    let project_root = format!("{}", env!("CARGO_MANIFEST_DIR").replace("/benchtest", ""));
    let (path_to_assignment, path_to_script) = match language {
        Language::Javascript => {
            if Command::new("which")
                .arg("node")
                .output()?
                .stdout
                .is_empty()
            {
                bail!("Could not find node, please install node");
            }
            let path_to_script = format!("{project_root}/benchtest/src/js.sh");
            let path_to_assignment = format!("{project_root}/{year}/src/bin/{filename}.js",);
            (path_to_assignment, path_to_script)
        }
        Language::Rust => {
            if Command::new("which")
                .arg("cargo")
                .output()?
                .stdout
                .is_empty()
            {
                bail!("Could not find cargo, please install rust and cargo via rustup");
            }
            cargo_build(year, &filename)?;
            let path_to_script = format!("{project_root}/benchtest/src/rust.sh");
            let path_to_assignment = format!("{project_root}/target/release/{filename}");
            (path_to_assignment, path_to_script)
        }
    };
    run_code_and_measure_time(
        path_to_assignment,
        path_to_script,
        filename,
        amount_of_runs,
        output_folder,
    )?;
    Ok(())
}

fn run_code_and_measure_time(
    path_to_assignment: String,
    path_to_script: String,
    filename: String,
    amount_of_runs: &String,
    output_folder: String,
) -> Result<()> {
    if Path::new(&path_to_assignment).exists() {
        let now = Instant::now();
        // Code block to measure.
        {
            let result = Command::new(path_to_script)
                .args([&amount_of_runs, &path_to_assignment])
                .output()?;
            if !result.stderr.is_empty() {
                bail!(
                    "File: '{path_to_assignment}' contains errors in stderr while running. Stderr: {:?}", 
                    String::from_utf8(result.stderr)
                );
            }
        }
        let elapsed = now.elapsed();
        let result = format!("{:.2?}", elapsed);
        write_to_file(
            &output_folder,
            &format!("{}{}.txt", &output_folder, &filename),
            result,
            &amount_of_runs,
        )?;
    }
    Ok(())
}

fn write_to_file(
    folder: &String,
    file: &String,
    data: String,
    amount_of_runs: &String,
) -> Result<()> {
    let pre_text = format!("{amount_of_runs} run(s) took: {}", data);
    std::fs::create_dir_all(folder).context("cannot create this folder")?;
    std::fs::write(&file, pre_text).context("cannot write to file")?;
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

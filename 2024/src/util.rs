use std::path::PathBuf;

pub fn read_data_for_day(filename: &str) -> Option<String> {
    read_data(&filename, "data")
}

pub fn read_test_data_for_day(filename: &str) -> Option<String> {
    read_data(&filename, "test-data")
}

fn read_data(filename: &str, folder: &str) -> Option<String> {
    let path: PathBuf = [
        env!("CARGO_MANIFEST_DIR").replace(
            "/adventofcode",
            &format!("/adventofcode/adventofcode-private/{folder}"),
        ),
        format!("{}.txt", filename),
    ]
    .iter()
    .collect();
    match std::fs::read_to_string(&path) {
        Ok(ok) => Some(ok),
        Err(err) => {
            println!("could not read file {:#?} {:#?}", &path, err);
            None
        }
    }
}

use std::path::PathBuf;

pub fn read_data_for_day(day: u32) -> Option<String> {
    let path: PathBuf = [
        env!("CARGO_MANIFEST_DIR"),
        "data",
        &format!("{}.txt", &day.to_string())
    ].iter().collect();
    match std::fs::read_to_string(&path) {
        Ok(ok) => Some(ok),
        Err(err) => {
            println!("could not read file {:#?} {:#?}", &path, err);
            None
        }
    }
}

pub fn read_test_data_for_day(filename: &str) -> Option<String> {
    read_test_data(filename.to_string())
}

pub fn read_test_data(filename: String) -> Option<String> {
    let path: PathBuf = [
        env!("CARGO_MANIFEST_DIR"),
        "data-test",
        &format!("{}", filename)
    ].iter().collect();
    match std::fs::read_to_string(&path) {
        Ok(ok) => Some(ok),
        Err(err) => {
            println!("could not read file {:#?} {:#?}", &path, err);
            None
        }
    }
}
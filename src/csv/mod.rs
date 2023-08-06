use std::io::BufRead;
use std::path::PathBuf;

pub mod pt;

pub fn read_csv<T>(filepath: PathBuf, process: fn(Vec<&str>) -> T) -> Vec<T> {
    let mut result = Vec::new();

    let file = std::fs::File::open(filepath).unwrap();
    let reader = std::io::BufReader::new(file);
    for line in reader.lines().map(|v| v.unwrap()) {
        let trimmed = line.trim();
        if trimmed.starts_with("#") {
            continue;
        }

        let fields = trimmed
            .split(',')
            .map(|v| v.trim())
            .filter(|v| !v.is_empty())
            .collect::<Vec<&str>>();
        result.push(process(fields));
    }

    result
}
use glob::glob;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};

const CODEOWNERS_PATH: &str = ".github/CODEOWNERS";
const DIFF_MARKER: &str = "diff --git ";

fn normalize_dir(original: &str, current_dir: &str) -> String {
    let normalized_path = match original.strip_prefix("/") {
        Some(fixed) => fixed,
        None => original,
    };

    current_dir.to_owned() + normalized_path
}

fn ignore_line(line: &str) -> bool {
    line.starts_with("#") || line.is_empty()
}

fn parse_line(line: &str) -> (String, String) {
    let mut parsed: Vec<&str> = line.split(' ').collect();
    let dir = parsed.remove(0).to_string();
    let owners = parsed.join(", ");

    (dir, owners)
}

fn unwrap_path(full_dir: &str, current_dir: &str, owners: &str) -> Vec<(String, String)> {
    glob(&full_dir)
        .expect("Failed to read glob pattern")
        .filter_map(Result::ok)
        .map(|path| path.display().to_string().replace(current_dir, ""))
        .map(|path| (path, owners.to_string()))
        .collect()
}

fn parse_diff_line(line: &str) -> (String, String) {
    let changes = line.strip_prefix(DIFF_MARKER).unwrap();
    let mut files: Vec<&str> = changes.split(" ").collect();
    let from = files.remove(0).strip_prefix("a/").unwrap();
    let to = files.remove(0).strip_prefix("b/").unwrap();

    (from.to_string(), to.to_string())
}

fn main() {
    let path = env::current_dir().unwrap();
    let current_dir = path.display().to_string() + "/";

    let file_path = current_dir.to_owned() + CODEOWNERS_PATH;
    let file = match File::open(file_path) {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };

    let ownership: Vec<(String, String)> = io::BufReader::new(file)
        .lines()
        .filter_map(Result::ok)
        .filter(|line| !ignore_line(&line))
        .map(|line| parse_line(&line))
        .map(|(dir, owners)| (normalize_dir(&dir, &current_dir), owners))
        .flat_map(|(full_dir, owners)| unwrap_path(&full_dir, &current_dir, &owners))
        .collect();

    let changed_files = io::stdin()
        .lines()
        .filter_map(Result::ok)
        .filter(|line| line.starts_with(DIFF_MARKER));

    for line in changed_files {
        let (from, to) = parse_diff_line(&line);

        for (path, owners) in &ownership {
            if from.contains(path) {
                println!("{}\t{}", from, owners);
            }

            if from != to && to.contains(path) {
                println!("{}\t{}", to, owners);
            }
        }
    }
}

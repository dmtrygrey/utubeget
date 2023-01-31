use std::fs::File;
use std::io::Read;

pub fn read_urls(filename: &str) -> Option<Vec<String>> {
    let mut lines: Vec<String> = Vec::new();
    let mut file = File::open(filename).unwrap_or_else(|_| panic!("No {filename} was found!"));
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    for line in contents.lines() {
        lines.push(line.to_string());
    }
    Some(lines)
}

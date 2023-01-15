use std::fs::File;
use std::io::Read;

const FILE_NAME: &str = "urls.txt";

pub fn read_urls() -> Option<Vec<String>> {
    let mut lines: Vec<String> = Vec::new(); 
    let mut file = File::open( FILE_NAME )
                   .unwrap_or_else(|_| panic!("No {FILE_NAME} was found!"));
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    for line in contents.lines() {
        lines.push(line.to_string());
    }
    Some(lines)
}

use std::process::Command;
use std::fs::File;
use std::io::Read;
use anyhow::{anyhow, bail, Result};
use log;

pub fn create_directory(name: &str) -> Result<()> {
    log::info!("Creating directory: {}", &name);
    match std::fs::create_dir(name) {
        Ok(_) => Ok(()),
        Err(error) => match error.kind() {
            std::io::ErrorKind::AlreadyExists => {
                log::debug!("Directory {} already axists", &name);
                Ok(())
            },
            _ => {
                let error = String::from(format!("Error during directory creation: {}", &name));
                log::error!("{}", &error);
                bail!(error);
            },
        },
    }
}

pub fn is_exists(path: &str) -> Result<()> {
    if !std::path::Path::new(&path).exists() {
        let warn = String::from(format!("File/Path doesn't exist: {}", &path));
        log::warn!("{}", &warn);
        Err(anyhow!(warn))
    } else {
        Ok(())
    }
}

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

pub fn find_youtube_downloader() -> Option<String> {
    let whereis = Command::new("which")
        .arg("yt-dlp")
        .output()
        .expect("Error: whereis call failed");
    
    match whereis.status.success() {
        true => {
            let output = String::from_utf8(whereis.stdout).unwrap();
            log::debug!("yt-dlp location is: {}", &output);
            Some(output)
        },
        false => {
            log::error!("yt-dlp wasn't found in file system, you can install it with: $ pip install yt-dlp");
            None
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Ok;
    use rand::{thread_rng, Rng};
    use rand::distributions::Alphanumeric;

    #[test]
    fn test_whereis() {
        match find_youtube_downloader() {
            Some(output) => assert!(output.contains("yt-dlp")),
            None => assert!(false, "Couldn't find yt-dlp"),
        }
    }

    #[test]
    fn test_dir_creating() -> Result<()> {
        let rand_dir: String = thread_rng()
            .sample_iter(&Alphanumeric)
            .take(5)
            .map(char::from)
            .collect();
        let dir_name: String = format!("/tmp/test{}", rand_dir);
        create_directory(&dir_name).unwrap();
        assert_eq!(is_exists(&dir_name).unwrap(), ());
        Ok(())
    }
}
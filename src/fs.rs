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
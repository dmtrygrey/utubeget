use anyhow::{bail, Result};
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
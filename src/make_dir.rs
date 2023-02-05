use anyhow::{bail, Result};

pub fn create_directory(name: &str) -> Result<()> {
    match std::fs::create_dir(name) {
        Ok(_) => Ok(()),
        Err(error) => match error.kind() {
            std::io::ErrorKind::AlreadyExists => {
                println!("Directory {} Already Exists!", &name);
                Ok(())
            }
            _ => {
                println!("Error during directory creation: {}", &name);
                bail!("")
            }
        },
    }
}

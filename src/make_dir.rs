pub fn create_directory(name: &str) -> std::io::Result<()> {
    std::fs::create_dir(name)?;
    Ok(())
}

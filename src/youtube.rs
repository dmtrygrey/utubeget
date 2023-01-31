use anyhow::{anyhow, Result};
use std::process::Command;

pub fn get_book_name(link: &str) -> String {
    let output: String = fetch_vid_name(link);
    output
}

// TODO loop until you get video name or video file

/// Clean youtube-dl cache, good thing to avoid error 403
pub fn clean_cache() -> Result<()> {
    Command::new("youtube-dl").arg("--rm-cache-dir").output()?;
    Ok(())
}

/// Downloads audio from youtube
pub fn fetch_audio(retries: u32, dir: &str, link: &str) -> Result<()> {
    for retry in 0..retries {
        print!("Download {}, try {}", &link, &retry);
        download_audio(dir, link)?;
    }
    Ok(())
}

pub fn download_audio(dir: &str, link: &str) -> Result<()> {
    let mut thread = Command::new("/usr/bin/youtube-dl")
        .arg("-f")
        .arg("bestaudio[ext=m4a]")
        .arg("-x")
        .arg("--audio-format")
        .arg("mp3")
        .arg("--write-thumbnail")
        .arg("--fragment-retries")
        .arg("10")
        .arg("-R")
        .arg("10")
        .arg("-o")
        .arg(&format!("{}/%(id)s.%(ext)s", &dir))
        .arg(link)
        .spawn()
        .expect("Error: couldn't create youtube-dl thread");

    if let Err(error) = thread.wait() {
        return Err(anyhow!("Failed to download audio, code: {:?}", error));
    }

    Ok(())
}

/// Downloads video name from youtube
fn fetch_vid_name(link: &str) -> String {
    let youtube_call = Command::new("/usr/bin/youtube-dl")
        .arg("-e")
        .arg(link)
        .output()
        .expect(&format!("Error: Couldn't get {} video", &link));

    let output = String::from_utf8(youtube_call.stdout).unwrap();
    output
}

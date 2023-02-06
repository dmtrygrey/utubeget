use anyhow::{anyhow, Result};
use std::process::{Command};

pub fn get_book_name(link: &str) -> String {
    let output: String = fetch_vid_name(link);
    output
}

/// Clean youtube-dl cache, good thing to avoid error 403
pub fn clean_cache() -> Result<()> {
    Command::new("youtube-dl").arg("--rm-cache-dir").output()?;
    Ok(())
}

/// Downloads audio from youtube
pub fn fetch_audio(retries: u32, dir: &str, link: &str) -> Result<()> {
    for retry in 1..=retries {
        log::debug!("Download {}, attempt {}", &link, &retry);
        match download_audio(dir, link) {
            Ok(_) => {
                log::debug!("Successfully downloaded {}", &link);
                break;
            },
            Err(e) => log::error!("Error: {}, downloading: {}", e, link),
        }
    }
    Ok(())
}

pub fn download_audio(dir: &str, link: &str) -> Result<()> {
    let mut thread = Command::new("/usr/bin/youtube-dl")
        .arg("-c")
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

    let exit_code = thread.wait().expect("Failed to wait on child process");
    match exit_code.success() {
        true => {
            log::debug!("Youtube-dl exited with success, exit code: {:?}", exit_code);
            return Ok(())
        },
        false => {
            log::error!("Youtube-dl error happend , exit code {:?}", exit_code);
            return Err(anyhow!("Failed to download audio, code: {:?}", exit_code));
        }
    }
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

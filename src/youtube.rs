use thiserror::Error;
use std::process::Command;

#[derive(Debug, Error)]
#[non_exhaustive]
pub enum DownloadError {
    #[error("Server error")]
    ConnectionError,
    #[error("Some other error")]
    SomeOtherError,
}

pub fn get_book_name(link: &str) -> String {
    let output: String = fetch_vid_name(link);
    output 
}

// TODO loop until you get video name or video file

/// Clean youtube-dl cache, good thing to avoid error 403
pub fn clean_cache() -> Result<String, String> {
    let cmd = Command::new("youtube-dl")
        .arg("--rm-cache-dir")
        .output()
        .unwrap();

    match cmd.status.success() {
        true => Ok("Success".to_string()),
        false => return Err(format!("Youtube-dl error: {:?}", cmd.status)),
    }
}

/// Downloads audio from youtube
pub fn fetch_audio(dir: &str, link: &str) -> Result<String, String> {
    let mut thread = Command::new("/usr/bin/youtube-dl")
        .arg("-f").arg("bestaudio[ext=m4a]")
        .arg("-x")
        .arg("--audio-format").arg("mp3")
        .arg("--write-thumbnail")
        .arg("--fragment-retries").arg("10")
        .arg("-R").arg("10")
        .arg("-o").arg(&format!("{}/%(id)s.%(ext)s", &dir))
        .arg(link)
        .spawn()
        .expect("Error: couldn't create youtube-dl thread");

    let status = thread.wait().unwrap();

    match status.success() {
        true => Ok("Success".to_string()),
        false => return Err(format!("Youtube-dl error: {:?}", status)),
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
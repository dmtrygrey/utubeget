use std::process::Command;

#[non_exhaustive]
pub enum DownloadError {
    ConnectionError,
    SomeOtherError,
}

pub fn get_book_name(link: &str) -> String {
    let output: String = fetch_vid_name(link);
    output 
}

// TODO loop until you get video name or video file

/// Downloads audio from youtube
pub fn fetch_audio( link: &str ) -> Result<String, String> {
    let mut thread = Command::new("/usr/bin/youtube-dl")
        .arg("-f")
        .arg("bestaudio[ext=m4a]")
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
fn fetch_vid_name( link: &str ) -> Result<String, std::error {
    let youtube_call = Command::new("/usr/bin/youtube-dl")
        .arg("-e")
        .arg(link)
        .output()
        .expect(&format!("Error: Couldn't get {} video", &link));

    let output = String::from_utf8(youtube_call.stdout).unwrap();
    output
}
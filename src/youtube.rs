use std::process::Command;

pub fn get_book_name(link: &str) -> String {
    let output: String = fetch_vid_name(link);
    output 
}

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
fn fetch_vid_name( link: &str ) -> String {
    let youtube_call = Command::new("/usr/bin/youtube-dl")
        .arg("-e")
        .arg(link)
        .output()
        .expect(&format!("Error: Couldn't get {} video", &link));

    let output = String::from_utf8(youtube_call.stdout).unwrap();
    output
}
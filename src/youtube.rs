use std::process::Command;

pub fn get_book_name(link: &str) -> String {
    let output: String = fetch_vid_name(link);
    output 
}

fn fetch_audio( link: &str ) {
//youtube-dl -f 'bestaudio[ext=m4a]' 'http://youtu.be/hTvJoYnpeRQ'
    let _youtube_call = Command::new("/usr/bin/youtube-dl")
        .arg("-f")
        .arg("bestaudio[ext=mp3]")
        .arg(link)
        .output()
        .unwrap();
}

fn fetch_vid_name( link: &str ) -> String {
    let youtube_call = Command::new("/usr/bin/youtube-dl")
        .arg("-e")
        .arg(link)
        .output()
        .unwrap();

    let output = String::from_utf8(youtube_call.stdout).unwrap();
    output
}



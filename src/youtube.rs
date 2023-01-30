use std::process::Command;

pub fn get_book_name(link: &str) -> String {
    let output: String = fetch_vid_name(link);
    output 
}

pub fn fetch_audio( link: &str ) {
    let youtube_call = Command::new("/usr/bin/youtube-dl")
        .arg("-f")
        .arg("bestaudio[ext=mp3]")
        .arg(link)
        .output()
        .expect(&format!("Error: couldn't download {}", &link));

    match youtube_call.status.success() {
        true => println!("Download successful?"),
        false => println!("Some error {:?}", &youtube_call.status),
    }
}

fn fetch_vid_name( link: &str ) -> String {
    let youtube_call = Command::new("/usr/bin/youtube-dl")
        .arg("-e")
        .arg(link)
        .output()
        .expect(&format!("Error: Couldn't get {} video", &link));

    let output = String::from_utf8(youtube_call.stdout).unwrap();
    output
}
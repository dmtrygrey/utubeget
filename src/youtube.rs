use std::process::Command;
//use crate::filter::*;

pub fn get_book_name(link: String) -> String {
    let output: String = fetch_vid_name(link);

    output 
}

fn fetch_vid_name( link: String ) -> String {
    let youtube_call = Command::new("/usr/bin/youtube-dl")
        .arg("-e")
        .arg(link)
        .output()
        .unwrap();

    let output = String::from_utf8(youtube_call.stdout).unwrap();
    output
}


#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_must_use)]
#![allow(dead_code)]
#![allow(unused_mut)]
use std::error::Error;
use utubeget::file::read_urls;
use utubeget::filter::filter;
use utubeget::make_dir::create_directory;
use utubeget::youtube::{get_book_name, fetch_audio};

// TODO
// Read file with urls
// loop over urls
// Get name of video from url
// convert url into latin and create folder with such name with no spaces etc
// load only audio type quality and convert it to mp3
// TODO if error, log to file this url

fn main() -> Result<(), Box<dyn Error>> {
    let book_urls = read_urls("urls.txt").unwrap();

    book_urls.into_iter().for_each(|url| {
        let book_name: String = get_book_name(&url);
        let latin_name: String = filter(&book_name);
        let newdir: String = String::from("/home/bit/Desktop/test");
        let bookdir = &format!("{}/{}", newdir, latin_name);
        match create_directory(&bookdir) {
            Ok(result) => { 
                println!("Created directory: {}", bookdir);
            },
            Err(error) => match error.kind() {
                std::io::ErrorKind::AlreadyExists => {
                    println!("Directory {} Already Exists!", &bookdir);
                    println!("download link: {}", &url);
                    fetch_audio(&url);
                },
                _ => println!("Error during directory creation: {}", &bookdir),
            },
        }
    });

    Ok(())
}

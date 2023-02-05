use anyhow::Context;
use std::error::Error;
use utubeget::file::read_urls;
use utubeget::filter::filter;
use utubeget::make_dir::create_directory;
use utubeget::youtube::{clean_cache, fetch_audio, get_book_name};

fn main() -> Result<(), Box<dyn Error>> {
    let book_urls = read_urls("urls.txt").unwrap();

    clean_cache().context("Cleaning youtube-dl cache")?;
    book_urls.into_iter().for_each(|url| {
        let book_name: String = get_book_name(&url);
        let latin_name: String = filter(&book_name);
        let newdir: String = String::from("/home/bit/Desktop/test");
        let bookdir = &format!("{}/{}", newdir, latin_name);
        create_directory(&bookdir).context("Creating directory").unwrap();
        fetch_audio(10, &bookdir, &url).unwrap();
    });

    Ok(())
}

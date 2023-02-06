use anyhow::Context;
use std::error::Error;
use utubeget::file::read_urls;
use utubeget::filter::filter;
use utubeget::make_dir::create_directory;
use utubeget::youtube::{clean_cache, fetch_audio, get_book_name};

// TODO add cli arg parser
fn main() -> Result<(), Box<dyn Error>> {
    env_logger::builder()
        .filter_level(log::LevelFilter::Debug)
        .format_target(false)
        .format_timestamp(None)
        .init();
    let book_urls = read_urls("urls.txt").unwrap();

    clean_cache().context("Cleaning youtube-dl cache")?;
    book_urls.into_iter().for_each(|url| {
        log::debug!("Try to process {}", url );
        let book_name: String = get_book_name(&url);
        let latin_name: String = filter(&book_name);
        let newdir: String = String::from("/home/bit/Desktop/test");
        let bookdir = &format!("{}/{}", newdir, latin_name);
        create_directory(&bookdir)
            .context("Creating directory")
            .unwrap();
        fetch_audio(10, &bookdir, &url).unwrap();
    });

    Ok(())
}

use anyhow::Context;
use std::error::Error;
use utubeget::file::read_urls;
use utubeget::filter::filter;
use utubeget::make_dir::create_directory;
use utubeget::youtube::{clean_cache, fetch_audio, get_book_name};
use utubeget::args;

// TODO:
// Add quiet option for youtube-dl
// Add option for enabling debug level, otherwise lnfo by default
fn main() -> Result<(), Box<dyn Error>> {
    env_logger::builder()
        .filter_level(log::LevelFilter::Debug)
        .format_target(false)
        .format_timestamp(None)
        .init();

    let cli_args = args::parse_cli_args().unwrap();

    let book_urls = read_urls(&cli_args.filename).unwrap();

    clean_cache().context("Cleaning youtube-dl cache")?;
    book_urls.into_iter().for_each(|url| {
        log::debug!("Try to process {}", &url);
        let book_name: String = get_book_name(&url).unwrap();
        let latin_name: String = filter(&book_name);
        let bookdir = &format!("{}/{}", &cli_args.output_dir, latin_name);
        create_directory(&bookdir)
            .context("Creating directory")
            .unwrap();
        fetch_audio(cli_args.retry_num, &cli_args.output_dir, &url).unwrap();
    });

    Ok(())
}

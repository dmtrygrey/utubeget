use std::error::Error;
use utubeget::args;
use utubeget::filter::filter;
use utubeget::fs::{create_directory, read_urls};
use utubeget::youtube::YoutubeDownloader;

fn main() -> Result<(), Box<dyn Error>> {
    env_logger::builder()
        .format_target(false)
        .format_timestamp(None)
        .init();

    let cli_args = args::parse_cli_args().unwrap();
    let book_urls = read_urls(&cli_args.filename).unwrap();
    let mut downloader = YoutubeDownloader::new();
    downloader.retries(6);
    
    for url in book_urls {
        log::debug!("Try to process {}", &url);
        downloader.link(&url);
        let book_name = downloader.video_name().unwrap();
        let latin_name = filter(&book_name);
        let bookdir = format!("{}/{}", &cli_args.output_dir, latin_name);
        create_directory(&bookdir).unwrap();
        downloader.location(&bookdir);
        downloader.fetch_audio().unwrap();
    };

    Ok(())
}

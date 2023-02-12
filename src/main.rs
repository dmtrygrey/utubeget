use anyhow::Context;
use std::error::Error;
use utubeget::file::read_urls;
use utubeget::filter::filter;
use utubeget::make_dir::create_directory;
use utubeget::youtube::{fetch_audio, get_book_name};
use utubeget::args;

// TODO:
// Add option for enabling debug level, otherwise lnfo by default
// Create file system module. To create dirs there and check if something exists etc..
// Make youtube object or tuple. With link and folder name in latin. Might help with debugging?
fn main() -> Result<(), Box<dyn Error>> {
    env_logger::builder()
        .format_target(false)
        .format_timestamp(None)
        .init();

    let cli_args = args::parse_cli_args().unwrap();

    let book_urls = read_urls(&cli_args.filename).unwrap();

    book_urls.into_iter().for_each(|url| {
        log::debug!("Try to process {}", &url);
        let book_name: String = get_book_name(&url).unwrap();
        let latin_name: String = filter(&book_name);
        let bookdir = &format!("{}/{}", &cli_args.output_dir, latin_name);
        create_directory(&bookdir)
            .context("Creating directory")
            .unwrap();
        fetch_audio(cli_args.retry_num, &bookdir, &url).unwrap();
    });

    Ok(())
}

use clap::Parser;
use anyhow::{anyhow, Result};
use log;
use crate::make_dir::create_directory;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /* File name with list of youtube urls */
    #[arg(short, long)]
    filename: String,

    /* Root directory for the books */
    #[arg(short, long)]
    output_dir: String,

    /* Number of retries to download one file */
    #[arg(short, long, default_value_t=5)]
    retries: u32,

    /* Disable yt-dlp output */
    #[arg(short, long)]
    quiet: bool,
}

#[derive(Default)]
pub struct CliParseResults {
    pub filename: String,
    pub output_dir: String,
    pub retry_num: u32,
    pub quiet: bool,
}

#[deny(unused_must_use)]
impl CliParseResults {
    fn new() -> Self {
        Self {
            ..Default::default()
        } 
    } 
    fn retries(mut self, number: u32) -> Self {
        self.retry_num = number;
        self
    }
    fn output_dir(mut self, output_dir: &str) -> Self {
        self.output_dir = output_dir.to_string();
        self
    }
    fn filename(mut self, filename: &str) -> Self {
        self.filename = filename.to_string();
        self
    }
    fn quiet(mut self, flag: bool) -> Self {
        self.quiet = flag;
        self
    }
}

pub fn parse_cli_args() -> Result<CliParseResults> {
    let args = Args::parse();

    let cliargs = CliParseResults::new()
        .filename(&args.filename)
        .output_dir(&args.output_dir)
        .quiet(args.quiet)
        .retries(args.retries);

    is_exists(&cliargs.filename)?;
    if let Err(_) = is_exists(&cliargs.output_dir) {
        create_directory(&cliargs.output_dir)?;
    }

    log::debug!("Arg: file name: {}", &cliargs.filename);
    log::debug!("Arg: root directory: {:?}", &cliargs.output_dir);
    log::debug!("Arg: number of retries: {}", &cliargs.retry_num);

    Ok(cliargs)
}

fn is_exists(path: &str) -> Result<()> {
    if !std::path::Path::new(&path).exists() {
        let warn = String::from(format!("File/Path doesn't exist: {}", &path));
        log::warn!("{}", &warn);
        Err(anyhow!(warn))
    } else {
        Ok(())
    }
}
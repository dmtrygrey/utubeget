use std::path::Path;
use clap::Parser;
use anyhow::{anyhow, Result};

/// Simple program to greet a person
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
}

pub struct CliParseResults {
    filename: Box<Path>,
    output_dir: Box<Path>,
    retry_num: u32,
}

pub fn parse_cli_args() -> Result<()> {
    let args = Args::parse();

    println!("File name: {}", args.filename);
    println!("Root directory: {:?}", args.output_dir);
    println!("Number of retries: {}", args.retries);


    Ok(())
}
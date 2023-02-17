use crate::fs;
use anyhow::{anyhow, Result};
use std::process::Command;
use std::{thread, time};

#[derive(Default)]
pub struct YoutubeDownloader {
    tool: String,
    link: String,
    download_location: String,
    retries: u32,
}

impl YoutubeDownloader {
    pub fn new() -> Self {
        Self {
            tool: fs::find_youtube_downloader().unwrap(),
            retries: 5,
            ..Default::default()
        }
    }
    pub fn link(&mut self, link: &str) {
        self.link = link.to_string();
    }
    pub fn retries(&mut self, retries: u32) {
        self.retries = retries;
    }
    pub fn location(&mut self, location: &str) {
        self.download_location = location.to_string();
    }
    pub fn video_name(&self) -> Result<String> {
        log::debug!("Getting video name from: {}", &self.link);
        let youtube_call = Command::new(&self.tool)
            .arg("-e")
            .arg(&self.link)
            .output()
            .expect("Error: run yt-dlp process");

        match youtube_call.status.code() {
            Some(code) => match code {
                0 => {
                    log::debug!("yt-dlp download video name success");
                    let output = String::from_utf8(youtube_call.stdout).unwrap();
                    Ok(output)
                },
                1..=256 => {
                    let error = String::from(format!(
                        "Error during yt-dlp video name download, code {}. Don't forget to update {}!",
                        code, &self.tool
                    ));
                    log::error!("{}", &error);
                    Err(anyhow!(error))
                },
                _ => {
                    let error = String::from(format!("Unknown error: {}", code));
                    log::error!("{}", &error);
                    Err(anyhow!(error))
                },
            },
            None => {
                let error = String::from(format!("Error status of shell call"));
                log::error!("{}", &error);
                Err(anyhow!(error))
            }
        }
    }
    pub fn fetch_audio(&self) -> Result<()> {
        let mut attemps = 0;
        while attemps != self.retries {
            log::debug!(
                "Download {}, attempt {}, left: {}",
                &self.link,
                &attemps,
                &self.retries - &attemps
            );
            match self.try_download_audio() {
                Ok(_) => {
                    log::debug!("Successfully downloaded {}", &self.link);
                    break;
                }
                Err(e) => {
                    log::warn!(
                        "{}, while downloading: {}, go to sleep 2 sec",
                        e,
                        &self.link
                    );
                    thread::sleep(time::Duration::from_secs(2));
                }
            }
            attemps += 1;
        }

        if attemps == self.retries {
            let error = String::from(format!("Exhausted number of attemps to download"));
            log::warn!("{}", &error);
            Err(anyhow!(error))
        } else {
            Ok(())
        }
    }

    fn try_download_audio(&self) -> Result<()> {
        log::debug!("Donwloading: {}", &self.link);
        let mut thread = Command::new(&self.tool)
            .arg("-c")
            .arg("-q")
            .arg("--progress")
            .arg("-f")
            .arg("bestaudio[ext=m4a]")
            .arg("-x")
            .arg("--audio-format")
            .arg("mp3")
            .arg("--write-thumbnail")
            .arg("--fragment-retries")
            .arg("10")
            .arg("-R")
            .arg("10")
            .arg("-o")
            .arg(&format!("{}/%(id)s.%(ext)s", &self.download_location))
            .arg(&self.link)
            .spawn()
            .expect("Error: couldn't create yt-dlp thread");

        let exit_code = thread.wait().expect("Failed to wait on child process");
        match exit_code.success() {
            true => Ok(()),
            false => Err(anyhow!(
                "yt-dlp issue happend, exit code {:?}",
                exit_code.code().unwrap()
            )),
        }
    }
}
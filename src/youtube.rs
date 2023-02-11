use anyhow::{anyhow, Result};
use std::process::Command;
use std::{thread, time};

pub fn get_book_name(link: &str) -> Result<String> {
    let output: String = fetch_vid_name(link)?;
    Ok(output)
}

/// Clean youtube-dl cache, good thing to avoid error 403
pub fn clean_cache() -> Result<()> {
    Command::new("youtube-dl").arg("--rm-cache-dir").output()?;
    Ok(())
}

/// Downloads audio from youtube
pub fn fetch_audio(retries: u32, dir: &str, link: &str) -> Result<()> {
    let mut attemps = 0;
    while attemps != retries {
        log::debug!("Download {}, attempt {}, left: {}", &link, &attemps, &retries-&attemps);
        match download_audio(dir, link) {
            Ok(_) => {
                log::debug!("Successfully downloaded {}", &link);
                break;
            }
            Err(e) => {
                log::warn!("{}, while downloading: {}, go to sleep 2 sec", e, link);
                thread::sleep(time::Duration::from_secs(2));
            },
        }
        attemps += 1;
    }

    if attemps == retries {
        let error = String::from(format!("Exhausted number of attemps to download"));
        log::warn!("{}", &error);
        Err(anyhow!(error))
    } else {
        Ok(())
    }
}

pub fn download_audio(dir: &str, link: &str) -> Result<()> {
    log::debug!("Donwloading: {}", &link);
    let mut thread = Command::new("/usr/bin/youtube-dl")
        .arg("-c")
        .arg("-f")
        .arg("bestaudio[ext=m4a]")
        .arg("-x").arg("--audio-format").arg("mp3")
        .arg("--write-thumbnail")
        .arg("--fragment-retries").arg("10")
        .arg("-R").arg("10")
        .arg("-o").arg(&format!("{}/%(id)s.%(ext)s", &dir))
        .arg(link)
        .spawn()
        .expect("Error: couldn't create youtube-dl thread");

    let exit_code = thread.wait().expect("Failed to wait on child process");
    match exit_code.success() {
        true => Ok(()),
        false => Err(anyhow!("Youtube-dl issue happend, exit code {:?}", exit_code.code().unwrap())),
    }
}

/// Downloads video name from youtube
fn fetch_vid_name(link: &str) -> Result<String> {
    log::debug!("Getting video name from: {}", &link);
    let youtube_call = Command::new("/usr/bin/youtube-dl")
        .arg("-e")
        .arg(link)
        .output()
        .expect("Error: run youtube-dl process");

    match youtube_call.status.code() {
        Some(code) => match code {
            0 => {
                log::debug!("Youtube-dl download video name success");
                let output = String::from_utf8(youtube_call.stdout).unwrap();
                Ok(output)
            }
            1..=256 => {
                let error = String::from(format!(
                    "Error during Youtube-dl video name download, code {}",
                    code
                ));
                log::error!("{}", &error);
                Err(anyhow!(error))
            }
            _ => {
                let error = String::from(format!("Unknown error: {}", code));
                log::error!("{}", &error);
                Err(anyhow!(error))
            }
        },
        None => {
            let error = String::from(format!("Error status of shell call"));
            log::error!("{}", &error);
            Err(anyhow!(error))
        }
    }
}
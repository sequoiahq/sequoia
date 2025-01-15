mod utils {
    pub mod logger;
}

mod modules {
    pub mod atresplayer;
    pub mod bilibili;
    pub mod cookies;
    pub mod deezer;
    pub mod distrotv;
    pub mod download;
    pub mod nbc;
}

use clap::{App, Arg};
use modules::download::download_video;
use std::process::exit; // Importing the function

#[tokio::main]
async fn main() {
    let matches = App::new("Sequoia")
        .version("1.0")
        .about("Reverse Engineering Toolkit")
        .arg(
            Arg::new("dl")
                .short('d')
                .long("download")
                .takes_value(true)
                .help("Download media by specifying service and URL in the format: SERVICE,URL,[COOKIE_FILE]"),
        )
        .get_matches();

    // dl arg
    if let Some(arg) = matches.value_of("dl") {
        let parts: Vec<&str> = arg.split(',').collect();
        if parts.len() < 2 || parts.len() > 3 {
            eprintln!("Usage: --download <SERVICE>,<URL>[,COOKIE_FILE]");
            exit(1);
        }

        let service = parts[0].to_lowercase();
        let url = parts[1];
        let cookie_file = parts.get(2).cloned();

        match service.as_str() {
            "bilibili" => {
                if let Err(e) = handle_bilibili(url).await {
                    eprintln!("Error with Bilibili service: {}", e);
                }
            }
            "nbc" => {
                if let Err(e) = handle_nbc(url).await {
                    eprintln!("Error with NBC service: {}", e);
                }
            }
            "atresplayer" => {
                if let Some(cookie_file) = cookie_file {
                    if let Err(e) = handle_atresplayer(url, cookie_file).await {
                        eprintln!("Error with Atresplayer service: {}", e);
                    }
                } else {
                    eprintln!(
                        "Service requires a cookie file: --download <SERVICE>,<URL>,<COOKIE_FILE>"
                    );
                    exit(1);
                }
            }
            "distrotv" => {
                if let Err(e) = handle_distrotv(url).await {
                    eprintln!("Error with DistroTV service: {}", e);
                }
            }
            _ => {
                eprintln!("Unsupported service: {}", service);
                exit(1);
            }
        }
    }

    // fairplay arg
 /*    if let Some(arg) = matches.value_of("fairplay") {
        let parts: Vec<&str> = arg.split(',').collect();
        if parts.len() != 2 {
            eprintln!("Usage: --fairplay <SRC>,<DEST>");
            exit(1);
        }

        let src = parts[0];
        let dest = parts[1];

        match modules::fairplay::decrypt(src, dest) {
            Ok(_) => println!("FairPlay decryption succeeded."),
            Err(e) => eprintln!("FairPlay decryption failed: {}", e),
        }
    }*/
}

// b-global
async fn handle_bilibili(url: &str) -> Result<(), Box<dyn std::error::Error>> {
    modules::bilibili::service::fetch_manifest_url(url).await?;
    Ok(())
}

async fn handle_nbc(url: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("Fetching video from NBC...");
    match modules::nbc::service::fetch_video_url(url) {
        Ok(playback_url) => {
            println!("Playback URL: {}", playback_url);
            Ok(())
        }
        Err(e) => Err(e),
    }
}

// a3p
async fn handle_atresplayer(
    url: &str,
    cookie_file: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    modules::atresplayer::service::download_episode(url, cookie_file).await?;
    Ok(())
}

// distrotv
async fn handle_distrotv(url: &str) -> Result<(), Box<dyn std::error::Error>> {
    let show_data = modules::distrotv::service::get_api_data(url).await?;
    let m3u8_url = modules::distrotv::service::get_m3u8_url(&show_data)?;
    let filename = modules::distrotv::service::create_filename(&show_data)?;

    println!("Starting download: {} -> {}", m3u8_url, filename);
    download_video(&m3u8_url, &filename, None)?; // Call refactored function

    println!("Download complete: {}", filename);
    Ok(())
}

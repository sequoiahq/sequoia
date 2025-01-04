mod utils {
    pub mod logger;
}

mod modules {
    pub mod atresplayer;
    pub mod cookies;
    pub mod deezer;
    pub mod nbc;
}

use clap::{App, Arg};
use std::process::exit;

#[tokio::main]
async fn main() {
    // Initialize CLI argument parser
    let matches = App::new("tumble - widevine DRM downloader")
        .version("1.0")
        .about("Downloads media content")
        .arg(
            Arg::new("keys")
                .short('k')
                .long("keys")
                .takes_value(true)
                .help("Run the Deezer handler with the given key"),
        )
        .arg(
            Arg::new("dl")
                .short('d')
                .long("download")
                .takes_value(true)
                .help("Fetch the NBC video with the given URL"),
        )
        .arg(
            Arg::new("atresplayer")
                .short('a')
                .long("atresplayer")
                .takes_value(true)
                .help("Fetch the Atresplayer video with the given URL and cookies file"),
        )
        .get_matches();

    // Handle 'keys' argument for Deezer
    if let Some(key) = matches.value_of("keys") {
        if key == "deezer" {
            match crate::modules::deezer::cbc::fetch_and_decode() {
                Ok(final_result) => println!("Key from bundle: {}", final_result),
                Err(e) => eprintln!("Error occurred: {}", e),
            }
        }
    }

    // Handle 'dl' argument for NBC
    if let Some(url) = matches.value_of("dl") {
        println!("Fetching video from NBC...");
        match crate::modules::nbc::service::fetch_video_url(url) {
            Ok(playback_url) => println!("Playback URL: {}", playback_url),
            Err(e) => eprintln!("Error occurred: {}", e),
        }
    }

    // Handle 'atresplayer' argument for Atresplayer
    if let Some(arg) = matches.value_of("atresplayer") {
        let parts: Vec<&str> = arg.split(',').collect();
        if parts.len() != 2 {
            eprintln!("Usage: --atresplayer <episode_url>,<cookie_file_path>");
            exit(1);
        }

        let episode_url = parts[0];
        let cookie_file = parts[1];

        // Get cookies from the Netscape-style cookie file
        match crate::modules::atresplayer::service::fetch_cookies(cookie_file) {
            Ok(cookies) => {
                let episode_id = episode_url
                    .split("_")
                    .last()
                    .unwrap_or_default()
                    .split("/")
                    .next()
                    .unwrap_or_default();
                let api_url = format!(
                    "https://api.atresplayer.com/player/v1/episode/{}?NODRM=true",
                    episode_id
                );

                match crate::modules::atresplayer::service::get_dash_hevc_source(&api_url, &cookies)
                    .await
                {
                    Ok(src) => println!("Found DASH HEVC source: {}", src),
                    Err(e) => eprintln!("Error occurred: {}", e),
                }
            }
            Err(e) => eprintln!("Failed to load cookies: {}", e),
        }
    }
}

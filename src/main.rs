mod utils {
    pub mod logger;
}

mod modules {
    pub mod atresplayer;
    pub mod cookies;
    pub mod deezer;
    pub mod nbc;
    pub mod bilibili;
}

use clap::{App, Arg};
use std::process::exit;
use tokio;

#[tokio::main]
async fn main() {
    // cli arg parser
    let matches = App::new("tumble - widevine DRM downloader")
        .version("1.0")
        .about("Downloads media content")
        .arg(
            Arg::new("dl")
                .short('d')
                .long("download")
                .takes_value(true)
                .help("Download media by specifying service and URL in the format: SERVICE,URL,[COOKIE_FILE]"),
        )
        .get_matches();

    // dl argument for services
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
                if let Err(e) = modules::bilibili::service::fetch_manifest_url(url).await {
                    eprintln!("Error with Bilibili service: {}", e);
                }
            }
            "nbc" => {
                println!("Fetching video from NBC...");
                match modules::nbc::service::fetch_video_url(url) {
                    Ok(playback_url) => println!("Playback URL: {}", playback_url),
                    Err(e) => eprintln!("Error occurred: {}", e),
                }
            }
            "atresplayer" => {
                if let Some(cookie_file) = cookie_file {
                    if let Err(e) = modules::atresplayer::service::download_episode(url, cookie_file).await {
                        eprintln!("Error with Atresplayer service: {}", e);
                    }
                } else {
                    eprintln!("Atresplayer requires a cookie file: --download atresplayer,<URL>,<COOKIE_FILE>");
                    exit(1);
                }
            }
            _ => {
                eprintln!("Unsupported service: {}", service);
                exit(1);
            }
        }
    }
}

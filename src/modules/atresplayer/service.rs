/*

Service code for Atresplayer
Written by @matt

Authorization: Cookies
Security: UHD@L3 - UHD@SL2000.
          Almost all titles aren't encrypted, only new ones and only if
          you use the MPD (required to get 2160p). Using HLS you get up 
          to 1080p unencrypted. */

use crate::modules::cookies;
use reqwest::Client;
use serde::Deserialize;
use std::collections::HashMap;
use std::process::{Command, ExitStatus};

#[derive(Deserialize, Debug)]
struct Source {
    #[serde(rename = "type")]
    source_type: String,
    src: String,
}

#[derive(Deserialize, Debug)]
struct ApiResponse {
    sources: Vec<Source>,
}

pub async fn download_episode(url: &str, cookie_file: &str) -> Result<(), String> {
    let cookies = fetch_cookies(cookie_file)?;
    let episode_id = url
        .split('_')
        .last()
        .unwrap_or_default()
        .split('/')
        .next()
        .unwrap_or_default();
    let api_url = format!(
        "https://api.atresplayer.com/player/v1/episode/{}?NODRM=true",
        episode_id
    );

    match get_dash_hevc_source(&api_url, &cookies).await {
        Ok(src) => {
            let status = Command::new("N_m3u8DL-RE")
                .arg(src)
                .spawn()
                .map_err(|e| format!("Failed to execute N_m3u8DL-RE: {}", e))?
                .wait()
                .map_err(|e| format!("Failed to wait for process to finish: {}", e))?;

            if status.success() {
                Ok(())
            } else {
                Err(format!(
                    "Process failed with exit code: {}",
                    status.code().unwrap_or(-1)
                ))
            }
        }
        Err(e) => Err(format!("Failed to get DASH HEVC source: {}", e)),
    }
}

pub async fn get_dash_hevc_source(
    url: &str,
    cookies: &HashMap<String, String>,
) -> Result<String, String> {
    let client = Client::new();

    let mut cookie_header = String::new();
    for (name, value) in cookies {
        cookie_header.push_str(&format!("{}={}; ", name, value));
    }

    let res = client
        .get(url)
        .header("User-Agent", "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/128.0.0.0 Safari/537.36")
        .header("Cookie", cookie_header)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if !res.status().is_success() {
        return Err(format!("Request failed with status: {}", res.status()));
    }

    let data: ApiResponse = res.json().await.map_err(|e| e.to_string())?;

    for source in data.sources {
        if source.source_type == "application/dash+hevc" {
            return Ok(source.src);
        }
    }

    Err("No 'application/dash+hevc' source found.".to_string())
}

pub fn fetch_cookies(cookie_file: &str) -> Result<HashMap<String, String>, String> {
    match cookies::get_cookies_from_netscape(cookie_file) {
        Ok(cookies) => Ok(cookies),
        Err(e) => Err(format!("Failed to load cookies: {}", e)),
    }
}

/*

Service code for Atresplayer
Written by @matt

Authorization: Cookies
Security: UHD@L3 - UHD@SL2000. (WIP)
          Some titles aren't encrypted.
          This implementation only supports unencrypted (most of the content, by the way), but
          decryption is a generic Widevine/PlayReady request without special headers.
*/

use crate::modules::cookies;
use reqwest::Client;
use serde::Deserialize;
use std::collections::HashMap;

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
        .header("User-Agent", "Mozilla/5.0")
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

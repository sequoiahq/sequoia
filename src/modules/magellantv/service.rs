use regex::Regex;
use std::error::Error as StdError;
use reqwest;
use serde_json::Value;

pub async fn fetch_video_data(
    url: &str,
    video_type: &str,
) -> Result<Value, Box<dyn StdError>> {
    let slug = url.split('/').last().ok_or("Invalid URL format")?;
    let api_url = format!(
        "https://www.magellantv.com/_next/data/Aj-saEZLNOJRaI5GU039w/watch/{}.json?type={}&slug={}",
        slug, video_type, slug
    );

    let response = reqwest::get(&api_url).await?;
    let json: Value = serde_json::from_str(&response.text().await?)?;
    Ok(json)
}

pub(crate) fn get_m3u8_url(data: &Value) -> Result<String, String> {
    if let Some(video_manifests) = data.pointer("/video/manifests") {
        if let Some(m3u8_url) = video_manifests.pointer("/v1/hls").and_then(|v| v.as_str()) {
            return Ok(m3u8_url.to_string());
        }
        if let Some(m3u8_url) = video_manifests.pointer("/v2/hls").and_then(|v| v.as_str()) {
            return Ok(m3u8_url.to_string());
        }
    }

    if let Some(seasons) = data.pointer("/pageProps/reactContext/series/seasons").and_then(|v| v.as_array()) {
        for season in seasons {
            if let Some(episode_list) = season.pointer("/episodeList").and_then(|v| v.as_array()) {
                for episode in episode_list {
                    if let Some(manifests) = episode.pointer("/manifests") {
                        if let Some(m3u8_url) = manifests.pointer("/v1/hls").and_then(|v| v.as_str()) {
                            return Ok(m3u8_url.to_string());
                        }
                        if let Some(m3u8_url) = manifests.pointer("/v2/hls").and_then(|v| v.as_str()) {
                            return Ok(m3u8_url.to_string());
                        }
                    }
                }
            }
        }
    }
    let re = Regex::new(r"https://media\.magellantv\.com(?:/[a-zA-Z0-9._/-]*)?\.m3u8").unwrap();
    let json_str = serde_json::to_string(data).map_err(|_| "Error serializing JSON")?;
    if let Some(caps) = re.captures(&json_str) {
        if let Some(m3u8_url) = caps.get(0) {
            return Ok(m3u8_url.as_str().to_string());
        }
    }

    Err("Error with service: m3u8 URL not found".to_string())
}

fn find_title(json: &Value) -> Option<String> {
    if let Some(title) = json.get("title").and_then(|v| v.as_str()) {
        return Some(title.to_string());
    }

    if let Some(obj) = json.as_object() {
        for (key, value) in obj {
            if key != "lucidVideo" {
                if let Some(title) = find_title(value) {
                    return Some(title);
                }
            }
        }
    }
    None
}

pub fn create_filename(
    json: &serde_json::Value,
    video_type: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    println!("JSON data: {:?}", json);

    let mut filename = String::new();

    if let Some(title) = find_title(json) {
        println!("Title found: {}", title);

        let title = title
            .replace("MagellanTV", "")
            .replace("_", "")
            .replace(":", "")
            .replace("-", " ");

        let title = title.trim().replace("  ", " ");
        let words: Vec<&str> = title.split_whitespace().collect();
        let title = words.join(".");

        
        if video_type == "s" { // series
            println!("Series found");
            if let Some(seasons_data) = json.pointer("/pageProps/reactContext/series/seasons") {
                if let Some(seasons) = seasons_data.as_array() {
                    if let Some(season) = seasons.get(0) {
                        if let Some(episode_list_data) = season.pointer("/episodeList") {
                            if let Some(episode_list) = episode_list_data.as_array() {
                                if let Some(episode) = episode_list.get(0) {
                                    filename = format!(
                                        "{}.S{:02}E{:02}.1080p.MGT.WEB-DL.AAC2.0.H264-NOGRP",
                                        title,
                                        season["seasonNumber"].as_u64().unwrap(),
                                        episode["episodeNumber"].as_u64().unwrap()
                                    );
                                }
                            }
                        }
                    }
                }
            }
        } else if video_type == "v" { // movies
            println!("Movie found");
            filename = format!("{}.1080p.MGT.WEB-DL.AAC2.0.H264-NOGRP", title);
        }
    } else {
        println!("No title found");
        filename = "Untitled.1080p.MGT.WEB-DL.AAC2.0.H264-NOGRP".to_string();
    }

    println!("Filename: {}", filename);
    Ok(filename)
}

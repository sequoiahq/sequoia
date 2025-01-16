use regex::Regex;
use std::error::Error as StdError;

/*
    Service code
    Written by @matt

    Authorization: None
    Security: UHD@None
*/

pub async fn fetch_video_data(
    url: &str,
    video_type: &str,
) -> Result<serde_json::Value, Box<dyn StdError>> {
    let api_url = format!(
        "https://www.magellantv.com/_next/data/Aj-saEZLNOJRaI5GU039w/watch/{}.json?type={}&slug={}",
        url.split("/").last().unwrap(),
        video_type,
        url.split("/").last().unwrap()
    );

    let response = reqwest::get(api_url).await?;
    let json: serde_json::Value = serde_json::from_str(&response.text().await?)?;

    Ok(json)
}

pub(crate) fn get_m3u8_url(data: &serde_json::Value) -> Result<String, String> {
    let re = Regex::new(r"https?://media\.magellantv\.com/(?:[^/]+/)+[^/]+\.m3u8").unwrap();
    let json_str = serde_json::to_string(data).unwrap();
    if let Some(caps) = re.captures(&json_str) {
        if let Some(m3u8_url) = caps.get(0) {
            return Ok(m3u8_url.as_str().to_string());
        }
    }
    Err("Error with service: m3u8 URL not found".to_string())
}

fn find_title(json: &serde_json::Value) -> Option<String> {
    if let Some(title) = json.get("title") {
        if title.is_string() {
            return Some(title.as_str().unwrap().to_string());
        }
    }

    for (key, value) in json.as_object().unwrap_or(&serde_json::Map::new()) {
        if key == "lucidVideo" {
            continue;
        }

        if let Some(title) = find_title(value) {
            return Some(title);
        }
    }

    None
}

pub fn create_filename(json: &serde_json::Value) -> Result<String, Box<dyn StdError>> {
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

        if json["series"]["detail"]["seasons"].is_array() {
            println!("Series found");
            // Series
            let season = &json["series"]["detail"]["seasons"][0];
            let episode = &season["episodeList"][0];

            filename = format!(
                "{}.S{:02}E{:02}.1080p.MGT.WEB-DL.AAC2.0.H264-NOGRP",
                title,
                season["seasonNumber"].as_u64().unwrap(),
                episode["episodeNumber"].as_u64().unwrap()
            );
        } else {
            println!("No series found, checking for video");
            // Video
            filename = format!("{}.1080p.MGT.WEB-DL.AAC2.0.H264-NOGRP", title);
        }
    } else {
        println!("No title found");
        filename = "Untitled.1080p.MGT.WEB-DL.AAC2.0.H264-NOGRP".to_string();
    }

    println!("Filename: {}", filename);
    Ok(filename)
}

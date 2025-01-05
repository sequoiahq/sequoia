/*

Service code for DistroTV
Written by @matt

Authorization: None
Security: None
*/

use serde_json::Value;
use std::error::Error as StdError;
use std::process::Command;
use urlencoding::encode;

// fetch api data from url
pub async fn get_api_data(show_url: &str) -> Result<Value, Box<dyn StdError>> {
    let show_name = show_url
        .trim_end_matches('/')
        .split('/')
        .last()
        .ok_or("Invalid URL: Could not extract show name.")?;
    let show_name_encoded = encode(show_name);

    let api_url = format!(
        "https://tv.jsrdn.com/tv_v5/show.php?name={}",
        show_name_encoded
    );
    // println!("Fetching API data from: {}", api_url);

    let response = reqwest::get(&api_url).await?;
    let data = response.json::<Value>().await?;
    Ok(data)
}

pub fn get_m3u8_url(show_data: &Value) -> Result<String, String> {
    // println!("API Response: {:?}", show_data);s

    // extract show_id from response
    let show_id = show_data
        .get("shows")
        .and_then(|shows| shows.as_object())
        .and_then(|shows| shows.keys().next()) // Get the first key (show_id)
        .ok_or("No show found in the response.")?;

    //println!("Extracted show_id: {}", show_id);

    // get show object using show_id
    let show = show_data
        .get("shows")
        .and_then(|shows| shows.get(show_id))
        .ok_or("Show data not found for the extracted show_id.")?;

    // extract 1st episode
    let episode = show
        .get("seasons")
        .and_then(|seasons| seasons.as_array())
        .and_then(|seasons| seasons.first())
        .and_then(|season| season.get("episodes"))
        .and_then(|episodes| episodes.as_array())
        .and_then(|episodes| episodes.first())
        .ok_or("No episodes found in the show data.")?;

    // get m3u8
    let m3u8_url = episode
        .get("content")
        .and_then(|content| content.get("url"))
        .and_then(|url| url.as_str())
        .ok_or("M3U8 URL not found in the episode content.")?;

    Ok(m3u8_url.to_string())
}

// create filename from metadata. TO-DO -> P2P naming
pub fn create_filename(show_data: &Value) -> Result<String, Box<dyn StdError>> {
    // Dynamically extract the show_id from the response
    let show_id = show_data
        .get("shows")
        .and_then(|shows| shows.as_object())
        .and_then(|shows| shows.keys().next()) // Get the first key (show_id)
        .ok_or("No show found in the response.")?;

    // println!("Extracted show_id for filename creation: {}", show_id);

    let show_title = show_data
        .get("shows")
        .and_then(|shows| shows.get(show_id))
        .and_then(|show| show.get("title"))
        .and_then(|title| title.as_str())
        .ok_or("Show title not found")?;

    let episode_title = show_data
        .get("shows")
        .and_then(|shows| shows.get(show_id))
        .and_then(|show| show.get("seasons"))
        .and_then(|seasons| seasons[0].get("episodes"))
        .and_then(|episodes| episodes[0].get("title"))
        .and_then(|title| title.as_str())
        .ok_or("Episode title not found")?;

    let filename = format!(
        "{}_{}",
        show_title.replace(' ', "_").to_lowercase(),
        episode_title.replace(' ', "_").to_lowercase()
    );

    Ok(filename)
}

// dl video
pub fn download_video(
    m3u8_url: &str,
    filename: &str,
    quality: Option<&str>,
) -> Result<(), Box<dyn StdError>> {
    let mut command = vec![
        "./N_m3u8DL-RE",
        m3u8_url,
        "--save-name",
        filename,
        "--thread-count",
        "40",
        "--mux-after-done",
        "mkv",
        "--auto-select",
    ];

    if let Some(q) = quality {
        command.push("--select-video");
        command.push(q);
    }

    let status = Command::new(command[0]).args(&command[1..]).status()?;

    if !status.success() {
        Err("Download failed")?
    }

    Ok(())
}

pub async fn fetch_and_process_video(
    show_url: &str,
    quality: Option<&str>,
) -> Result<(), Box<dyn StdError>> {
    // println!("Fetching API data for URL: {}", show_url);

    let show_data = get_api_data(show_url).await?;
    let m3u8_url = get_m3u8_url(&show_data)?;
    let filename = create_filename(&show_data)?;

    // println!("Starting download: {} -> {}", m3u8_url, filename);
    download_video(&m3u8_url, &filename, quality)?;
    // println!("Download complete: {}", filename);
    println!("Download complete");

    Ok(())
}

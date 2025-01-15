/*

Service code for DistroTV
Written by @matt

Authorization: None
Security: None

Check the documentation for more detailed information about the API;
my code is a little bit of a mess and maybe you get it better there,
*/
use crate::modules::download::download_video;
use serde_json::Value;
use std::error::Error as StdError;
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

    let response = reqwest::get(&api_url).await?;
    let data = response.json::<Value>().await?;
    Ok(data)
}

pub fn get_m3u8_url(show_data: &Value) -> Result<String, String> {
    let show_id = show_data
        .get("shows")
        .and_then(|shows| shows.as_object())
        .and_then(|shows| shows.keys().next())
        .ok_or("No show found in the response.")?;

    let show = show_data
        .get("shows")
        .and_then(|shows| shows.get(show_id))
        .ok_or("Show data not found for the extracted show_id.")?;

    let episode = show
        .get("seasons")
        .and_then(|seasons| seasons.as_array())
        .and_then(|seasons| seasons.first())
        .and_then(|season| season.get("episodes"))
        .and_then(|episodes| episodes.as_array())
        .and_then(|episodes| episodes.first())
        .ok_or("No episodes found in the show data.")?;

    let m3u8_url = episode
        .get("content")
        .and_then(|content| content.get("url"))
        .and_then(|url| url.as_str())
        .ok_or("M3U8 URL not found in the episode content.")?;

    Ok(m3u8_url.to_string())
}

// create filename from metadata
pub fn create_filename(show_data: &Value) -> Result<String, Box<dyn StdError>> {
    let show_id = show_data
        .get("shows")
        .and_then(|shows| shows.as_object())
        .and_then(|shows| shows.keys().next())
        .ok_or("No show found in the response.")?;

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
pub async fn fetch_and_process_video(
    show_url: &str,
    quality: Option<&str>,
) -> Result<(), Box<dyn StdError>> {
    let show_data = get_api_data(show_url).await?;
    let m3u8_url = get_m3u8_url(&show_data)?;
    let filename = create_filename(&show_data)?;

    download_video(&m3u8_url, &filename, quality)?;

    println!("Download complete");

    Ok(())
}

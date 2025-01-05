use reqwest::Client;
use std::error::Error;

// in progress. NOT WORKING ATM

pub async fn fetch_manifest_url(url: &str) -> Result<(), Box<dyn Error>> {
    // parse videoid from url
    let video_id = extract_video_id(url).ok_or("Invalid URL format for Bilibili")?;

    // build api url
    let api_url = format!(
        "https://api.bilibili.tv/intl/gateway/web/playurl?ep_id={}&device=wap&platform=web&qn=64&tf=0&type=0",
        video_id
    );

    // load cookies from helper 
    let cookies = crate::modules::cookies::get_cookies_from_netscape("./bb.txt")?; // to-do: dynamic cookie loading in toml config
    let cookie_header = cookies
        .into_iter()
        .map(|(k, v)| format!("{}={}", k, v))
        .collect::<Vec<_>>()
        .join("; ");

    // setup http client
    let client = Client::new();
    let response = client
        .get(&api_url)
        .header("referer", "https://www.bilibili.tv/")
        .header("cookie", cookie_header)
        .send()
        .await?;

    if !response.status().is_success() {
        return Err(format!("Failed to fetch manifest: {}", response.status()).into());
    }

    let response_body = response.text().await?;
    println!("Response from Bilibili API: {}", response_body);

    Ok(())
}

// helper to extract vidID
fn extract_video_id(url: &str) -> Option<String> {
    let parsed_url = url::Url::parse(url).ok()?;
    let segments: Vec<&str> = parsed_url.path_segments()?.collect();

    if url.contains("/video/") {
        segments
            .iter()
            .position(|&s| s == "video")
            .and_then(|i| segments.get(i + 1))
            .map(|s| s.to_string())
    } else if url.contains("/play/") {
        let numeric_segments: Vec<&str> = segments
            .iter()
            .filter(|&s| s.chars().all(|c| c.is_numeric()))
            .copied()
            .collect();
        numeric_segments
            .get(1)
            .or_else(|| numeric_segments.get(0))
            .map(|s| s.to_string())
    } else {
        None
    }
}

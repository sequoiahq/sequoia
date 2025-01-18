/*

Service code for Crackle
Written by @matt

Authorization: None
Security: FHD@L3

TO-DO: better regex. only 1 title working at the moment
Cannot consider this a full service, just a draft.
*/
use regex::Regex;
use reqwest::{header::HeaderMap, Client};
use serde::Deserialize;
use std::error::Error;
use std::process::{Command, Stdio};

#[derive(Deserialize, Debug)]
struct VODResponse {
    data: Data,
}

#[derive(Deserialize, Debug)]
struct Data {
    contentId: String,
    mediaId: i32,
    mode: String,
    streams: Vec<Stream>,
    sidecar: Option<Vec<Sidecar>>,
}

#[derive(Deserialize, Debug)]
struct Stream {
    #[serde(rename = "type")]
    stream_type: String,
    url: String,
    drm: Option<Drm>,
}

#[derive(Deserialize, Debug)]
struct Drm {
    keyUrl: String,
    keyCert: String,
}

#[derive(Deserialize, Debug)]
struct Sidecar {
    #[serde(rename = "type")]
    sidecar_type: String,
    url: String,
}

#[derive(Deserialize, Debug)]
struct MPDResponse {
    manifestUrl: String,
    trackingUrl: String,
}

fn extract_uuid(url: &str) -> Result<String, Box<dyn Error>> {
    let re = Regex::new(r"https://www\.crackle\.com/watch/([a-f0-9\-]+)/")?;
    let captures = re.captures(url).ok_or("Failed to capture UUID")?;
    Ok(captures[1].to_string())
}

fn clean_url(url: &str) -> String {
    let re =
        Regex::new(r"(https://prod-vod-cdn1\.crackle\.com/v1/dash/[a-f0-9\-]+)(\?.*)?").unwrap();
    if let Some(caps) = re.captures(url) {
        // Keep only the base URL before the first `aws.sessionId`
        let base_url = &caps[1]; // This is the part before any query parameters
        return base_url.to_string();
    }
    url.to_string()
}

// Function to extract the AWS session ID from the URL
fn extract_aws_session_id_from_url(url: &str) -> Result<String, Box<dyn Error>> {
    let re = Regex::new(r"aws\.sessionId=([a-f0-9\-]+)")?;
    if let Some(captures) = re.captures(url) {
        Ok(captures[1].to_string())
    } else {
        Err("AWS session ID not found".into())
    }
}

fn create_final_url(mpd_url: &str, aws_session_id: &str) -> String {
    let base_url = "https://prod-vod-cdn1.crackle.com";
    let cleaned_url = clean_url(mpd_url);

    // Build the final URL with only one aws.sessionId
    format!("{}{}", base_url, cleaned_url)
}
pub(crate) async fn process_crackle_url(
    url: &str,
    drm: Option<&str>,
    filename: &str,
) -> Result<(), Box<dyn Error>> {
    let uuid = extract_uuid(url)?;

    let api_url = format!("https://prod-api.crackle.com/playback/vod/{}", uuid);

    let mut headers = HeaderMap::new();
    headers.insert(
        "User-Agent",
        "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:124.0) Gecko/20100101 Firefox/124.0"
            .parse()
            .unwrap(),
    );
    headers.insert("x-crackle-brand", "crackle".parse().unwrap());
    headers.insert(
        "x-crackle-platform",
        "5FE67CCA-069A-42C6-A20F-4B47A8054D46".parse().unwrap(),
    );
    headers.insert("x-crackle-region", "us".parse().unwrap());
    headers.insert("x-crackle-locale", "en-us".parse().unwrap());
    headers.insert("x-crackle-apiversion", "v2.0.0".parse().unwrap());

    let client: Client = Client::new();

    eprintln!("Sending request to API URL: {}", api_url);
    eprintln!("Headers: {:?}", headers);

    let res = client.get(&api_url).headers(headers.clone()).send().await?;

    let status = res.status();
    if !status.is_success() {
        eprintln!("Failed request with status: {}", status);
        let body = res.text().await?;
        eprintln!("Response body: {}", body);
        return Err(format!("Failed to fetch data: {}", status).into());
    }

    let body = res.text().await?;
    eprintln!("API Response: {}", body);

    let vod_response: VODResponse = serde_json::from_str(&body)?;

    eprintln!("Parsed VOD Response: {:?}", vod_response);

    // Handle stream selection with DRM logic
    let selected_stream = vod_response.data.streams.into_iter().find(|s| match drm {
        Some("playready") => s.stream_type.contains("playready"),
        Some("widevine") => s.stream_type.contains("widevine"),
        _ => false,
    });

    if let Some(stream) = selected_stream {
        eprintln!("Selected stream: {:?}", stream);

        let mpd_res = client
            .post(&stream.url)
            .headers(headers)
            .body("{}")
            .send()
            .await?;

        let mpd_status = mpd_res.status();
        if !mpd_status.is_success() {
            eprintln!("Failed to fetch MPD data with status: {}", mpd_status);
            let mpd_body = mpd_res.text().await?;
            eprintln!("MPD Response body: {}", mpd_body);
            return Err(format!("Failed to fetch MPD data: {}", mpd_status).into());
        }

        let mpd_body = mpd_res.text().await?;
        eprintln!("MPD Response: {}", mpd_body);

        let mpd_response: MPDResponse = serde_json::from_str(&mpd_body)?;

        eprintln!("Parsed MPD Response: {:?}", mpd_response);

        // Extract AWS session ID
        let aws_session_id = extract_aws_session_id_from_url(&mpd_response.manifestUrl)?;

        // Create the final URL
        let final_url = create_final_url(&mpd_response.manifestUrl, &aws_session_id);

        eprintln!("Final URL for download: {}", final_url);

        // Now run the subprocess to download the stream using N_m3u8DL-RE
        let mut command = vec!["N_m3u8DL-RE", &final_url];

        let status = Command::new(command[0]).args(&command[1..]).status()?;

        if !status.success() {
            eprintln!("Download failed with status: {:?}", status);
            return Err("Download failed".into());
        }

        eprintln!("Download completed successfully.");
    } else {
        eprintln!("No suitable stream found.");
    }

    Ok(())
}

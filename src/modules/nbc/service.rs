use reqwest::blocking::Client;
use reqwest::header::{HeaderMap, ACCEPT, CONTENT_TYPE, USER_AGENT};
use serde::Deserialize;
use std::error::Error;
use urlencoding::encode;

#[derive(Deserialize, Debug)]
struct FriendshipResponse {
    data: FriendshipData,
}

#[derive(Deserialize, Debug)]
struct FriendshipData {
    video: VideoData,
}

#[derive(Deserialize, Debug)]
struct VideoData {
    mpxAccountId: String,
}

#[derive(Deserialize, Debug)]
struct LemonadeResponse {
    playbackUrl: String,
    #[serde(rename = "type")]
    video_type: String,
}

pub fn fetch_video_url(video_url: &str) -> Result<String, Box<dyn Error>> {
    // Encode the video URL to ensure it's safe for GraphQL query
    let encoded_video_url = encode(video_url);

    // Construct the Friendship URL with the provided video_url (encoded)
    let friendship_url = construct_friendship_url(&encoded_video_url);

    // Create the HTTP client
    let client = Client::new();

    // Prepare the GraphQL request body with the necessary parameters
    let graphql_body = construct_graphql_body(&encoded_video_url);

    // Prepare headers
    let mut headers = HeaderMap::new();
    headers.insert(USER_AGENT, "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Safari/537.36".parse()?);
    headers.insert(CONTENT_TYPE, "application/json".parse()?);
    headers.insert(ACCEPT, "application/json".parse()?);

    // Make the request to Friendship API (first request)
    let response = client
        .post(&friendship_url)
        .headers(headers.clone()) // Clone headers
        .body(graphql_body) // Add the body with the GraphQL query
        .send()?;

    // Debugging: Log the response status and URL
    println!("Friendship API Response Status: {}", response.status());
    println!("Friendship API Response URL: {}", response.url());

    // Extract the response body
    let response_text = response.text()?;

    // Log the raw response body for debugging
    println!("Friendship API Response Body: {}", response_text);

    // Deserialize the Friendship API response
    let friendship_response: FriendshipResponse = serde_json::from_str(&response_text)?;

    // Extract the mpxAccountId from the response
    let mpx_account_id = friendship_response.data.video.mpxAccountId;

    if mpx_account_id.is_empty() {
        return Err("No mpxAccountId found.".into());
    }

    // Construct the Lemonade URL with the mpxAccountId and video_url
    let lemonade_url = construct_lemonade_url(&mpx_account_id, video_url);

    // Make the request to Lemonade API (second request)
    let lemonade_response = client
        .get(&lemonade_url)
        .headers(headers) // Use headers from before
        .send()?;

    // Debugging: Log the Lemonade response status
    println!(
        "Lemonade API Response Status: {}",
        lemonade_response.status()
    );
    println!("Lemonade API Response URL: {}", lemonade_response.url());

    // Extract the Lemonade API response text
    let lemonade_text = lemonade_response.text()?;

    // Deserialize the Lemonade API response
    let lemonade_data: LemonadeResponse = serde_json::from_str(&lemonade_text)?;

    // Return the playback URL from Lemonade response
    Ok(lemonade_data.playbackUrl)
}

fn construct_friendship_url(encoded_video_url: &str) -> String {
    let variables = format!(
        r#"{{"userId":"3185060550472549879","device":"web","platform":"web","language":"en","oneApp":true,"authorized":false,"isDayZero":true,"name":"nbc-nightly-news/video/nbc-nightly-news-1225/{}","type":"VIDEO","timeZone":"America/New_York","ld":true,"profile":["00000","11111"],"nbcAffiliateName":"wnbc","telemundoAffiliateName":"wnju","nationalBroadcastType":"eastCoast","app":"nbc","appVersion":"1236001-personalization","queryName":"bonanzaPage"}}"#,
        encoded_video_url
    );

    format!(
        "https://friendship.nbc.co/v2/graphql?variables={}",
        variables
    )
}

fn construct_graphql_body(encoded_video_url: &str) -> String {
    let variables = format!(
        r#"{{"userId":"3185060550472549879","device":"web","platform":"web","language":"en","oneApp":true,"authorized":false,"isDayZero":true,"name":"nbc-nightly-news/video/nbc-nightly-news-1225/{}","type":"VIDEO","timeZone":"America/New_York","ld":true,"profile":["00000","11111"],"nbcAffiliateName":"wnbc","telemundoAffiliateName":"wnju","nationalBroadcastType":"eastCoast","app":"nbc","appVersion":"1236001-personalization","queryName":"bonanzaPage"}}"#,
        encoded_video_url
    );

    format!(r#"{{"query":"{{ \"variables\": {} }}"#, variables)
}

fn construct_lemonade_url(mpx_account_id: &str, video_url: &str) -> String {
    format!(
        "https://lemonade.nbc.com/v1/vod/{}/{}?platform=web&browser=other&programmingType=Full%20Episode",
        mpx_account_id, video_url
    )
}

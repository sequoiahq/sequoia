use reqwest::{header, Client, Error};
use serde::Deserialize;
use std::collections::HashMap;

/*

Service code for AMC
Written by @matt

Authorization: None
Security: FHD@L3

*/


#[derive(Deserialize, Debug)]
struct AnonymousTokenResponse {
    success: bool,
    status: u16,
    data: AnonymousTokenData,
}

#[derive(Deserialize, Debug)]
struct AnonymousTokenData {
    access_token: String,
}

#[derive(Deserialize, Debug)]
struct PlaybackJsonData {
    sources: Vec<Source>,
}

#[derive(Deserialize, Debug)]
struct Source {
    key_systems: Option<HashMap<String, KeySystem>>,
    src: String,
}

#[derive(Deserialize, Debug)]
struct KeySystem {
    license_url: String,
}

#[derive(Deserialize, Debug)]
struct AMCResponse {
    data: AMCData,
}

#[derive(Deserialize, Debug)]
struct AMCData {
    playback_json_data: PlaybackJsonData,
}
pub async fn get_anonymous_token() -> Result<String, Error> {
    let client = Client::new();
    let url = "https://gw.cds.amcn.com/auth-orchestration-id/api/v1/unauth";

    let access_token = "undefined"; // IDK, have to find token yet

    let mut headers = header::HeaderMap::new();
    headers.insert(
        header::HeaderName::from_static("Accept"),
        header::HeaderValue::from_static("*/*"),
    );
    headers.insert(
        header::HeaderName::from_static("Accept-Language"),
        header::HeaderValue::from_static("es-ES,es;q=0.9,sq;q=0.8,hy;q=0.7,en;q=0.6"),
    );
    headers.insert(
        header::HeaderName::from_static("Content-Type"),
        header::HeaderValue::from_static("application/json"),
    );
    headers.insert(
        header::HeaderName::from_static("Origin"),
        header::HeaderValue::from_static("https://www.amc.com"),
    );
    headers.insert(
        header::HeaderName::from_static("Priority"),
        header::HeaderValue::from_static("u=1, i"),
    );
    headers.insert(
        header::HeaderName::from_static("Sec-CH-UA"),
        header::HeaderValue::from_static(
            "\"Chromium\";v=\"128\", \"Not;A=Brand\";v=\"24\", \"Google Chrome\";v=\"128\"",
        ),
    );
    headers.insert(
        header::HeaderName::from_static("Sec-CH-UA-Mobile"),
        header::HeaderValue::from_static("?0"),
    );
    headers.insert(
        header::HeaderName::from_static("Sec-CH-UA-Platform"),
        header::HeaderValue::from_static("\"Linux\""),
    );
    headers.insert(
        header::HeaderName::from_static("Sec-Fetch-Dest"),
        header::HeaderValue::from_static("empty"),
    );
    headers.insert(
        header::HeaderName::from_static("Sec-Fetch-Mode"),
        header::HeaderValue::from_static("cors"),
    );
    headers.insert(
        header::HeaderName::from_static("Sec-Fetch-Site"),
        header::HeaderValue::from_static("cross-site"),
    );
    headers.insert(header::HeaderName::from_static("User-Agent"), header::HeaderValue::from_static("Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/128.0.0.0 Safari/537.36"));
    headers.insert(
        header::HeaderName::from_static("X-AMCN-Adobe-ID"),
        header::HeaderValue::from_static("some_value"),
    );
    headers.insert(
        header::HeaderName::from_static("X-AMCN-App-Version"),
        header::HeaderValue::from_static("3.33.0"),
    );
    headers.insert(
        header::HeaderName::from_static("X-AMCN-Device-Ad-ID"),
        header::HeaderValue::from_static("c420f44f-606e-4026-a203-7a7ab165641c"),
    );
    headers.insert(
        header::HeaderName::from_static("X-AMCN-Device-ID"),
        header::HeaderValue::from_static("c420f44f-606e-4026-a203-7a7ab165641c"),
    );
    headers.insert(
        header::HeaderName::from_static("X-AMCN-Language"),
        header::HeaderValue::from_static("en-us"),
    );
    headers.insert(
        header::HeaderName::from_static("X-AMCN-Network"),
        header::HeaderValue::from_static("amc"),
    );
    headers.insert(
        header::HeaderName::from_static("X-AMCN-Platform"),
        header::HeaderValue::from_static("web"),
    );
    headers.insert(
        header::HeaderName::from_static("X-AMCN-Service-Group-ID"),
        header::HeaderValue::from_static("1"),
    );
    headers.insert(
        header::HeaderName::from_static("X-AMCN-Service-ID"),
        header::HeaderValue::from_static("amc"),
    );
    headers.insert(
        header::HeaderName::from_static("X-AMCN-Tenant"),
        header::HeaderValue::from_static("amcn"),
    );
    headers.insert(
        header::HeaderName::from_static("X-CCPA-Do-Not-Sell"),
        header::HeaderValue::from_static("doNotPassData"),
    );
    headers.insert(
        header::HeaderName::from_static("Authorization"),
        format!("Bearer {}", access_token).parse().unwrap(),
    );

    let response = client.post(url).headers(headers).send().await?;
    let anonymous_token_response: AnonymousTokenResponse = response.json().await?;
    Ok(anonymous_token_response.data.access_token)
}

pub async fn get_license_urls(access_token: String) -> Result<(), Error> {
    let client = Client::new();
    let url = "https://www.amc.com/api/playback/v1/playback";

    let mut headers = header::HeaderMap::new();
    headers.insert(
        header::HeaderName::from_static("Accept"),
        header::HeaderValue::from_static("*/*"),
    );
    headers.insert(
        header::HeaderName::from_static("Accept-Language"),
        header::HeaderValue::from_static("es-ES,es;q=0.9,sq;q=0.8,hy;q=0.7,en;q=0.6"),
    );
    headers.insert(
        header::HeaderName::from_static("Content-Type"),
        header::HeaderValue::from_static("application/json"),
    );
    headers.insert(
        header::HeaderName::from_static("Origin"),
        header::HeaderValue::from_static("https://www.amc.com"),
    );
    headers.insert(
        header::HeaderName::from_static("Priority"),
        header::HeaderValue::from_static("u=1, i"),
    );
    headers.insert(
        header::HeaderName::from_static("Sec-CH-UA"),
        header::HeaderValue::from_static(
            "\"Chromium\";v=\"128\", \"Not;A=Brand\";v=\"24\", \"Google Chrome\";v=\"128\"",
        ),
    );
    headers.insert(
        header::HeaderName::from_static("Sec-CH-UA-Mobile"),
        header::HeaderValue::from_static("?0"),
    );
    headers.insert(
        header::HeaderName::from_static("Sec-CH-UA-Platform"),
        header::HeaderValue::from_static("\"Linux\""),
    );
    headers.insert(
        header::HeaderName::from_static("Sec-Fetch-Dest"),
        header::HeaderValue::from_static("empty"),
    );
    headers.insert(
        header::HeaderName::from_static("Sec-Fetch-Mode"),
        header::HeaderValue::from_static("cors"),
    );
    headers.insert(
        header::HeaderName::from_static("Sec-Fetch-Site"),
        header::HeaderValue::from_static("cross-site"),
    );
    headers.insert(header::HeaderName::from_static("User-Agent"), header::HeaderValue::from_static("Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/128.0.0.0 Safari/537.36"));
    headers.insert(
        header::HeaderName::from_static("X-AMCN-Adobe-ID"),
        header::HeaderValue::from_static("some_value"),
    );
    headers.insert(
        header::HeaderName::from_static("X-AMCN-App-Version"),
        header::HeaderValue::from_static("3.33.0"),
    );
    headers.insert(
        header::HeaderName::from_static("X-AMCN-Device-Ad-ID"),
        header::HeaderValue::from_static("c420f44f-606e-4026-a203-7a7ab165641c"),
    );
    headers.insert(
        header::HeaderName::from_static("X-AMCN-Device-ID"),
        header::HeaderValue::from_static("c420f44f-606e-4026-a203-7a7ab165641c"),
    );
    headers.insert(
        header::HeaderName::from_static("X-AMCN-Language"),
        header::HeaderValue::from_static("en-us"),
    );
    headers.insert(
        header::HeaderName::from_static("X-AMCN-Network"),
        header::HeaderValue::from_static("amc"),
    );
    headers.insert(
        header::HeaderName::from_static("X-AMCN-Platform"),
        header::HeaderValue::from_static("web"),
    );
    headers.insert(
        header::HeaderName::from_static("X-AMCN-Service-Group-ID"),
        header::HeaderValue::from_static("1"),
    );
    headers.insert(
        header::HeaderName::from_static("X-AMCN-Service-ID"),
        header::HeaderValue::from_static("amc"),
    );
    headers.insert(
        header::HeaderName::from_static("X-AMCN-Tenant"),
        header::HeaderValue::from_static("amcn"),
    );
    headers.insert(
        header::HeaderName::from_static("X-CCPA-Do-Not-Sell"),
        header::HeaderValue::from_static("doNotPassData"),
    );
    headers.insert(
        header::HeaderName::from_static("Authorization"),
        format!("Bearer {}", access_token).parse().unwrap(),
    );

    let body = serde_json::json!({
        "adobeShortMediaToken": "",
        "hba": false,
        "adtags": {
            "lat": 0,
            "url": "https://www.amc.com/shows/show-me-more/episodes/season-1-inside-the-walking-dead-daryl-dixon-s2--1071134",
            "playerWidth": 1920,
            "playerHeight": 1080,
            "ppid": 1,
            "mode": "on-demand",
            "uid2": "",
        },
        "useLowResVideo": false,
    });

    let response = client.post(url).headers(headers).json(&body).send().await?;
    let amc_response: AMCResponse = response.json().await?;

    for source in amc_response.data.playback_json_data.sources {
        if let Some(key_systems) = source.key_systems {
            if let Some(widevine) = key_systems.get("com.widevine.alpha") {
                println!("Widevine License URL: {}", widevine.license_url);
            }
            if let Some(playready) = key_systems.get("com.microsoft.playready") {
                println!("PlayReady License URL: {}", playready.license_url);
            }
        }
        if source.src.contains("widevine") {
            println!("Widevine MPD URL: {}", source.src);
        } else if source.src.contains("playready") {
            println!("PlayReady MPD URL: {}", source.src);
        }
    }

    Ok(())
}

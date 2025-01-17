/*use crate::utils::logger::log_info;
use regex::Regex;
use reqwest::blocking::get;

pub fn fetch_and_decode() -> Result<String, Box<dyn std::error::Error>> {
    log_info("Start Deezer fetch and decode process.");

    // get initial explore page
    let url = "https://www.deezer.com/en/channels/explore";
    let body = get(url)?.text()?;

    // regex to extract app-web url
    let re = Regex::new(r#"script src="(https:\/\/[a-z-\.\/]+app-web[a-z0-9\.]+)""#)?;
    let app_web_url = re
        .captures(&body)
        .and_then(|caps| caps.get(1).map(|m| m.as_str().to_string()))
        .ok_or("App web URL not found")?;

    log_info(&format!("App web URL found: {}", app_web_url));

    let app_web_body = get(&app_web_url)?.text()?;

    // regex for t1 and t2
    let re_t1 = Regex::new(r#"%5B0x61(%2C0x[0-9a-z+]+)+%2C0x67%5D"#)?;
    let re_t2 = Regex::new(r#"%5B0x31(%2C0x[0-9a-z+]+)+%2C0x34%5D"#)?;

    let t1 = re_t1
        .captures(&app_web_body)
        .and_then(|caps| caps.get(0).map(|m| m.as_str().to_string()))
        .ok_or("T1 data not found")?;

    let t2 = re_t2
        .captures(&app_web_body)
        .and_then(|caps| caps.get(0).map(|m| m.as_str().to_string()))
        .ok_or("T2 data not found")?;

    let t1_decoded = decode_and_reverse(&t1)?;
    let t2_decoded = decode_and_reverse(&t2)?;

    // combine
    let mut final_string = String::new();
    for i in 0..8 {
        final_string.push(t1_decoded.chars().nth(i).unwrap());
        final_string.push(t2_decoded.chars().nth(i).unwrap());
    }

    // log_info(&format!("Final decoded string: {}", final_string));

    Ok(final_string)
}

fn decode_and_reverse(input: &str) -> Result<String, Box<dyn std::error::Error>> {
    // replace %5B, %2C, %5D, 0x in nothing
    let decoded = input
        .replace("%5B", "")
        .replace("%2C", "")
        .replace("%5D", "")
        .replace("0x", "x");

    // convert to string
    let decoded_bytes = decode_bytes(&decoded)?;

    let reversed: String = decoded_bytes.chars().rev().collect();
    Ok(reversed)
}

fn decode_bytes(input: &str) -> Result<String, Box<dyn std::error::Error>> {
    // split the input string by 'x' to process each byte
    let parts: Vec<&str> = input.split('x').filter(|&s| !s.is_empty()).collect();

    // convert each hex part to a character
    let decoded_bytes: Result<Vec<char>, _> = parts
        .iter()
        .map(|&part| {
            u8::from_str_radix(part, 16)
                .map(|byte| byte as char)
                .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        })
        .collect();

    decoded_bytes.map(|bytes| bytes.into_iter().collect())
}
*/
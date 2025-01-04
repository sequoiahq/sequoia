use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

// get cookies from netscape-like txt
pub fn get_cookies_from_netscape(cookie_file: &str) -> io::Result<HashMap<String, String>> {
    let file = File::open(cookie_file)?;
    let reader: BufReader<File> = BufReader::new(file);

    let mut cookies = HashMap::new();
    for line in reader.lines() {
        let line = line?;
        if line.starts_with('#') || line.trim().is_empty() {
            continue;
        }

        let parts: Vec<&str> = line.split('\t').collect();
        if parts.len() >= 7 {
            let cookie_name = parts[5].to_string();
            let cookie_value = parts[6].to_string();
            cookies.insert(cookie_name, cookie_value);
        }
    }

    Ok(cookies)
}

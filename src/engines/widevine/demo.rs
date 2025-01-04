use std::fs::File;
use std::io::BufReader;

use isahc::ReadResponseExt;
use hex_lit::hex;
use widevine::{Device, Cdm, Pssh, LicenseType};

fn main() {
    // init new device
    let device = Device::read_wvd(BufReader::new(File::open(&wvd_path).unwrap())).unwrap();
    let cdm = Cdm::new(device);

    /// create new CDM request
    let pssh = Pssh::from_b64("AAAAW3Bzc2gAAAAA7e+LqXnWSs6jyCfc1R0h7QAAADsIARIQ62dqu8s0Xpa7z2FmMPGj2hoNd2lkZXZpbmVfdGVzdCIQZmtqM2xqYVNkZmFsa3IzaioCSEQyAA==").unwrap();
    let request = cdm
        .open()
        .get_license_request(pssh, LicenseType::STREAMING)
        .unwrap();
    let challenge = request.challenge().unwrap();

    // Send the request to the license server
    let mut resp = isahc::post("https://cwip-shaka-proxy.appspot.com/no_auth", challenge).unwrap();
    let resp_data = resp.bytes().unwrap();

    // Decrypt the received keys and select the key with the required ID
    let keys = request.get_keys(&resp_data).unwrap();
    let key = keys.content_key(&hex!("ccbf5fb4c2965be7aa130ffb3ba9fd73")).unwrap();

    assert_eq!(key.key, hex!("9cc0c92044cb1d69433f5f5839a159df"));
}

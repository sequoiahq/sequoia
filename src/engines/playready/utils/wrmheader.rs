use std::error::Error;
use std::fmt;

use crate::key::Key;
use crate::pssh::PSSH;

#[derive(Debug)]
pub struct WRMHeader {
    pub header: String,
    pub version: String,
    pub data: Vec<u8>,
}

impl WRMHeader {
    pub fn new(header: String) -> Self {
        let mut version = String::new();
        let mut data = Vec::new();
        let mut offset = 0;
        while offset < header.len() {
            let length =
                u16::from_le_bytes([header.as_bytes()[offset], header.as_bytes()[offset + 1]]);
            let data_part = header.as_bytes()[offset + 2..offset + 2 + length as usize].to_vec();
            if offset == 0 {
                version = String::from_utf8_lossy(&data_part).into_owned();
            } else {
                data.extend_from_slice(&data_part);
            }
            offset += 2 + length as usize;
        }
        WRMHeader {
            header,
            version,
            data,
        }
    }

    pub fn read_attributes(&self) -> (Vec<Key>, String, String, String) {
        let mut keys = Vec::new();
        let mut la_url = String::new();
        let mut lui_url = String::new();
        let mut ds_id = String::new();
        let mut offset = 0;
        while offset < self.data.len() {
            let length = u16::from_le_bytes([self.data[offset], self.data[offset + 1]]);
            let data = self.data[offset + 2..offset + 2 + length as usize].to_vec();
            match offset {
                0 => {
                    let key_id = String::from_utf8_lossy(&data).into_owned();
                    let key_type = 0x0001; // AES128CTR
                    let cipher_type = 0x0001; // RSA128
                    let key_length = 16;
                    let key = vec![0; key_length];
                    keys.push(Key::new(key_id, key_type, cipher_type, key_length, key));
                }
                1 => {
                    la_url = String::from_utf8_lossy(&data).into_owned();
                }
                2 => {
                    lui_url = String::from_utf8_lossy(&data).into_owned();
                }
                3 => {
                    ds_id = String::from_utf8_lossy(&data).into_owned();
                }
                _ => {}
            }
            offset += 2 + length as usize;
        }
        (keys, la_url, lui_url, ds_id)
    }

    pub fn to_v4_0_0_0(&self) -> String {
        let (keys, la_url, lui_url, ds_id) = self.read_attributes();
        let key_id = keys[0].key_id.clone();
        let mut header = String::new();
        header.push_str("<MHEADER xmlns=\"http://schemas.microsoft.com/DRM/2007/03/PlayReadyHeader\" version=\"4.0.0.0\">");
        header.push_str("<DATA>");
        header.push_str("<PROTECTINFO>");
        header.push_str("<KEYLEN>16</KEYLEN>");
        header.push_str("<ALGID>AESCTR</ALGID>");
        header.push_str("</PROTECTINFO>");
        header.push_str("<KID>");
        header.push_str(&key_id);
        header.push_str("</KID>");
        if !la_url.is_empty() {
            header.push_str("<LA_URL>");
            header.push_str(&la_url);
            header.push_str("</LA_URL>");
        }
        if !lui_url.is_empty() {
            header.push_str("<LUI_URL>");
            header.push_str(&lui_url);
            header.push_str("</LUI_URL>");
        }
        if !ds_id.is_empty() {
            header.push_str("<DS_ID>");
            header.push_str(&ds_id);
            header.push_str("</DS_ID>");
        }
        header.push_str("</DATA>");
        header.push_str("</MHEADER>");
        header
    }
}

impl fmt::Display for WRMHeader {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "WRMHeader {{ header: {:?}, version: {:?}, data: {:?} }}",
            self.header, self.version, self.data
        )
    }
}

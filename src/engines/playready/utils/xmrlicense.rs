use std::error::Error;
use std::fmt;

use crate::key::Key;

#[derive(Debug)]
pub struct XmrLicense {
    pub header: Vec<u8>,
    pub content_keys: Vec<Key>,
}

impl XmrLicense {
    pub fn loads(data: Vec<u8>) -> Result<Self, Box<dyn Error>> {
        let mut offset = 0;
        let header_length = u16::from_le_bytes([data[offset], data[offset + 1]]);
        let header = data[offset + 2..offset + 2 + header_length as usize].to_vec();
        offset += 2 + header_length as usize;
        let mut content_keys = Vec::new();
        while offset < data.len() {
            let length = u16::from_le_bytes([data[offset], data[offset + 1]]);
            let data = data[offset + 2..offset + 2 + length as usize].to_vec();
            let key_id = String::from_utf8_lossy(&data[0..16]).into_owned();
            let key_type = 0x0001; // AES128CTR
            let cipher_type = 0x0001; // RSA128
            let key_length = 16;
            let key = data[16..16 + key_length].to_vec();
            content_keys.push(Key::new(key_id, key_type, cipher_type, key_length, key));
            offset += 2 + length as usize;
        }
        Ok(XmrLicense { header, content_keys })
    }

    pub fn get_content_keys(&self) -> impl Iterator<Item = Key> + '_ {
        self.content_keys.iter().cloned()
    }
}

impl fmt::Display for XmrLicense {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "XmrLicense {{ header: {:?}, content_keys: {:?} }}", self.header, self.content_keys)
    }
}
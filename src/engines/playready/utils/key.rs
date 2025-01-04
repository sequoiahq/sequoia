use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum KeyType {
    Invalid = 0x0000,
    AES128CTR = 0x0001,
    RC4 = 0x0002,
    AES128ECB = 0x0003,
    Cocktail = 0x0004,
    AESCBC = 0x0005,
    UNKNOWN = 0xffff,
}

#[derive(Debug)]
pub enum CipherType {
    Invalid = 0x0000,
    RSA128 = 0x0001,
    ChainedLicense = 0x0002,
    ECC256 = 0x0003,
    ECCforScalableLicenses = 0x0004,
    Scalable = 0x0005,
    UNKNOWN = 0xffff,
}

#[derive(Debug)]
pub struct Key {
    pub key_id: String,
    pub key_type: KeyType,
    pub cipher_type: CipherType,
    pub key_length: u32,
    pub key: Vec<u8>,
}

impl Key {
    pub fn new(key_id: String, key_type: u32, cipher_type: u32, key_length: u32, key: Vec<u8>) -> Self {
        Key {
            key_id,
            key_type: match key_type {
                0x0001 => KeyType::AES128CTR,
                0x0002 => KeyType::RC4,
                0x0003 => KeyType::AES128ECB,
                0x0004 => KeyType::Cocktail,
                0x0005 => KeyType::AESCBC,
                _ => KeyType::UNKNOWN,
            },
            cipher_type: match cipher_type {
                0x0001 => CipherType::RSA128,
                0x0002 => CipherType::ChainedLicense,
                0x0003 => CipherType::ECC256,
                0x0004 => CipherType::ECCforScalableLicenses,
                0x0005 => CipherType::Scalable,
                _ => CipherType::UNKNOWN,
            },
            key_length,
            key,
        }
    }
}

impl fmt::Display for Key {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Key {{ key_id: {:?}, key_type: {:?}, cipher_type: {:?}, key_length: {:?}, key: {:?} }}", self.key_id, self.key_type, self.cipher_type, self.key_length, self.key)
    }
}
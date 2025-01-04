use std::error::Error;
use std::fmt;

use elliptic::ec::Secp256r1;
use elliptic::key::SecretKey;
use elliptic::sec1::ToEncodedPoint;

#[derive(Debug)]
pub struct EccKey {
    key_pair: SecretKey<Secp256r1>,
}

impl EccKey {
    pub fn generate() -> Self {
        let key_pair = SecretKey::new_random();
        EccKey { key_pair }
    }

    pub fn loads(data: Vec<u8>) -> Result<Self, Box<dyn Error>> {
        let key_pair = SecretKey::from_bytes(&data)?;
        Ok(EccKey { key_pair })
    }

    pub fn dumps(&self) -> Vec<u8> {
        self.key_pair.to_bytes().to_vec()
    }
}

impl fmt::Display for EccKey {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "EccKey {{ key_pair: {:?} }}", self.key_pair)
    }
}
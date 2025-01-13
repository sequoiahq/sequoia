use std::error::Error;
use std::fmt;

use elliptic::ec::Secp256r1;
use elliptic::key::SecretKey;
use elliptic::sec1::ToEncodedPoint;

#[derive(Debug)]
pub struct EccKey {
    // elliptic curve keypair
    key_pair: SecretKey<Secp256r1>,
}

impl EccKey {

    // generate random EccKey instance
    pub fn generate() -> Self {
        let key_pair = SecretKey::new_random();
        EccKey { key_pair }
    }

    // load instance from byte array
    pub fn loads(data: Vec<u8>) -> Result<Self, Box<dyn Error>> {
        
        // parse bytearray into SecretKey
        let key_pair = SecretKey::from_bytes(&data)?;
        Ok(EccKey { key_pair })
    }

    // dump instance to byte array 
    pub fn dumps(&self) -> Vec<u8> {
        // keypair to bytes
        self.key_pair.to_bytes().to_vec()
    }
}

impl fmt::Display for EccKey {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "EccKey {{ key_pair: {:?} }}", self.key_pair)
    }
}

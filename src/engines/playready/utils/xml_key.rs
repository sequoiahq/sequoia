use std::error::Error;
use std::fmt;

use crate::ecc_key::EccKey;
use crate::elgamal::ElGamal;

#[derive(Debug)]
pub struct XmlKey {
    pub shared_point: EccKey,
    pub shared_key_x: Vec<u8>,
    pub shared_key_y: Vec<u8>,
    pub aes_iv: Vec<u8>,
    pub aes_key: Vec<u8>,
}

impl XmlKey {
    pub fn new() -> Self {
        let shared_point = EccKey::generate();
        let shared_key_x = shared_point.dumps()[0..16].to_vec();
        let shared_key_y = shared_point.dumps()[16..32].to_vec();
        let aes_iv = shared_key_x.clone();
        let aes_key = shared_key_y.clone();
        XmlKey {
            shared_point,
            shared_key_x,
            shared_key_y,
            aes_iv,
            aes_key,
        }
    }

    pub fn get_point(&self, curve: ElGamal) -> (Vec<u8>, Vec<u8>) {
        (self.shared_key_x.clone(), self.shared_key_y.clone())
    }
}

impl fmt::Display for XmlKey {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "XmlKey {{ shared_point: {:?}, shared_key_x: {:?}, shared_key_y: {:?}, aes_iv: {:?}, aes_key: {:?} }}", self.shared_point, self.shared_key_x, self.shared_key_y, self.aes_iv, self.aes_key)
    }
}

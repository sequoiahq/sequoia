use std::error::Error;
use std::fmt;

use elliptic::ec::Secp256r1;
use elliptic::key::SecretKey;
use elliptic::sec1::ToEncodedPoint;

#[derive(Debug)]
pub struct ElGamal {
    curve: Secp256r1,
}

impl ElGamal {
    pub fn new() -> Self {
        ElGamal {
            curve: Secp256r1,
        }
    }

    pub fn encrypt(&self, message: &str, public_key: &SecretKey<Secp256r1>) -> (Vec<u8>, Vec<u8>) {
        let message_point = self.curve.point_from_bytes(message.as_bytes()).unwrap();
        let ephemeral_key = SecretKey::new_random();
        let point1 = self.curve.g * ephemeral_key;
        let point2 = message_point + public_key * ephemeral_key;
        (point1.to_bytes().to_vec(), point2.to_bytes().to_vec())
    }

    pub fn decrypt(&self, encrypted: (Vec<u8>, Vec<u8>), private_key: &SecretKey<Secp256r1>) -> Vec<u8> {
        let point1 = self.curve.point_from_bytes(&encrypted.0).unwrap();
        let point2 = self.curve.point_from_bytes(&encrypted.1).unwrap();
        let shared_secret = point1 * private_key;
        let decrypted_message = point2 - shared_secret;
        decrypted_message.to_bytes().to_vec()
    }
}

impl fmt::Display for ElGamal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ElGamal {{ curve: {:?} }}", self.curve)
    }
}
use std::collections::HashMap;
use std::error::Error;
use std::fmt;

use crate::device::Device;
use crate::ecc_key::EccKey;
use crate::elgamal::ElGamal;
use crate::key::Key;
use crate::pssh::PSSH;
use crate::wrmheader::WRMHeader;
use crate::xml_key::XmlKey;

#[derive(Debug)]
pub struct Cdm {
    pub encryption_key: EccKey,
}

impl Cdm {
    pub fn from_device(device: Device) -> Self {
        Cdm {
            encryption_key: device.encryption_key.clone(),
        }
    }

    pub fn get_license_challenge(&self, wrm_header: String) -> Vec<u8> {
        let wrm_header = WRMHeader::new(wrm_header);
        let key_id = wrm_header.read_attributes().0[0].value.clone();
        let mut challenge = Vec::new();
        challenge.extend_from_slice(key_id.as_bytes());
        challenge
    }

    pub fn parse_license(&self, license: Vec<u8>) -> Vec<Key> {
        let xmr_license = XmrLicense::loads(license);
        let keys = xmr_license.get_content_keys().collect::<Vec<_>>();
        keys.into_iter()
            .map(|key| {
                Key::new(
                    key.key_id,
                    key.key_type,
                    key.cipher_type,
                    key.key_length,
                    key.key,
                )
            })
            .collect()
    }
}

use std::collections::VecDeque;
use std::error::Error;
use std::fmt;

use crate::bcert::CertificateChain;
use crate::ecc_key::EccKey;

#[derive(Debug)]
pub struct Device {
    pub group_certificate: CertificateChain,
    pub encryption_key: EccKey,
    pub signing_key: EccKey,
    pub security_level: u32,
}

impl Device {
    pub fn loads(data: Vec<u8>) -> Result<Self, Box<dyn Error>> {
        let mut offset = 0;
        let signature = &data[offset..offset + 3];
        offset += 3;
        let version = u8::from_le_bytes([data[offset]]);
        offset += 1;
        let group_certificate_length = u32::from_le_bytes([data[offset + 0], data[offset + 1], data[offset + 2], data[offset + 3]]) as usize;
        offset += 4;
        let group_certificate = CertificateChain::loads(data[offset..offset + group_certificate_length].to_vec())?;
        offset += group_certificate_length;
        let encryption_key = EccKey::loads(data[offset..offset + 96].to_vec())?;
        offset += 96;
        let signing_key = EccKey::loads(data[offset..offset + 96].to_vec())?;
        let security_level = group_certificate.get_security_level();
        Ok(Device {
            group_certificate,
            encryption_key,
            signing_key,
            security_level,
        })
    }

    pub fn load(file_path: &str) -> Result<Self, Box<dyn Error>> {
        let mut file = std::fs::File::open(file_path)?;
        let mut data = Vec::new();
        file.read_to_end(&mut data)?;
        Device::loads(data)
    }

    pub fn dumps(&self) -> Vec<u8> {
        let mut data = Vec::new();
        data.extend_from_slice(&self.group_certificate.dumps());
        data.extend_from_slice(&self.encryption_key.dumps());
        data.extend_from_slice(&self.signing_key.dumps());
        data
    }

    pub fn dump(&self, file_path: &str) -> Result<(), Box<dyn Error>> {
        let mut file = std::fs::File::create(file_path)?;
        file.write_all(&self.dumps())?;
        Ok(())
    }

    pub fn get_name(&self) -> String {
        format!("{}_sl{}", self.group_certificate, self.security_level)
    }
}

impl fmt::Display for Device {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Device {{ group_certificate: {:?}, encryption_key: {:?}, signing_key: {:?}, security_level: {:?} }}", self.group_certificate, self.encryption_key, self.signing_key, self.security_level)
    }
}
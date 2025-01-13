use std::collections::VecDeque;
use std::error::Error;
use std::fmt;

use crate::bcert::CertificateChain;
use crate::ecc_key::EccKey;

#[derive(Debug)]
pub struct Device {
    pub group_certificate: CertificateChain, // bgroupcert
    pub encryption_key: EccKey, // encryption key
    pub signing_key: EccKey, // zgpriv
    pub security_level: u32, // level (possible options -> sl150-sl2000-sl3000)
}

impl Device {

    // load device instance from byte array
    pub fn loads(data: Vec<u8>) -> Result<Self, Box<dyn Error>> {
        let mut offset = 0;

        // skip 3byte signature
        let signature = &data[offset..offset + 3];
        offset += 3;

        // 1byte version
        let version = u8::from_le_bytes([data[offset]]);
        offset += 1;

        // length of groupcert (4bytes)
        let group_certificate_length = u32::from_le_bytes([data[offset + 0], data[offset + 1], data[offset + 2], data[offset + 3]]) as usize;
        offset += 4;

        // load groupcert chain
        let group_certificate = CertificateChain::loads(data[offset..offset + group_certificate_length].to_vec())?;
        offset += group_certificate_length;

        // load both encryption and signing key (zgpriv.dat)
        // each of them is 96bytes long.
        // it's worth mentioning that most devices have a 32byte signing key and changing the
        // encoding would convert them to 96bytes. 
        // also, when exporting a device from a prd definition you get a 96byte-converted key
        // instead of the original 32 bytes signing key. 
        
        // don't confuse signing key and encryption key. i've written about how playready works on
        // the docs (check /docs/playready/definition.md on the monorepo), but i'm gonna outline it
        // briefly.
        // signing key = zgpriv.dat
        // when you provision a device (bgroupcert+zgpriv.dat=bdevcert.dat) you use the signing key
        // to sign the certificate. when a provider un-whitelists the certificate, they're
        // blacklisting the provision, not the certificate itself, so the bdevcert.dat isn't gonna
        // work, but if you provision the certificate again, you'll have ANOTHER different
        // bdevcert.dat which is in fact whitelisted by the provider.
        
        let encryption_key = EccKey::loads(data[offset..offset + 96].to_vec())?;
        offset += 96;

        let signing_key = EccKey::loads(data[offset..offset + 96].to_vec())?;

        // determine security level
        let security_level = group_certificate.get_security_level();
        Ok(Device {
            group_certificate,
            encryption_key,
            signing_key,
            security_level,
        })
    }

    // load Device instance from prd file
    pub fn load(file_path: &str) -> Result<Self, Box<dyn Error>> {
        let mut file = std::fs::File::open(file_path)?;
        let mut data = Vec::new();
        file.read_to_end(&mut data)?;
        Device::loads(data)
    }

    // dumps Device instance to byte array
    pub fn dumps(&self) -> Vec<u8> {
        let mut data = Vec::new();
        // append groupcert chain, encryption key and signing key
        data.extend_from_slice(&self.group_certificate.dumps());
        data.extend_from_slice(&self.encryption_key.dumps());
        data.extend_from_slice(&self.signing_key.dumps());
        data
    }

    // dumps instance as well
    pub fn dump(&self, file_path: &str) -> Result<(), Box<dyn Error>> {
        let mut file = std::fs::File::create(file_path)?;
        file.write_all(&self.dumps())?;
        Ok(())
    }

    // returns name for the certificate. (MODEL_slLEVEL)
    pub fn get_name(&self) -> String {
        format!("{}_sl{}", self.group_certificate, self.security_level) 
            // e.g samsung_gt-n8000-eur-xx_gt-n8000_sl2000
     }
    }
}

impl fmt::Display for Device {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Device {{ group_certificate: {:?}, encryption_key: {:?}, signing_key: {:?}, security_level: {:?} }}", self.group_certificate, self.encryption_key, self.signing_key, self.security_level)
    }
}

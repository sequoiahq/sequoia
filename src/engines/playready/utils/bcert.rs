use std::collections::VecDeque;
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct Certificate {
    pub data: Vec<u8>,
}

impl Certificate {
    pub fn new(data: Vec<u8>) -> Self {
        Certificate { data }
    }
}

#[derive(Debug)]
pub struct CertificateChain {
    certificates: VecDeque<Certificate>,
}

impl CertificateChain {
    pub fn loads(data: Vec<u8>) -> Result<Self, Box<dyn Error>> {
        let mut certificates = VecDeque::new();
        let mut offset = 0;
        while offset < data.len() {
            let length = u32::from_le_bytes([
                data[offset + 0],
                data[offset + 1],
                data[offset + 2],
                data[offset + 3],
            ]) as usize;
            let certificate = Certificate::new(data[offset + 4..offset + 4 + length].to_vec());
            certificates.push_back(certificate);
            offset += 4 + length;
        }
        Ok(CertificateChain { certificates })
    }

    pub fn get_security_level(&self) -> u32 {
        self.certificates.back().unwrap().data[0] as u32
    }

    pub fn prepend(&mut self, certificate: Certificate) {
        self.certificates.push_front(certificate);
    }
}

impl fmt::Display for CertificateChain {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "CertificateChain {{ certificates: {:?} }}",
            self.certificates
        )
    }
}

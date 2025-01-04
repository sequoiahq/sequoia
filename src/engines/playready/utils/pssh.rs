use std::error::Error;
use std::fmt;

use crate::wrmheader::WRMHeader;

#[derive(Debug)]
pub struct PSSH {
    pub wrm_headers: Vec<String>,
}

impl PSSH {
    pub fn new(data: Vec<u8>) -> Self {
        let mut wrm_headers = Vec::new();
        let pssh_box = PSSHBox::parse(data).unwrap();
        for record in pssh_box.records {
            if let Some(wrm_header) = record.data {
                wrm_headers.push(wrm_header);
            }
        }
        PSSH { wrm_headers }
    }

    pub fn get_wrm_headers(&self, downgrade_to_v4: bool) -> Vec<String> {
        if downgrade_to_v4 {
            self.wrm_headers.iter().map(|header| WRMHeader::new(header.clone()).to_v4_0_0_0()).collect()
        } else {
            self.wrm_headers.clone()
        }
    }
}

impl fmt::Display for PSSH {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "PSSH {{ wrm_headers: {:?} }}", self.wrm_headers)
    }
}

#[derive(Debug)]
pub struct PSSHBox {
    pub length: u32,
    pub pssh: Vec<u8>,
    pub fullbox: u32,
    pub system_id: Vec<u8>,
    pub data_length: u32,
    pub data: Vec<u8>,
    pub records: Vec<Record>,
}

impl PSSHBox {
    pub fn parse(data: Vec<u8>) -> Result<Self, Box<dyn Error>> {
        let length = u32::from_le_bytes([data[0], data[1], data[2], data[3]]);
        let pssh = data[4..8].to_vec();
        let fullbox = u32::from_le_bytes([data[8], data[9], data[10], data[11]]);
        let system_id = data[12..28].to_vec();
        let data_length = u32::from_le_bytes([data[28], data[29], data[30], data[31]]);
        let data = data[32..32 + data_length as usize].to_vec();
        let mut records = Vec::new();
        let mut offset = 0;
        while offset < data.len() {
            let length = u16::from_le_bytes([data[offset], data[offset + 1]]);
            let data = data[offset + 2..offset + 2 + length as usize].to_vec();
            records.push(Record { length, data });
            offset += 2 + length as usize;
        }
        Ok(PSSHBox {
            length,
            pssh,
            fullbox,
            system_id,
            data_length,
            data,
            records,
        })
    }
}

impl fmt::Display for PSSHBox {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "PSSHBox {{ length: {:?}, pssh: {:?}, fullbox: {:?}, system_id: {:?}, data_length: {:?}, data: {:?}, records: {:?} }}", self.length, self.pssh, self.fullbox, self.system_id, self.data_length, self.data, self.records)
    }
}

#[derive(Debug)]
pub struct Record {
    pub length: u16,
    pub data: Vec<u8>,
}

impl fmt::Display for Record {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Record {{ length: {:?}, data: {:?} }}", self.length, self.data)
    }
}
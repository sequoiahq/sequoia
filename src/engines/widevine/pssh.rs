use std::io::{Cursor, Read};
use byteorder::{BigEndian, ReadBytesExt};
use protobuf::Message;

use crate::Error;
use widevine_proto::license_protocol::WidevinePsshData;

/*
 represents a PSSH (Protection System Specific Header).
 this contains initialization data and key identifiers for the content
 example widevine PSSH (base64 encoded): 
`AAAAMnBzc2gAAAAA7e+LqXnWSs6jyCfc1R0h7QAAABISEExI4U7xdBB4q3Cj9xmJSsE=`
*/
#[derive(Debug, Clone, PartialEq, Eq)]
#[allow(missing_docs)]
#[non_exhaustive]
pub struct Pssh {
    /// init data, normally serialized in PSSH box/header
    pub init_data: Vec<u8>,
    /// a list of KIDs (key ids, 16 bytes each) used for decryption
    pub key_ids: Vec<[u8; 16]>,
}

impl Pssh {
    // parses a base64-encoded PSSH string into a pssh structure
    pub fn from_b64(pssh: &str) -> Result<Self, Error> {
        let pssh_bts = data_encoding::BASE64
            .decode(pssh.as_bytes())
            .map_err(|e| Error::InvalidInput(format!("base64: {e}").into()))?;
        Self::from_bytes(&pssh_bts)
    }

    // creates a pssh object from raw bytes (either a pssh box or a cenc header).
    pub fn from_bytes(pssh: &[u8]) -> Result<Self, Error> {
        // parse as an mp4 pssh box.
        if let Some(res) = Self::try_parse_box(pssh) {
            return Ok(res);
        }

        // parse as a WidevinePsshData protobuf message.
        let pssh_data = WidevinePsshData::parse_from_bytes(pssh)?;
        let pssh_serialized = pssh_data.write_to_bytes()?;
        
        // validate if matches (serialized data and input)
        if pssh != pssh_serialized {
            return Err(Error::InvalidInput("could not decode PSSH data".into()));
        }

        // extract key ids from parsed data.
        let key_ids = pssh_data
            .key_ids
            .into_iter()
            .map(|key| key.try_into())
            .collect::<Result<_, _>>()
            .map_err(|_| Error::InvalidInput("unexpected key_id length".into()))?;

        Ok(Pssh {
            init_data: pssh_serialized,
            key_ids,
        })
    }

    /// parse pssh data as mp4 pssh box.
    fn try_parse_box(pssh: &[u8]) -> Option<Self> {
        let mut rdr = Cursor::new(pssh);

        // read and validate box size.
        let size = rdr.read_u32::<BigEndian>().ok()?;
        if pssh.len() != size as usize {
            return None;
        }

        // read and validate box header.
        let mut box_header = [0u8; 4];
        rdr.read_exact(&mut box_header).ok()?;
        if &box_header != b"pssh" {
            return None;
        }

        // extract version and flags.
        let version_and_flags = rdr.read_u32::<BigEndian>().ok()?;
        let version: u8 = (version_and_flags >> 24).try_into().ok()?;
        if version > 1 {
            return None;
        }

        /* 
        there're specific system ids for every individual drm.
        examples of system ids of popular drms:
        
        PlayReady: 9a04f079-9840-4286-ab92-e65be0885f95
        Widevine: edef8ba9-79d6-4ace-a3c8-27dcd51d21ed
        Fairplay: 94ce86fb-07ff-4f43-adb8-93d2fa968ca2
        ClearKey AES-128: 3ea8778f-7742-4bf9-b18b-e834b2acbd47	
        ClearKey Sample AES: be58615b-19c4-4684-88b3-c8c57e99e957

        you can check all system ids on the dashif reference page
        https://dashif.org/identifiers/content_protection/
        */

        // validate system id
        let mut system_id = [0u8; 16];
        rdr.read_exact(&mut system_id).ok()?;
        if system_id != [
            0xed, 0xef, 0x8b, 0xa9, 0x79, 0xd6, 0x4a, 0xce, 0xa3, 0xc8, 0x27, 0xdc, 0xd5, 0x1d,
            0x21, 0xed,
        ] {
            return None;
        }

        // extract KIDs if version is 1
        let mut key_ids = Vec::new();
        if version == 1 {
            let kid_count = rdr.read_u32::<BigEndian>().ok()?;
            for _ in 0..kid_count {
                let mut key_id = [0u8; 16];
                rdr.read_exact(&mut key_id).ok()?;
                key_ids.push(key_id);
            }
        }

        // read init data
        let init_data_len = rdr.read_u32::<BigEndian>().ok()?;
        let mut init_data = Vec::new();
        rdr.take(init_data_len.into())
            .read_to_end(&mut init_data)
            .ok();

        Some(Self { init_data, key_ids })
    }
}

impl TryFrom<&[u8]> for Pssh {
    type Error = Error;

    // convert byte slice into pssh object
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Self::from_bytes(value)
    }
}


// tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_pssh() {
        let pssh = Pssh::from_b64("AAAAW3Bzc2gAAAAA7e+LqXnWSs6jyCfc1R0h7QAAADsIARIQ62dqu8s0Xpa7z2FmMPGj2hoNd2lkZXZpbmVfdGVzdCIQZmtqM2xqYVNkZmFsa3IzaioCSEQyAA==").unwrap();
        assert_eq!(
            pssh.init_data,
            [
                8, 1, 18, 16, 235, 103, 106, 187, 203, 52, 94, 150, 187, 207, 97, 102, 48, 241,
                163, 218, 26, 13, 119, 105, 100, 101, 118, 105, 110, 101, 95, 116, 101, 115, 116,
                34, 16, 102, 107, 106, 51, 108, 106, 97, 83, 100, 102, 97, 108, 107, 114, 51, 106,
                42, 2, 72, 68, 50, 0,
            ]
        );
        assert!(pssh.key_ids.is_empty());
    }
}

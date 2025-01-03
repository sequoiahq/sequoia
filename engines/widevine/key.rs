use std::fmt::Write;

use aes::cipher::block_padding::Pkcs7;
use aes::cipher::{BlockDecryptMut, KeyIvInit};
use aes::Aes128;
use cbc::Decryptor;

use crate::Error;
use widevine_proto::license_protocol::license::{
    key_container::KeyType as PbKeyType, KeyContainer,
};

/// set of widevine keys
#[derive(Debug, Clone)]
pub struct KeySet(Vec<Key>);

/// key
#[derive(Clone)]
pub struct Key {
    /// type
    pub typ: KeyType,
    /// id
    pub kid: [u8; 16],
    /// key itself
    pub key: Vec<u8>,
}

impl std::fmt::Debug for Key {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{:?}] ", self.typ)?;
        data_encoding::HEXLOWER.encode_write(&self.kid, f)?;
        f.write_char(':')?;
        data_encoding::HEXLOWER.encode_write(&self.key, f)?;
        Ok(())
    }
}

/// key type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub enum KeyType {
    // signing requests/responses
    SIGNING = 1,
    // decrypting content
    CONTENT = 2,
    // control block for license renewals. No key.
    KEY_CONTROL = 3,
    // wrapped keys for auxiliary crypto operations
    OPERATOR_SESSION = 4,
    // entitlement keys
    ENTITLEMENT = 5,
    // partner specific content key
    OEM_CONTENT = 6,
}

impl From<PbKeyType> for KeyType {
    fn from(value: PbKeyType) -> Self {
        match value {
            PbKeyType::SIGNING => Self::SIGNING,
            PbKeyType::CONTENT => Self::CONTENT,
            PbKeyType::KEY_CONTROL => Self::KEY_CONTROL,
            PbKeyType::OPERATOR_SESSION => Self::OPERATOR_SESSION,
            PbKeyType::ENTITLEMENT => Self::ENTITLEMENT,
            PbKeyType::OEM_CONTENT => Self::OEM_CONTENT,
        }
    }
}

impl KeySet {
    pub(crate) fn from_key_container(
        container: Vec<KeyContainer>,
        enc_key: &[u8; 16],
    ) -> Result<Self, Error> {
        Ok(Self(
            container
                .into_iter()
                .filter_map(|c| Key::from_key_container(c, enc_key).ok())
                .collect(),
        ))
    }

    // returns iterator providing keys of specific type
    pub fn of_type(&self, typ: KeyType) -> impl Iterator<Item = &'_ Key> {
        self.0.iter().filter(move |key| key.typ == typ)
    }

    // get first key of given type
    pub fn first_of_type(&self, typ: KeyType) -> Result<&'_ Key, Error> {
        self.0
            .iter()
            .find(|key| key.typ == typ)
            .ok_or_else(|| Error::InvalidLicense(format!("did not receive {typ:?} key").into()))
    }

    // get content key with given id
    pub fn content_key(&self, id: &[u8]) -> Result<&'_ Key, Error> {
        self.0
            .iter()
            .find(|key| key.typ == KeyType::CONTENT && key.kid == id)
            .ok_or_else(|| {
                Error::InvalidLicense(
                    format!("did not receive key {}", data_encoding::HEXLOWER.encode(id)).into(),
                )
            })
    }
}

impl Key {
    pub(crate) fn from_key_container(
        mut container: KeyContainer,
        enc_key: &[u8; 16],
    ) -> Result<Self, Error> {
        if container.id().len() > 16 {
            return Err(Error::InvalidLicense(
                "Key ID is longer than 16 bytes".into(),
            ));
        }
        let mut kid_vec = container.id.take().unwrap_or_default();
        kid_vec.resize(16, 0);
        let kid: [u8; 16] = kid_vec.try_into().unwrap();

        let iv: [u8; 16] = container
            .iv()
            .try_into()
            .map_err(|_| Error::InvalidLicense("Key IV has unexpected length".into()))?;
        let dec = Decryptor::<Aes128>::new(enc_key.into(), &iv.into());
        let key = dec
            .decrypt_padded_vec_mut::<Pkcs7>(container.key())
            .map_err(|_| Error::InvalidLicense("Padding error decrypting key".into()))?;

        Ok(Self {
            typ: container.type_().into(),
            kid,
            key,
        })
    }
}
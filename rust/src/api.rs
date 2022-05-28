use aes::cipher::{block_padding::Pkcs7, BlockDecryptMut, BlockEncryptMut, KeyIvInit};
use anyhow::Result;
use std::sync::Arc;

pub fn greet() -> String {
    "Hello from Rust! 🦀".into()
}

pub struct Cryptor {
    key: Box<[u8]>,
    iv: Box<[u8]>
}

impl Cryptor {
    pub fn new(key: Vec<u8>, iv_length: usize) -> Cryptor {
        let iv = vec![0x24; iv_length];

        Cryptor {
            key: key.into_boxed_slice(),
            iv: iv.into_boxed_slice(),
        }
    }

    fn encryptor(&self) -> cbc::Encryptor::<aes::Aes128> {
        cbc::Encryptor::<aes::Aes128>::new((*self.key).into(), (*self.iv).into())
    }

    fn decryptor(&self) -> cbc::Decryptor::<aes::Aes128> {
        cbc::Decryptor::<aes::Aes128>::new((*self.key).into(), (*self.iv).into())
    }

    pub fn encrypt(& self, text: &str) -> Vec<u8> {
        let text_len = text.len();
        let len = ((text_len / 16) + 1) * 16;
        let mut buf = vec![0x0u8; len];
        buf[..text_len].copy_from_slice(text.as_bytes());
        let enc = self.encryptor();
        let ct = enc.encrypt_padded_mut::<Pkcs7>(&mut buf, text_len)
                .unwrap();
        Vec::from(ct)
    }

    pub fn decrypt(& self, value: &[u8]) -> String {
        let mut buf = Vec::from(value);
        let enc = self.decryptor();
        let ct = enc.decrypt_padded_mut::<Pkcs7>(&mut buf)
                .unwrap();
        String::from_utf8(Vec::from(ct)).unwrap()
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct CryptorHandle(pub i64, pub i64);

impl_pool_object_handle!(Arc<Cryptor>, CryptorHandle);

pub fn cryptor_new(sub_pool_id: i64, key: Vec<u8>, iv_length: usize) -> Result<CryptorHandle> {
    Ok(pool::put(sub_pool_id, Arc::new(Cryptor::new(key, iv_length))))
}

pub fn cryptor_encrpyt(cryptor: CryptorHandle, text: String) -> Result<Vec<u8>> {
    let mut cryptor = pool::get_cloned(cryptor)?;
    let result = cryptor.encrypt(text);
    Ok(result)
}

pub fn cryptor_remove(cryptor: CryptorHandle) -> Result<()> {
    pool::remove(cryptor);
    Ok(())
}

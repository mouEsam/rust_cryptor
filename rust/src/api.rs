use std::cell::{Ref, RefCell};
use aes::cipher::{block_padding::Pkcs7, BlockDecryptMut, BlockEncryptMut, KeyIvInit};
use std::sync::{Arc, Mutex};
use anyhow::{anyhow, Result};
use std::collections::HashMap;
use flutter_rust_bridge::*;

pub fn greet() -> String {
    "Hello from Rust! 🦀".into()
}

#[derive(Debug)]
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
pub struct CryptorHandle(pub i64);

lazy_static! {
    static ref POOL: Mutex<RefCell<HashMap<CryptorHandle, Arc<Cryptor>>>> = {
        let m = Mutex::new(RefCell::new(HashMap::new()));
        m
    };
}

pub fn cryptor_new(sub_pool_id: i64, key: Vec<u8>, iv_length: usize) -> Result<CryptorHandle> {
    let handle = CryptorHandle(sub_pool_id);
    POOL.lock().unwrap_or_else(|er| {
        er.into_inner()
    }).get_mut().insert(handle.clone(), Arc::new(Cryptor::new(key, iv_length)));
    Ok(handle)
}

pub fn cryptor_encrypt(cryptor: CryptorHandle, text: String) -> Result<Vec<u8>> {
    let mut pool = POOL.lock().unwrap_or_else(|er| {
        er.into_inner()
    });
    let cryptor = pool.get_mut().get_mut(& cryptor).unwrap();
    let result = cryptor.encrypt(text.as_str());
    Ok(result)
}

pub fn cryptor_decrypt(cryptor: CryptorHandle, data: Vec<u8>) -> Result<String> {
    let mut pool = POOL.lock().unwrap_or_else(|er| {
        er.into_inner()
    });
    let cryptor = pool.get_mut().get_mut(& cryptor).unwrap();
    let result = cryptor.decrypt(data.as_slice());
    Ok(result)
}

pub fn cryptor_remove(cryptor: CryptorHandle) -> Result<()> {
    POOL.lock().unwrap_or_else(|er| {
        er.into_inner()
    }).get_mut().remove(&cryptor).unwrap();
    Ok(())
}

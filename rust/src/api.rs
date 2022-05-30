use aes::cipher::{block_padding::Pkcs7, BlockDecryptMut, BlockEncryptMut, KeyIvInit};
use std::sync::{Arc, RwLock};
use anyhow::{Result};
use std::collections::HashMap;


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
        let iv = vec![0x0; iv_length];

        Cryptor {
            key: key.into_boxed_slice(),
            iv: iv.into_boxed_slice(),
        }
    }

    fn encryptor(&self) -> cbc::Encryptor<aes::Aes256> {
        cbc::Encryptor::<aes::Aes256>::new((*self.key).into(), (*self.iv).into())
    }

    fn decryptor(&self) -> cbc::Decryptor<aes::Aes256> {
        cbc::Decryptor::<aes::Aes256>::new((*self.key).into(), (*self.iv).into())
    }

    pub fn encrypt(& self, text: &str) -> Vec<u8> {
        let enc = self.encryptor();
        let ct = enc.encrypt_padded_vec_mut::<Pkcs7>(text.as_bytes());
        Vec::from(ct)
    }

    pub fn decrypt(& self, value: &[u8]) -> String {
        let enc = self.decryptor();
        let ct = enc.decrypt_padded_vec_mut::<Pkcs7>(&value)
                .unwrap();
        String::from_utf8(Vec::from(ct)).unwrap()
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct CryptorHandle(pub i64);

lazy_static! {
    static ref POOL: RwLock<HashMap<CryptorHandle, Arc<Cryptor>>> = {
        let m = RwLock::new(HashMap::new());
        m
    };
}

pub fn cryptor_new(sub_pool_id: i64, key: Vec<u8>, iv_length: usize) -> Result<CryptorHandle> {
    let handle = CryptorHandle(sub_pool_id);
    POOL.write().unwrap_or_else(|er| {
        er.into_inner()
    }).insert(handle.clone(), Arc::new(Cryptor::new(key, iv_length)));
    Ok(handle)
}

fn cryptor_read(cryptor: & CryptorHandle) -> Arc<Cryptor> {
    let pool = POOL.read().unwrap_or_else(|er| {
        er.into_inner()
    });
    pool.get(cryptor).unwrap().clone()
}

pub fn cryptor_encrypt(cryptor: CryptorHandle, text: String) -> Result<Vec<u8>> {
    let cryptor = cryptor_read(& cryptor);
    let result = cryptor.encrypt(text.as_str());
    Ok(result)
}

pub fn cryptor_decrypt(cryptor: CryptorHandle, data: Vec<u8>) -> Result<String> {
    let cryptor = cryptor_read(& cryptor);
    let result = cryptor.decrypt(data.as_slice());
    Ok(result)
}

pub fn cryptor_remove(cryptor: CryptorHandle) -> Result<()> {
    let mut lock_guard = POOL.write().unwrap_or_else(|er| {
        er.into_inner()
    });
    lock_guard.remove(&cryptor).unwrap();
    Ok(())
}

mod bridge_generated; /* AUTO INJECTED BY flutter_rust_bridge. This line may not be accurate, and you can change it according to your needs. */
mod api;

#[macro_use]
extern crate lazy_static;

#[cfg(test)]
mod tests {
    use hex_literal::hex;
    use crate::api::{Cryptor, cryptor_decrypt, cryptor_encrypt, cryptor_new, cryptor_remove};

    #[test]
    fn encryption_works() {
        let key = [0x42u8; 16];
        let plaintext = "hello world! this is my plaintext.";
        let ciphertext = hex!(
            "c7fe247ef97b21f07cbdd26cb5d346bf"
            "d27867cb00d9486723e159978fb9a5f9"
            "14cfb228a710de4171e396e7b6cf859e"
        );

        let cryptor = Cryptor::new(Vec::from(key), 16);
        let ct = cryptor.encrypt(plaintext);

        assert_eq!(ct, &ciphertext[..]);
    }

    #[test]
    fn decryption_works() {
        let key = [0x42u8; 16];
        let plaintext = "hello world! this is my plaintext.";
        let ciphertext = hex!(
            "c7fe247ef97b21f07cbdd26cb5d346bf"
            "d27867cb00d9486723e159978fb9a5f9"
            "14cfb228a710de4171e396e7b6cf859e"
        );

        let cryptor = Cryptor::new(Vec::from(key), 16);
        let pt = cryptor.decrypt(&ciphertext);

        assert_eq!(pt, plaintext);
    }

    #[test]
    fn encryption_api_works() {
        let key = [0x42u8; 16];
        let plaintext = "hello world! this is my plaintext.";
        let ciphertext = hex!(
            "c7fe247ef97b21f07cbdd26cb5d346bf"
            "d27867cb00d9486723e159978fb9a5f9"
            "14cfb228a710de4171e396e7b6cf859e"
        );

        let cryptor = cryptor_new(14600, Vec::from(key), 16).unwrap();
        let ct = cryptor_encrypt(cryptor.clone(), plaintext.into()).unwrap();
        cryptor_remove(cryptor).unwrap();

        assert_eq!(ct, &ciphertext[..]);
    }

    #[test]
    fn decryption_api_works() {
        let key = [0x42u8; 16];
        let plaintext = "hello world! this is my plaintext.";
        let ciphertext = hex!(
            "c7fe247ef97b21f07cbdd26cb5d346bf"
            "d27867cb00d9486723e159978fb9a5f9"
            "14cfb228a710de4171e396e7b6cf859e"
        );

        let cryptor = cryptor_new(15680, Vec::from(key), 16).unwrap();
        let pt = cryptor_decrypt(cryptor.clone(), Vec::from(ciphertext)).unwrap();
        cryptor_remove(cryptor).unwrap();

        assert_eq!(pt, plaintext);
    }
}

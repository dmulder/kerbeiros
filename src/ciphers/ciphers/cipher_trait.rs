pub use crate::error::*;

pub trait KerberosCipher {
    fn generate_key(&self, raw_key: &[u8], salt: &[u8]) -> Vec<u8>;
    fn generate_key_from_password(&self, password: &str, salt: &[u8]) -> Vec<u8>;
    fn decrypt(&self, key: &[u8], key_usage: i32, ciphertext: &[u8]) -> Result<Vec<u8>>;

    fn generate_key_and_decrypt(
        &self,
        raw_key: &[u8],
        salt: &[u8],
        key_usage: i32,
        ciphertext: &[u8],
    ) -> Result<Vec<u8>> {
        let key = self.generate_key(raw_key, salt);
        return self.decrypt(&key, key_usage, ciphertext);
    }

    fn generate_key_from_password_and_decrypt(
        &self,
        password: &str,
        salt: &[u8],
        key_usage: i32,
        ciphertext: &[u8],
    ) -> Result<Vec<u8>> {
        let key = self.generate_key_from_password(password, salt);
        return self.decrypt(&key, key_usage, ciphertext);
    }

    fn encrypt(&self, key: &[u8], key_usage: i32, plaintext: &[u8]) -> Vec<u8>;

    fn generate_key_and_encrypt(
        &self,
        raw_key: &[u8],
        salt: &[u8],
        key_usage: i32,
        ciphertext: &[u8],
    ) -> Vec<u8> {
        let key = self.generate_key(raw_key, salt);
        return self.encrypt(&key, key_usage, ciphertext);
    }

    fn generate_key_from_password_and_encrypt(
        &self,
        password: &str,
        salt: &[u8],
        key_usage: i32,
        ciphertext: &[u8],
    ) -> Vec<u8> {
        let key = self.generate_key_from_password(password, salt);
        return self.encrypt(&key, key_usage, ciphertext);
    }
}

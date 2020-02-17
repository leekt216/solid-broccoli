#![feature(const_generics)]
pub mod des;
trait BlockCipher {
    fn encrypt(input: [u8;8], key: [u8;8]) -> [u8;8];
    fn decrypt(cipher: [u8;8], key: [u8;8]) -> [u8;8];
}

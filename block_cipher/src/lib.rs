pub mod des;
mod consts;
trait BlockCipher {
    fn encrypt(input: [u8;8], key: [u8;8]) -> [u8;8];
    fn decrypt(cipher: [u8;8], key: [u8;8]) -> [u8;8];
}

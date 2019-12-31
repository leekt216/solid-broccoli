/// DES on (K,I) -> R
/// K : key, 56 bit
/// I : input, 64 bit 
/// R : Result, 64 bit
/// Algorithm :
/// fn initialPermutation(I: 64bit) -> 64bit;
/// fn split(IP: 64bit) -> 32bit, 32bit;
/// fn feistel(L0: 32bit, R0: 32bit, K: 48bit) -> 32bit, 32bit;
/// fn inverseIP(R16, L16) -> 64bit;


fn initialPermutation(i: u64) -> u64 {
    
}

fn finalPermutation(i: u64) -> u64) {

}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

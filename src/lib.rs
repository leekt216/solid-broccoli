/// DES on (K,I) -> R
/// K : key, 56 bit
/// I : input, 64 bit 
/// R : Result, 64 bit
/// Algorithm :
/// fn initialPermutation(I: 64bit) -> 64bit;
/// fn split(IP: 64bit) -> 32bit, 32bit;
/// fn feistel(L0: 32bit, R0: 32bit, K: 48bit) -> 32bit, 32bit;
/// fn inverseIP(R16, L16) -> 64bit;


fn permutation_table(x:u8) -> u8 {
    let table = [
        58,50,42,34,26,18,10,02,
        60,52,44,36,28,20,12,04,
        62,54,46,38,30,22,14,06,
        64,56,48,40,32,24,16,08,
        57,49,41,33,25,17,09,01,
        59,51,46,35,27,19,11,03,
        61,33,45,37,29,21,13,05,
        63,55,47,39,31,23,15,07,
    ];
    table[x as usize] - 1
}

fn initial_permutation(input: &[u8;8]) -> [u8;8] {
    let mut r : [u8; 8] = [0 as u8;8];
    for i in 0..8 {
        r[i] = permutation_table(input[i] - 1);
    }
    r.clone()
}

#[cfg(test)]
mod tests {
    use crate::*;
    #[test]
    fn it_works() {
        let x = initial_permutation(&[3,3,3,3,3,3,3,3]);
        dbg!(x);
        assert_eq!(2 + 2, 4);
    }
}

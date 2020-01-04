/// DES on (K,I) -> R
/// K : key, 56 bit
/// I : input, 64 bit 
/// R : Result, 64 bit
/// Algorithm :
/// fn initialPermutation(I: 64bit) -> 64bit;
/// fn split(IP: 64bit) -> 32bit, 32bit;
/// fn feistel(L0: 32bit, R0: 32bit, K: 48bit) -> 32bit, 32bit;
/// fn inverseIP(R16, L16) -> 64bit;
trait Permutation {
    const TABLE: [u8; 64];
    fn run(input: u64) -> u64 {
        let mut r : u64 = 0;
        for i in 0..64 {
            r |=  if input & (1<<Self::TABLE[i as usize]) != 0 {1<<i} else {0};
        }
        r.clone()
    }
}

struct InitialPermutation {}
impl Permutation for InitialPermutation {
    const TABLE: [u8; 64]= [
        57, 49, 41, 33, 25, 17, 9, 1,
        59, 51, 43, 35, 27, 19, 11, 3,
        61, 53, 45, 37, 29, 21, 13, 5,
        63, 55, 47, 39, 31, 23, 15, 7,
        56, 48, 40, 32, 24, 16, 8, 0,
        58, 50, 42, 34, 26, 18, 10, 2,
        60, 52, 44, 36, 28, 20, 12, 4,
        62, 54, 46, 38, 30, 22, 14, 6
    ];
}

struct FinalPermutation {}
impl Permutation for FinalPermutation {
    const TABLE: [u8; 64] = [
        39, 7, 47, 15, 55, 23, 63, 31,
        38, 6, 46, 14, 54, 22, 62, 30,
        37, 5, 45, 13, 53, 21, 61, 29,
        36, 4, 44, 12, 52, 20, 60, 28,
        35, 3, 43, 11, 51, 19, 59, 27,
        34, 2, 42, 10, 50, 18, 58, 26,
        33, 1, 41, 9, 49, 17, 57, 25,
        32, 0, 40, 8, 48, 16, 56, 24
    ];
}

fn split(input: u64) -> (u32, u32) {
    let bytes = input.to_be_bytes();
    let mut l : [u8;4] = Default::default();
    l.copy_from_slice(&bytes[0..4]);
    let mut r : [u8;4] = Default::default();
    r.copy_from_slice(&bytes[4..8]);
    (u32::from_be_bytes(l),u32::from_be_bytes(r))
}

fn xor(lhs: u64, rhs: u64) -> u64 {
    lhs ^ rhs
}

#[cfg(test)]
mod DESTests {
    use crate::*;
    #[test]
    fn should_permut_correctly() {
        let input = 81985529216486895 as u64;
        let x = InitialPermutation::run(input);
        assert_eq!(x, 14699974583363760298 as u64);
    }

    #[test]
    fn should_split_correctly() {
        let input = 14699974583363760298 as u64;
        let (l,r) = split(input);
        assert_eq!((l,r), (3422604543 as u32, 4037734570 as u32));
    }
}

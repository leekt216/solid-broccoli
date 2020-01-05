/// DES on (K,I) -> R
/// K : key, 56 bit
/// I : input, 64 bit 
/// R : Result, 64 bit
/// Algorithm :
/// fn initialPermutation(I: 64bit) -> 64bit;
/// fn split(IP: 64bit) -> 32bit, 32bit;
/// fn feistel(L0: 32bit, R0: 32bit, K: 48bit) -> 32bit, 32bit;
/// fn inverseIP(R16, L16) -> 64bit;
const EXPANSION_TABLE : [u8; 48] = [
    31, 0, 1, 2, 3, 4, 3, 4,
    5, 6, 7, 8, 7, 8, 9, 10,
    11, 12, 11, 12, 13, 14, 15, 16,
    15, 16, 17, 18, 19, 20, 19, 20,
    21, 22, 23, 24, 23, 24, 25, 26,
    27, 28, 27, 28, 29, 30, 31, 0
];

trait BitUtil {
    fn get_bit(&self, index: u8) -> bool;
    //TODO : change to Result?
    fn set_bit(&mut self, index: u8) -> bool;
}

impl BitUtil for [u8] {
    fn get_bit(&self, index: u8) -> bool {
        if index > (self.len() *8 - 1) as u8 { false }
        else {
            //get self index from index
            let s = index / 8;
            let i = index % 8;
            self[s as usize] & (1<<(7-i)) !=0
        }
    }

    //set bit to 1
    fn set_bit(&mut self, index: u8) -> bool {
        if index > (self.len() *8 - 1) as u8 { false }
        else {
            //get self index from index
            let s = index / 8;
            let i = index % 8;
            self[s as usize] |= 1 << (7-i);
            true
        }
    }
}

trait Permutation {
    const TABLE: [u8; 64];
    fn run(input: [u8;8]) -> [u8;8] {
        let mut r : [u8;8] = Default::default();
        for i in 0..64 {
            if input.get_bit(Self::TABLE[i as usize]) {
                r.set_bit(i);
            }
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

fn split(input: [u8;8]) -> ([u8;4], [u8;4]) {
    let mut l : [u8;4] = Default::default();
    l.copy_from_slice(&input[0..4]);
    let mut r : [u8;4] = Default::default();
    r.copy_from_slice(&input[4..8]);
    (l,r)
}

fn expand(input: [u8;4]) -> [u8;6] {
    let mut r : [u8;6] = Default::default();
    for i in 0..48{
        if input.get_bit(EXPANSION_TABLE[i as usize]) {
            r.set_bit(i);
        }
    }
    r.clone()
}

#[cfg(test)]
mod DESTests {
    use crate::*;
    #[test]
    fn should_permut_correctly() {
        let input = 81985529216486895 as u64;
        let x = InitialPermutation::run(input.to_be_bytes());
        assert_eq!(u64::from_be_bytes(x), 14699974583363760298);
    }

    #[test]
    fn should_split_correctly() {
        let input = 14699974583363760298 as u64;
        let (l,r) = split(input.to_be_bytes());
        assert_eq!((l,r), ((3422604543 as u32).to_be_bytes(), (4037734570 as u32).to_be_bytes()));
    }

    #[test]
    fn should_expand_correctly() {
        let input = 4037734570 as u32;
        let expanded = expand(input.to_be_bytes());
        assert_eq!(expanded, (134232046966101 as u64).to_be_bytes()[2..8]);
    }
}

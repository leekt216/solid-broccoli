#![feature(const_generics)]
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
    fn get_bit(&self, index: usize) -> bool;
    //TODO : change to Result?
    fn set_bit(&mut self, index: usize) -> bool;
    fn rotate(&self, rhs: usize) -> Self;
    fn rotate_as_bits(&self, rhs: usize, bits: usize) -> Self;
    fn right_shift(&self, rhs: usize) -> Self;
    fn left_shift(&self, rhs: usize) -> Self;
}

impl<const N: usize> BitUtil for [u8; N] {
    fn get_bit(&self, index: usize) -> bool {
        if index >= N*8 { false }
        else {
            //get self index from index
            let s = index / 8;
            let i = index % 8;
            self[s as usize] & (1<<(7-i)) !=0
        }
    }

    //set bit to 1
    fn set_bit(&mut self, index: usize) -> bool {
        if index >= N*8 { false }
        else {
            //get self index from index
            let s = index / 8;
            let i = index % 8;
            self[s as usize] |= 1 << (7-i);
            true
        }
    }

    //rotate left
    fn rotate(&self, rhs: usize) -> [u8; N] {
        let mut x = self.clone();
        for i in 0..N {
            x[i] = self[i] << rhs;
            let k = if i+1 == self.len()  {0} else {i+1};
            x[i] += self[k] >> (8-rhs);
        }
        x
    }

    //right shift 
    fn right_shift(&self, rhs: usize) -> [u8; N] {
        let mut x = self.clone();
        x[0] = self[0] >> rhs;
        for i in 1..N {
            x[i] = self[i] >> rhs;
            x[i] += self[i-1] << (8-rhs);
        }
        x
    }
    
    //left shift 
    fn left_shift(&self, rhs: usize) -> [u8; N] {
        let mut x = self.clone();
        for i in 0..N-1 {
            x[i] = self[i] << rhs;
            x[i] += self[i+1] >> (8-rhs);
        }
        x[N-1] = self[N-1] << rhs;
        x
    }

    fn rotate_as_bits(&self, rhs:usize, bits: usize) -> [u8; N] {
        let mut x = self.rotate(rhs);
        for i in 0..N {
            x[i] = self[i] << rhs;
            let (k, o) = if i+1 == self.len() {(0, N*8 - bits)} else {(i+1, 0)};
            x[i] += self[k] >> (8 - rhs - o);
        }

        for i in 0..(N*8 - bits) {
            x[0] &= 255 as u8 >> i + 1;
        }
        x
    }
}

trait Permutation {
    const TABLE: [u8; 64];
    fn run(input: [u8;8]) -> [u8;8] {
        let mut r : [u8;8] = Default::default();
        for i in 0..64 {
            if input.get_bit(Self::TABLE[i as usize] as usize) {
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

fn split_64bit(input: [u8;8]) -> ([u8;4], [u8;4]) {
    let mut l : [u8;4] = Default::default();
    l.copy_from_slice(&input[0..4]);
    let mut r : [u8;4] = Default::default();
    r.copy_from_slice(&input[4..8]);
    (l,r)
}

fn split_56bit(input: [u8;7]) -> ([u8;4], [u8;4]) {
    let mut l : [u8;4] = Default::default();
    l.copy_from_slice(&input[0..4]);
    l = l.right_shift(4);
    let mut r : [u8;4] = Default::default();
    r.copy_from_slice(&input[3..7]);
    r = r.left_shift(4).right_shift(4);
    (l,r)
}

const PARITY_DROP_TABLE: [u8;56] = [
    56, 48, 40, 32, 24, 16, 8, 0,
    57, 49, 41, 33, 25, 17, 9, 1,
    58, 50, 42, 34, 26, 18, 10, 2,
    59, 51, 43, 35, 62, 54, 46, 38,
    30, 22, 14, 6, 61, 53, 45, 37,
    29, 21, 13, 5, 60, 52, 44, 36,
    28, 20, 12, 4, 27, 19, 11, 3
];

fn parity_drop(input: [u8;8]) -> [u8;7] {
    let mut r : [u8;7] = Default::default();
    for i in 0..56 {
        if input.get_bit(PARITY_DROP_TABLE[i as usize] as usize)  {
            r.set_bit(i);
        }
    }
    r.clone()
}

/*
fn round_keys(input: [u8;8]) ->[[u8;6];16]{
    let mut r : [[u8;6]; 16] = Default::default();
}

*/
fn expand(input: [u8;4]) -> [u8;6] {
    let mut r : [u8;6] = Default::default();
    for i in 0..48{
        if input.get_bit(EXPANSION_TABLE[i as usize] as usize) {
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
        let input = 81_985_529_216_486_895 as u64;
        let x = InitialPermutation::run(input.to_be_bytes());
        assert_eq!(u64::from_be_bytes(x), 14_699_974_583_363_760_298);
    }

    #[test]
    fn should_split_correctly() {
        let input = 14_699_974_583_363_760_298 as u64;
        let (l,r) = split_64bit(input.to_be_bytes());
        assert_eq!((l,r), ((3_422_604_543 as u32).to_be_bytes(), (4_037_734_570 as u32).to_be_bytes()));
    }

    #[test]
    fn should_expand_correctly() {
        let input = 4_037_734_570 as u32;
        let expanded = expand(input.to_be_bytes());
        assert_eq!(expanded, (134_232_046_966_101 as u64).to_be_bytes()[2..8]);
    }

    #[test]
    fn should_drop_parity_correctly() {
        let input = 1383827165325090801 as u64;
        let dropped = parity_drop(input.to_be_bytes());
        assert_eq!(dropped, (67779029043144591 as u64).to_be_bytes()[1..8]);
    }

    #[test]
    fn should_rotate_correctly() {
        let input = [127 as u8, 0 as u8];
        let rotated = input.rotate(1);
        assert_eq!(rotated, [254 as u8, 0 as u8]);
    }

    #[test]
    fn should_rotate_partial_correctly() {
        let input = [127 as u8, 0 as u8];
        let rotated = input.rotate_as_bits(1,15);
        assert_eq!(rotated, [126 as u8, 1 as u8]);
    }

    #[test]
    fn should_shift_correctly(){
        let input = [127 as u8, 0 as u8];
        let rotated = input.right_shift(1);
        assert_eq!(rotated, [63 as u8, 128 as u8]);
    }

    #[test]
    fn should_split_56bit_correctly() {
        let input = [255 as u8;7];
        let (l,r) = split_56bit(input);
        assert_eq!((l,r),([15, 255,255,255],[15,255,255,255]));
    }
}

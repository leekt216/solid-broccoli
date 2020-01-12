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

trait BitUtil {
    fn get_bit(&self, index: usize) -> bool;
    //TODO : change to Result?
    fn set_bit(&mut self, index: usize) -> bool;
    fn rotate(&self, rhs: usize) -> Self;
    fn rotate_as_bits(&self, rhs: usize, bits: usize) -> Self;
    fn right_shift(&self, rhs: usize) -> Self;
    fn left_shift(&self, rhs: usize) -> Self;
    fn xor(&self, x: Self) -> Self;
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

    fn xor(&self, x: [u8;N]) -> [u8;N] {
        let mut r = self.clone();
        for i in 0..N {
            r[i] ^= x[i];
        }
        r
    }

    fn rotate_as_bits(&self, rhs:usize, bits: usize) -> [u8; N] {
        let mut x = self.rotate(rhs);
        for i in 0..N {
            x[i] = self[i] << rhs;
            let (k, o) = if i+1 == self.len() {(0, N*8 - bits)} else {(i+1, 0)};
            x[i] += self[k] >> (8 - rhs - o);
        }

        for i in 0..(N*8 - bits) {
            x[0] &= 255 as u8 >> (i + 1);
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

fn merge_28bits(l: [u8;4], r:[u8;4]) -> [u8;7] {
    let l_temp = l.left_shift(4);
    let mut m : [u8;7] = Default::default();
    m[0] = l_temp[0];
    m[1] = l_temp[1];
    m[2] = l_temp[2];
    m[3] = l_temp[3] + r[0];
    m[4] = r[1];
    m[5] = r[2];
    m[6] = r[3];
    m
}

fn merge_32bits(l: [u8;4], r:[u8;4]) -> [u8;8] {
    let mut m : [u8;8] = Default::default();
    m[0] = l[0];
    m[1] = l[1];
    m[2] = l[2];
    m[3] = l[3];
    m[4] = r[0];
    m[5] = r[1];
    m[6] = r[2];
    m[7] = r[3];
    m
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
const PBOX_TABLE: [u8; 48] = [
    13, 16, 10, 23, 0, 4, 2, 27,
    14, 5, 20, 9, 22, 18, 11, 3,
    25, 7, 15, 6, 26, 19, 12, 1,
    40, 51, 30, 36, 46, 54, 29, 39,
    50, 44, 32, 47, 43, 48, 38, 55,
    33, 52, 45, 41, 49, 35, 28, 31
];

fn pbox_compress(input: [u8;7])->[u8;6] {
    let mut r:[u8;6] = Default::default();
    for i in 0..48 {
        if input.get_bit(PBOX_TABLE[i as usize] as usize) {
            r.set_bit(i);
        }
    }
    r.clone()
}

const ROUND_SHIFTS : [usize; 16] = [
    1, 1, 2, 2, 2, 2, 2, 2,
    1, 2, 2, 2, 2, 2, 2, 1
];

fn round_keys(input: [u8;8]) ->[[u8;6];16]{
    let mut keys : [[u8;6]; 16] = Default::default();
    let pc1 : [u8;7] = parity_drop(input);
    let (mut l,mut r) = split_56bit(pc1);
    for i in 0..16 {
        let temp_l : [u8;4] = l.rotate_as_bits(ROUND_SHIFTS[i], 28);
        let temp_r : [u8;4] = r.rotate_as_bits(ROUND_SHIFTS[i], 28);
        let m : [u8;7]  = merge_28bits(temp_l, temp_r);
        keys[i] = pbox_compress(m);
        l = temp_l;
        r = temp_r;
    }
    keys
}

const EXPANSION_TABLE : [u8; 48] = [
    31, 0, 1, 2, 3, 4, 3, 4,
    5, 6, 7, 8, 7, 8, 9, 10,
    11, 12, 11, 12, 13, 14, 15, 16,
    15, 16, 17, 18, 19, 20, 19, 20,
    21, 22, 23, 24, 23, 24, 25, 26,
    27, 28, 27, 28, 29, 30, 31, 0
];

fn expand(input: [u8;4]) -> [u8;6] {
    let mut r : [u8;6] = Default::default();
    for i in 0..48{
        if input.get_bit(EXPANSION_TABLE[i as usize] as usize) {
            r.set_bit(i);
        }
    }
    r.clone()
}

fn split_48bits(input: [u8;6]) -> [u8; 8] {
    let mut r :[u8;8] = Default::default();
    let mut temp = input.clone();
    for i in 0..8 {
        r[i] = [temp[0]].right_shift(2)[0];
        temp = temp.left_shift(6);
    }
    r
}

const SBOX : [[[u8;16];4];8] =
[
    [
        [14, 4, 13, 1, 2, 15, 11, 8, 3, 10, 6, 12, 5, 9, 0, 7],
        [0, 15, 7, 4, 14, 2, 13, 1, 10, 6, 12, 11, 9, 5, 3, 8],
        [4, 1, 14, 8, 13, 6, 2, 11, 15, 12, 9, 7, 3, 10, 5, 0],
        [15, 12, 8, 2, 4, 9, 1, 7, 5, 11, 3, 14, 10, 0, 6, 13]
    ], 
    [
        [15, 1, 8, 14, 6, 11, 3, 4, 9, 7, 2, 13, 12, 0, 5, 10],
        [3, 13, 4, 7, 15, 2, 8, 14, 12, 0, 1, 10, 6, 9, 11, 5],
        [0, 14, 7, 11, 10, 4, 13, 1, 5, 8, 12, 6, 9, 3, 2, 15],
        [13, 8, 10, 1, 3, 15, 4, 2, 11, 6, 7, 12, 0, 5, 14, 9]
    ], 
    [
        [10, 0, 9, 14, 6, 3, 15, 5, 1, 13, 12, 7, 11, 4, 2, 8], 
        [13, 7, 0, 9, 3, 4, 6, 10, 2, 8, 5, 14, 12, 11, 15, 1], 
        [13, 6, 4, 9, 8, 15, 3, 0, 11, 1, 2, 12, 5, 10, 14, 7], 
        [1, 10, 13, 0, 6, 9, 8, 7, 4, 15, 14, 3, 11, 5, 2, 12]
    ], 
    [
        [7, 13, 14, 3, 0, 6, 9, 10, 1, 2, 8, 5, 11, 12, 4, 15], 
        [13, 8, 11, 5, 6, 15, 0, 3, 4, 7, 2, 12, 1, 10, 14, 9], 
        [10, 6, 9, 0, 12, 11, 7, 13, 15, 1, 3, 14, 5, 2, 8, 4], 
        [3, 15, 0, 6, 10, 1, 13, 8, 9, 4, 5, 11, 12, 7, 2, 14]
    ], 
    [
        [2, 12, 4, 1, 7, 10, 11, 6, 8, 5, 3, 15, 13, 0, 14, 9], 
        [14, 11, 2, 12, 4, 7, 13, 1, 5, 0, 15, 10, 3, 9, 8, 6], 
        [4, 2, 1, 11, 10, 13, 7, 8, 15, 9, 12, 5, 6, 3, 0, 14], 
        [11, 8, 12, 7, 1, 14, 2, 13, 6, 15, 0, 9, 10, 4, 5, 3]
    ], 
    [
        [12, 1, 10, 15, 9, 2, 6, 8, 0, 13, 3, 4, 14, 7, 5, 11], 
        [10, 15, 4, 2, 7, 12, 9, 5, 6, 1, 13, 14, 0, 11, 3, 8], 
        [9, 14, 15, 5, 2, 8, 12, 3, 7, 0, 4, 10, 1, 13, 11, 6], 
        [4, 3, 2, 12, 9, 5, 15, 10, 11, 14, 1, 7, 6, 0, 8, 13]
    ], 
    [
        [4, 11, 2, 14, 15, 0, 8, 13, 3, 12, 9, 7, 5, 10, 6, 1], 
        [13, 0, 11, 7, 4, 9, 1, 10, 14, 3, 5, 12, 2, 15, 8, 6], 
        [1, 4, 11, 13, 12, 3, 7, 14, 10, 15, 6, 8, 0, 5, 9, 2], 
        [6, 11, 13, 8, 1, 4, 10, 7, 9, 5, 0, 15, 14, 2, 3, 12]
    ],
    [
        [13, 2, 8, 4, 6, 15, 11, 1, 10, 9, 3, 14, 5, 0, 12, 7],
        [1, 15, 13, 8, 10, 3, 7, 4, 12, 5, 6, 11, 0, 14, 9, 2],
        [7, 11, 4, 1, 9, 12, 14, 2, 0, 6, 10, 13, 15, 3, 5, 8],
        [2, 1, 14, 7, 4, 10, 8, 13, 15, 12, 9, 0, 3, 5, 6, 11]
    ]
];

fn sbox_lookup(i: usize, bit6: u8) -> u8 {
    let f : u8 = ((bit6 >> 4) & 2) + (bit6 & 1);
    let j : u8 = (bit6 & 30) >> 1;
    SBOX[i][f as usize][j as usize]
}

const STRAIGHT_PERMUTATION :[u8;32] = [
    15, 6, 19, 20, 28, 11, 27, 16,
    0, 14, 22, 25, 4, 17, 30, 9,
    1, 7, 23, 13, 31, 26, 2, 8,
    18, 12, 29, 5, 21, 10, 3, 24
];

fn straight_permutation(input: [u8;4]) -> [u8;4] {
    let mut r : [u8;4] = Default::default();
    for i in 0..32{
        if input.get_bit(STRAIGHT_PERMUTATION[i as usize] as usize) {
            r.set_bit(i);
        }
    }
    r.clone()
}

fn feistel(input: [u8;4], key: [u8;6]) -> [u8;4] {
    let expanded: [u8;6] = expand(input);
    let xored : [u8;6] = expanded.xor(key);
    let bit6s : [u8;8] = split_48bits(xored);
    let mut result : u32 = 0u32;
    for i in 0..8 {
        let op = sbox_lookup(i,bit6s[i]) as u32;
        result += (op << (7-i)*4);
    }
    let r : [u8;4] = straight_permutation(result.to_be_bytes());
    r
}

fn encrypt(input: [u8;8], key: [u8;8]) -> [u8;8] {
    let keys = round_keys(key);
    let ip = InitialPermutation::run(input);
    let (mut l, mut r) = split_64bit(ip);

    for i in 0..16 {
        let newR = l.xor(feistel(r, keys[i]));
        let newL = r;
        if i != 15 {
            r = newR;
            l = newL;
        }
        else {
            l = newR;
        }
    }
    let m : [u8;8] = merge_32bits(l,r);
    let c : [u8;8] = FinalPermutation::run(m);
    c
}

//test set
//plaintext = "123456ABCD132536"
//key = "AABB09182736CCDD"
//initial permutation = "14A7D67818CA18AD"
//cipher text = "C0B7A8D05F3A829C"
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
        let input = 1_383_827_165_325_090_801 as u64;
        let dropped = parity_drop(input.to_be_bytes());
        assert_eq!(dropped, (67_779_029_043_144_591 as u64).to_be_bytes()[1..8]);
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
        let input = [129 as u8;7];
        let (l,r) = split_56bit(input);
        assert_eq!((l,r),([8, 24, 24, 24],[1,129,129,129]));
        let m = merge_28bits(l,r);
        assert_eq!(m, input);
    }

    #[test]
    fn should_xor_correctly() {
        let input = [129u8, 255u8];
        let x = [3u8,128u8];
        let m = input.xor(x);
        assert_eq!(m,[130u8, 127u8]);
    }

    #[test]
    fn should_split_48bits_correctly() {
        let input = [255u8;6];
        let arr = split_48bits(input);
        assert_eq!(arr,[63u8;8]);
    }

    #[test]
    fn should_return_appropriate_sbox() {
        let input = 63u8;
        let sbox = sbox_lookup(7,input);
        assert_eq!(sbox, SBOX[7][3][15]);
    }

    #[test]
    fn should_encrypt_correctly() {
        let input = [ 0x12, 0x34, 0x56 ,0xab, 0xcd, 0x13, 0x25, 0x36 ];
        let key = [0xaa, 0xbb, 0x09, 0x18, 0x27, 0x36, 0xcc, 0xdd];
        let c : [u8;8] = encrypt(input, key);
        assert_eq!([0xc0, 0xb7, 0xa8, 0xd0, 0x5f, 0x3a, 0x82, 0x9c], c);
    }
}

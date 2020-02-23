/// DES on (K,I) -> R
/// K : key, 56 bit
/// I : input, 64 bit 
/// R : Result, 64 bit
/// Algorithm :
/// fn initialPermutation(I: 64bit) -> 64bit;
/// fn split(IP: 64bit) -> 32bit, 32bit;
/// fn feistel(L0: 32bit, R0: 32bit, K: 48bit) -> 32bit, 32bit;
/// fn inverseIP(R16, L16) -> 64bit;
use crate::consts;
use bitutils::*;
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

fn parity_drop(input: [u8;8]) -> [u8;7] {
    let mut r : [u8;7] = Default::default();
    for i in 0..56 {
        if input.get_bit(consts::PARITY_DROP[i as usize] as usize)  {
            r.set_bit(i);
        }
    }
    r.clone()
}

fn pbox_compress(input: [u8;7])->[u8;6] {
    let mut r:[u8;6] = Default::default();
    for i in 0..48 {
        if input.get_bit(consts::PBOX[i as usize] as usize) {
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

fn expand(input: [u8;4]) -> [u8;6] {
    let mut r : [u8;6] = Default::default();
    for i in 0..48{
        if input.get_bit(consts::EXPANSION[i as usize] as usize) {
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

fn sbox_lookup(i: usize, bit6: u8) -> u8 {
    let f : u8 = ((bit6 >> 4) & 2) + (bit6 & 1);
    let j : u8 = (bit6 & 30) >> 1;
    consts::SBOX[i][f as usize][j as usize]
}

fn permutation(table: [u8;64], input: [u8;8]) -> [u8;8] {
    let mut r : [u8;8] = Default::default();
    for i in 0..64{
        if input.get_bit(table[i as usize] as usize) {
            r.set_bit(i);
        }
    }
    r.clone()
}

fn straight_permutation(input: [u8;4]) -> [u8;4] {
    let mut r : [u8;4] = Default::default();
    for i in 0..32{
        if input.get_bit(consts::STRAIGHT_PERMUTATION[i as usize] as usize) {
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
        result += op << ((7-i)*4);
    }
    let r : [u8;4] = straight_permutation(result.to_be_bytes());
    r
}

pub fn encrypt(input: [u8;8], key: [u8;8]) -> [u8;8] {
    let keys = round_keys(key);
    let ip = permutation(consts::INITIAL_PERMUTATION, input);
    //let ip = InitialPermutation::run(input);
    let (mut l, mut r) = split_64bit(ip);

    for i in 0..16 {
        let new_r = l.xor(feistel(r, keys[i]));
        let new_l = r;
        if i != 15 {
            r = new_r;
            l = new_l;
        }
        else {
            l = new_r;
        }
    }
    let m : [u8;8] = merge_32bits(l,r);
    let c = permutation(consts::FINAL_PERMUTATION, m);
    //let c : [u8;8] = FinalPermutation::run(m);
    c
}

pub fn decrypt(cipher: [u8;8], key: [u8;8]) -> [u8;8] {
    let mut keys = round_keys(key);
    keys.reverse();
    let ip = permutation(consts::INITIAL_PERMUTATION, cipher);
    let (mut l, mut r) = split_64bit(ip);

    for i in 0..16 {
        let new_r = l.xor(feistel(r, keys[i]));
        let new_l = r;
        if i != 15 {
            r = new_r;
            l = new_l;
        }
        else {
            l = new_r;
        }
    }
    let m : [u8;8] = merge_32bits(l,r);
    let c = permutation(consts::FINAL_PERMUTATION, m);
    c
}

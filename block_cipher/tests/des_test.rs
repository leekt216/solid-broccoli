use block_cipher::des::DES;
//test set
//plaintext = "123456ABCD132536"
//key = "AABB09182736CCDD"
//initial permutation = "14A7D67818CA18AD"
//cipher text = "C0B7A8D05F3A829C"
/*#[test]
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
*/
#[test]
fn should_encrypt_correctly() {
    let input = [ 0x12, 0x34, 0x56 ,0xab, 0xcd, 0x13, 0x25, 0x36 ];
    let key = [0xaa, 0xbb, 0x09, 0x18, 0x27, 0x36, 0xcc, 0xdd];
    let c : [u8;8] = DES::encrypt(input, key);
    assert_eq!([0xc0, 0xb7, 0xa8, 0xd0, 0x5f, 0x3a, 0x82, 0x9c], c);
}

#[test]
fn should_decrypt_correctly() {
    let input = [ 0x12, 0x34, 0x56 ,0xab, 0xcd, 0x13, 0x25, 0x36 ];
    let key = [0xaa, 0xbb, 0x09, 0x18, 0x27, 0x36, 0xcc, 0xdd];
    let c : [u8;8] = DES::encrypt(input, key);
    let d : [u8;8] = DES::decrypt(c, key);
    assert_eq!(input, d);
}

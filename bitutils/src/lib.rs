#![feature(const_generics)]
pub trait BitUtil {
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

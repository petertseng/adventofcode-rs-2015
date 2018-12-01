#![allow(clippy::unreadable_literal)]

const K: [u32; 64] = [
    0xd76aa478, 0xe8c7b756, 0x242070db, 0xc1bdceee, 0xf57c0faf, 0x4787c62a, 0xa8304613, 0xfd469501,
    0x698098d8, 0x8b44f7af, 0xffff5bb1, 0x895cd7be, 0x6b901122, 0xfd987193, 0xa679438e, 0x49b40821,
    0xf61e2562, 0xc040b340, 0x265e5a51, 0xe9b6c7aa, 0xd62f105d, 0x02441453, 0xd8a1e681, 0xe7d3fbc8,
    0x21e1cde6, 0xc33707d6, 0xf4d50d87, 0x455a14ed, 0xa9e3e905, 0xfcefa3f8, 0x676f02d9, 0x8d2a4c8a,
    0xfffa3942, 0x8771f681, 0x6d9d6122, 0xfde5380c, 0xa4beea44, 0x4bdecfa9, 0xf6bb4b60, 0xbebfbc70,
    0x289b7ec6, 0xeaa127fa, 0xd4ef3085, 0x04881d05, 0xd9d4d039, 0xe6db99e5, 0x1fa27cf8, 0xc4ac5665,
    0xf4292244, 0x432aff97, 0xab9423a7, 0xfc93a039, 0x655b59c3, 0x8f0ccc92, 0xffeff47d, 0x85845dd1,
    0x6fa87e4f, 0xfe2ce6e0, 0xa3014314, 0x4e0811a1, 0xf7537e82, 0xbd3af235, 0x2ad7d2bb, 0xeb86d391,
];

#[rustfmt::skip]
const S : [u8; 64] = [
    7, 12, 17, 22,
    7, 12, 17, 22,
    7, 12, 17, 22,
    7, 12, 17, 22,
    5, 9, 14, 20,
    5, 9, 14, 20,
    5, 9, 14, 20,
    5, 9, 14, 20,
    4, 11, 16, 23,
    4, 11, 16, 23,
    4, 11, 16, 23,
    4, 11, 16, 23,
    6, 10, 15, 21,
    6, 10, 15, 21,
    6, 10, 15, 21,
    6, 10, 15, 21,
];

const BLOCK_SIZE: usize = 512;
const LENGTH_BITS: usize = 64;

pub struct MD5 {
    // Avoid repeatedly allocating by reusing this block.
    bytes: [u8; BLOCK_SIZE / 8],
}

impl Default for MD5 {
    fn default() -> Self {
        Self::new()
    }
}

impl MD5 {
    pub fn new() -> Self {
        Self {
            bytes: [0; BLOCK_SIZE / 8],
        }
    }

    pub fn digest_has_zeroes(&mut self, s: &[u8], zeroes: u8) -> bool {
        let (a, _, _, _) = self.digest(s);
        if zeroes > 8 {
            panic!("We're not ready to deal with {} zeroes", zeroes);
        }
        let mut requisite_mask = 0u32;
        for _ in 0..zeroes {
            requisite_mask >>= 4;
            requisite_mask |= 0xf0000000;
        }
        a & requisite_mask == 0
    }

    #[allow(clippy::many_single_char_names)]
    fn digest(&mut self, s: &[u8]) -> (u32, u32, u32, u32) {
        self.bytes[0..s.len()].copy_from_slice(s);
        let mut len = s.len();
        if len + 1 > (BLOCK_SIZE - LENGTH_BITS) / 8 {
            panic!("Can't handle inputs that long: {}", len);
        }
        self.bytes[len] = 0b10000000u8;
        let zeroes_to_append = (BLOCK_SIZE - LENGTH_BITS) / 8 - (len + 1);
        for i in 0..zeroes_to_append {
            self.bytes[len + 1 + i] = 0;
        }
        len *= 8;
        for i in 0..(LENGTH_BITS / 8) {
            self.bytes[(BLOCK_SIZE - LENGTH_BITS) / 8 + i] = (len & 0xff) as u8;
            len >>= 8;
        }

        let mut a0 = 0x67452301u32;
        let mut b0 = 0xefcdab89u32;
        let mut c0 = 0x98badcfeu32;
        let mut d0 = 0x10325476u32;

        // This is the usual "for each chunk" portion,
        // but this code only handles one.
        {
            let m = {
                let mut m = [0; 16];
                for (i, word) in m.iter_mut().enumerate() {
                    *word = u32::from(self.bytes[4 * i]);
                    *word |= u32::from(self.bytes[4 * i + 1]) << 8;
                    *word |= u32::from(self.bytes[4 * i + 2]) << 16;
                    *word |= u32::from(self.bytes[4 * i + 3]) << 24;
                }
                m
            };

            let mut a = a0;
            let mut b = b0;
            let mut c = c0;
            let mut d = d0;
            for i in 0..64 {
                let (f, g) = if i <= 15 {
                    ((b & c) | (!b & d), i)
                } else if 16 <= i && i <= 31 {
                    ((d & b) | (!d & c), (5 * i + 1) % 16)
                } else if 32 <= i && i <= 47 {
                    (b ^ c ^ d, (3 * i + 5) % 16)
                } else {
                    (c ^ (b | !d), (7 * i) % 16)
                };
                let f = f.wrapping_add(a).wrapping_add(K[i]).wrapping_add(m[g]);
                a = d;
                d = c;
                c = b;
                b = b.wrapping_add(f.rotate_left(u32::from(S[i])));
            }
            a0 = a0.wrapping_add(a);
            b0 = b0.wrapping_add(b);
            c0 = c0.wrapping_add(c);
            d0 = d0.wrapping_add(d);
        }

        (flip_end(a0), flip_end(b0), flip_end(c0), flip_end(d0))
    }
}

fn flip_end(n: u32) -> u32 {
    ((n & 0xff) << 24) | ((n & 0xff00) << 8) | ((n & 0xff0000) >> 8) | ((n & 0xff000000) >> 24)
}

#[cfg(test)]
mod tests {

    use super::*;

    fn hex(words: (u32, u32, u32, u32)) -> String {
        [words.0, words.1, words.2, words.3]
            .iter()
            .map(|b| format!("{:08x}", b))
            .collect::<Vec<_>>()
            .join("")
    }

    #[test]
    fn flip_end_single_left() {
        assert_eq!(flip_end(0x01000000), 1);
    }

    #[test]
    fn flip_end_single_right() {
        assert_eq!(flip_end(0x00000001), 0x01000000);
    }

    #[test]
    fn flip_end_all() {
        assert_eq!(flip_end(0x12345678), 0x78563412);
    }

    #[test]
    fn empty_str() {
        let digest = MD5::new().digest(&[]);
        assert_eq!(hex(digest), "d41d8cd98f00b204e9800998ecf8427e");
    }

    #[test]
    fn fox() {
        let bytes: Vec<u8> = "The quick brown fox jumps over the lazy dog"
            .bytes()
            .collect();
        let digest = MD5::new().digest(&bytes);
        assert_eq!(hex(digest), "9e107d9d372bb6826bd81d3542a419d6");
    }

    #[test]
    fn period() {
        let bytes: Vec<u8> = "The quick brown fox jumps over the lazy dog."
            .bytes()
            .collect();
        let digest = MD5::new().digest(&bytes);
        assert_eq!(hex(digest), "e4d909c290d0fb1ca068ffaddf22cbd0");
    }

    #[test]
    fn aoc_digest1() {
        let bytes: Vec<u8> = "abcdef609043".bytes().collect();
        let digest = MD5::new().digest(&bytes);
        assert_eq!(hex(digest), "000001dbbfa3a5c83a2d506429c7b00e");
    }

    #[test]
    fn aoc_digest2() {
        let bytes: Vec<u8> = "pqrstuv1048970".bytes().collect();
        let digest = MD5::new().digest(&bytes);
        assert_eq!(hex(digest), "000006136ef2ff3b291c85725f17325c");
    }

    #[test]
    fn aoc_zeroes1() {
        let bytes: Vec<u8> = "abcdef609043".bytes().collect();
        assert!(MD5::new().digest_has_zeroes(&bytes, 5));
    }

    #[test]
    fn aoc_zeroes2() {
        let bytes: Vec<u8> = "pqrstuv1048970".bytes().collect();
        assert!(MD5::new().digest_has_zeroes(&bytes, 5));
    }
}

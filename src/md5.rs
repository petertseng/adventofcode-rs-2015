// Some ideas from https://openwall.info/wiki/people/solar/software/public-domain-source-code/md5
// https://gist.github.com/lifthrasiir/7501753

#![allow(clippy::unreadable_literal)]

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

macro_rules! mix(
    (F:  $x:expr, $y:expr, $z:expr) => ($z ^ ($x & ($y ^ $z)));
    (G:  $x:expr, $y:expr, $z:expr) => ($y ^ ($z & ($x ^ $y)));
    (H:  $x:expr, $y:expr, $z:expr) => (($x ^ $y) ^ $z);
    (H2: $x:expr, $y:expr, $z:expr) => ($x ^ ($y ^ $z));
    (I:  $x:expr, $y:expr, $z:expr) => ($y ^ ($x | !$z))
);

/*
 * The MD5 transformation for all four rounds.
 */
macro_rules! step(($f:ident: $a:expr, $b:expr, $c:expr, $d:expr,
                   $x:expr, $t:expr, $s:expr) => ({
    $a = $a.wrapping_add(mix!($f: $b, $c, $d)).wrapping_add($x).wrapping_add($t);
    $a = $a.rotate_left($s);
    $a = $a.wrapping_add($b);
}));

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

            step!(F: a, b, c, d, m[0], 0xd76aa478, 7);
            step!(F: d, a, b, c, m[1], 0xe8c7b756, 12);
            step!(F: c, d, a, b, m[2], 0x242070db, 17);
            step!(F: b, c, d, a, m[3], 0xc1bdceee, 22);
            step!(F: a, b, c, d, m[4], 0xf57c0faf, 7);
            step!(F: d, a, b, c, m[5], 0x4787c62a, 12);
            step!(F: c, d, a, b, m[6], 0xa8304613, 17);
            step!(F: b, c, d, a, m[7], 0xfd469501, 22);
            step!(F: a, b, c, d, m[8], 0x698098d8, 7);
            step!(F: d, a, b, c, m[9], 0x8b44f7af, 12);
            step!(F: c, d, a, b, m[10], 0xffff5bb1, 17);
            step!(F: b, c, d, a, m[11], 0x895cd7be, 22);
            step!(F: a, b, c, d, m[12], 0x6b901122, 7);
            step!(F: d, a, b, c, m[13], 0xfd987193, 12);
            step!(F: c, d, a, b, m[14], 0xa679438e, 17);
            step!(F: b, c, d, a, m[15], 0x49b40821, 22);

            step!(G: a, b, c, d, m[1], 0xf61e2562, 5);
            step!(G: d, a, b, c, m[6], 0xc040b340, 9);
            step!(G: c, d, a, b, m[11], 0x265e5a51, 14);
            step!(G: b, c, d, a, m[0], 0xe9b6c7aa, 20);
            step!(G: a, b, c, d, m[5], 0xd62f105d, 5);
            step!(G: d, a, b, c, m[10], 0x02441453, 9);
            step!(G: c, d, a, b, m[15], 0xd8a1e681, 14);
            step!(G: b, c, d, a, m[4], 0xe7d3fbc8, 20);
            step!(G: a, b, c, d, m[9], 0x21e1cde6, 5);
            step!(G: d, a, b, c, m[14], 0xc33707d6, 9);
            step!(G: c, d, a, b, m[3], 0xf4d50d87, 14);
            step!(G: b, c, d, a, m[8], 0x455a14ed, 20);
            step!(G: a, b, c, d, m[13], 0xa9e3e905, 5);
            step!(G: d, a, b, c, m[2], 0xfcefa3f8, 9);
            step!(G: c, d, a, b, m[7], 0x676f02d9, 14);
            step!(G: b, c, d, a, m[12], 0x8d2a4c8a, 20);

            step!(H: a, b, c, d, m[5], 0xfffa3942, 4);
            step!(H2: d, a, b, c, m[8], 0x8771f681, 11);
            step!(H: c, d, a, b, m[11], 0x6d9d6122, 16);
            step!(H2: b, c, d, a, m[14], 0xfde5380c, 23);
            step!(H: a, b, c, d, m[1], 0xa4beea44, 4);
            step!(H2: d, a, b, c, m[4], 0x4bdecfa9, 11);
            step!(H: c, d, a, b, m[7], 0xf6bb4b60, 16);
            step!(H2: b, c, d, a, m[10], 0xbebfbc70, 23);
            step!(H: a, b, c, d, m[13], 0x289b7ec6, 4);
            step!(H2: d, a, b, c, m[0], 0xeaa127fa, 11);
            step!(H: c, d, a, b, m[3], 0xd4ef3085, 16);
            step!(H2: b, c, d, a, m[6], 0x04881d05, 23);
            step!(H: a, b, c, d, m[9], 0xd9d4d039, 4);
            step!(H2: d, a, b, c, m[12], 0xe6db99e5, 11);
            step!(H: c, d, a, b, m[15], 0x1fa27cf8, 16);
            step!(H2: b, c, d, a, m[2], 0xc4ac5665, 23);

            step!(I: a, b, c, d, m[0], 0xf4292244, 6);
            step!(I: d, a, b, c, m[7], 0x432aff97, 10);
            step!(I: c, d, a, b, m[14], 0xab9423a7, 15);
            step!(I: b, c, d, a, m[5], 0xfc93a039, 21);
            step!(I: a, b, c, d, m[12], 0x655b59c3, 6);
            step!(I: d, a, b, c, m[3], 0x8f0ccc92, 10);
            step!(I: c, d, a, b, m[10], 0xffeff47d, 15);
            step!(I: b, c, d, a, m[1], 0x85845dd1, 21);
            step!(I: a, b, c, d, m[8], 0x6fa87e4f, 6);
            step!(I: d, a, b, c, m[15], 0xfe2ce6e0, 10);
            step!(I: c, d, a, b, m[6], 0xa3014314, 15);
            step!(I: b, c, d, a, m[13], 0x4e0811a1, 21);
            step!(I: a, b, c, d, m[4], 0xf7537e82, 6);
            step!(I: d, a, b, c, m[11], 0xbd3af235, 10);
            step!(I: c, d, a, b, m[2], 0x2ad7d2bb, 15);
            step!(I: b, c, d, a, m[9], 0xeb86d391, 21);

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

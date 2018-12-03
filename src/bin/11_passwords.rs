fn increment(pw: &mut [u8], i: usize) {
    if pw[i] == b'z' {
        pw[i] = b'a';
        if i == 0 {
            panic!("No more passwords after {:?}", pw);
        }
        increment(pw, i - 1);
    } else {
        pw[i] += 1;
    }
}

fn valid(pw: &[u8]) -> bool {
    // Not clear whether the letters in the pairs must be different,
    // so we'll assume they must be.
    // Less work this way (don't have to keep track of indices).
    let mut pairs = std::collections::HashSet::<u8>::new();
    let mut straight = false;

    let mut c1 = 0;
    let mut c2 = 0;

    for &c in pw {
        if c == b'i' || c == b'o' || c == b'l' {
            return false;
        }
        if c == c2 {
            pairs.insert(c);
        }
        if c1 != 0 && c1 + 1 == c2 && c2 + 1 == c {
            straight = true;
        }

        c1 = c2;
        c2 = c;
    }

    pairs.len() >= 2 && straight
}

fn main() {
    let input = std::env::args()
        .nth(1)
        .unwrap_or_else(adventofcode::read_input_file);
    let last = input.len() - 1;
    let mut bytes = input.bytes().collect::<Vec<_>>();

    for _ in 0..2 {
        while !valid(&bytes) {
            increment(&mut bytes, last);
        }
        println!("{}", String::from_utf8(bytes.clone()).unwrap());
        increment(&mut bytes, last);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use adventofcode::tests;

    tests! {
        test_incr {
            single("a", 0, "b");
            in_many_simple("aa", 1, "ab");
            in_many_z("az", 1, "ba");
            in_many_cascade("azz", 2, "baa");
        }
        test_valid {
            straight_and_bad("hijklmmn", false);
            repeat_no_straight("abbceffg", false);
            only_one_double("abbcegjk", false);
            after_abcdefgh("abcdffaa", true);
            after_ghijklmn("ghjaabcc", true);
            bad_after_meeting("aabcciii", false);
        }
    }

    fn test_incr(s: &str, i: usize, expect: &str) {
        let mut bytes = s.bytes().collect::<Vec<_>>();
        increment(&mut bytes, i);

        assert_eq!(String::from_utf8(bytes).unwrap(), expect);
    }

    fn test_valid(s: &str, expect: bool) {
        let bytes = s.bytes().collect::<Vec<_>>();
        assert_eq!(valid(&bytes), expect);
    }
}

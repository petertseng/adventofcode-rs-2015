// Considered using regex:
// https://github.com/rust-lang/regex
// However, note that it doesn't handle backreferences,
// so that excludes all of nice2 and 2/3 rules for nice1.

fn nice1(s: &str) -> bool {
    let mut cprev = '\0';
    let mut pair = false;
    let mut vowels = 0u32;
    for c in s.chars() {
        if c == cprev {
            pair = true;
        }
        if c == 'a' || c == 'e' || c == 'i' || c == 'o' || c == 'u' {
            vowels += 1;
        }
        // seems we can't add 1 to char without doing some casts,
        // which precludes doing (cprev == 'a' || cprev == ...) && c + 1 == cprev,
        // so just compare both.
        for (a, b) in &[('a', 'b'), ('c', 'd'), ('p', 'q'), ('x', 'y')] {
            if cprev == *a && c == *b {
                return false;
            }
        }
        cprev = c;
    }
    pair && vowels >= 3
}

fn nice2(s: &str) -> bool {
    let mut c1 = '\0';
    let mut c2 = '\0';

    let mut aba = false;
    let mut two_pair = false;
    let mut pairs = std::collections::HashMap::<(char, char), usize>::new();

    for (i, c) in s.chars().enumerate() {
        if c == c1 {
            aba = true;
            if two_pair {
                return true;
            }
        }

        let pair = (c2, c);
        let prev_index = *pairs.entry(pair).or_insert(i);
        if prev_index + 1 < i {
            two_pair = true;
            if aba {
                return true;
            }
        }

        c1 = c2;
        c2 = c;
    }

    false
}

fn main() {
    let input = adventofcode::read_input_file();

    println!("{}", input.lines().filter(|pw| nice1(pw)).count());
    println!("{}", input.lines().filter(|pw| nice2(pw)).count());
}

#[cfg(test)]
mod tests {
    use adventofcode::tests;

    tests! {
        nice1 {
            basic_nice1("ugknbfddgicrmopn");
            minimal_nice1("aaa");
        }
        naughty1 {
            no_double_letter("jchzalrnumimnmhp");
            bad_pair("haegwjzuvuyypxyu");
            only_one_vowel("dvszwmarrgswjxmb");
        }
        nice2 {
            basic_nice2("qjhvhtzxzqqjkmpb");
            almost_minimal_nice2("xxyxx");
            same_letter_aba_ok("xyaaaxy");
            aaaa_ok_for_pairs("aaaa");
        }
        naughty2 {
            no_aba("uurcxstgmygtbstg");
            no_pair("ieodomkazucvgmuy");
        }
    }

    fn nice1(s: &str) {
        assert!(super::nice1(s));
    }

    fn naughty1(s: &str) {
        assert!(!super::nice1(s));
    }

    fn nice2(s: &str) {
        assert!(super::nice2(s));
    }

    fn naughty2(s: &str) {
        assert!(!super::nice2(s));
    }
}

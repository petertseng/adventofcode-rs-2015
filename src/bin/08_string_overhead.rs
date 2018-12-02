// Could have done this with regex,
// just didn't want the extra dependency.

fn literal_overhead(s: &str) -> usize {
    let mut overhead = 2; // two double quotes
    let mut was_slash = false;
    for c in s.chars() {
        if was_slash {
            if c == 'x' {
                overhead += 3;
            } else if c == '\"' || c == '\\' {
                overhead += 1;
            } else {
                panic!("Unknown escape sequence \\{}", c);
            }
            was_slash = false;
            continue;
        }
        was_slash = c == '\\';
    }
    overhead
}

fn encoded_overhead(s: &str) -> usize {
    2 + s.chars().filter(|&c| c == '"' || c == '\\').count()
}

fn main() {
    let input = adventofcode::read_input_file();
    println!("{}", input.lines().map(literal_overhead).sum::<usize>());
    println!("{}", input.lines().map(encoded_overhead).sum::<usize>());
}

#[cfg(test)]
mod tests {
    use super::*;
    use adventofcode::tests;

    tests! {
        test1 {
            empty1(r#""""#, 2);
            abc1(r#""abc""#, 2);
            escaped_quote(r#""aaa\"aaa""#, 3);
            escaped_hexadecimal(r#""\x27""#, 5);
        }
        test2 {
            empty2(r#""""#, 4);
            abc2(r#""""#, 4);
            slash_and_quote(r#""aaa\"aaa""#, 6);
            slash_only(r#""\x27""#, 5);
        }
    }

    fn test1(s: &str, expect: usize) {
        assert_eq!(literal_overhead(s), expect);
    }

    fn test2(s: &str, expect: usize) {
        assert_eq!(encoded_overhead(s), expect);
    }
}

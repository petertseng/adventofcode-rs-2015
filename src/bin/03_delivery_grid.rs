use std::collections::HashSet;

#[derive(Clone, Default)]
struct Santa(i32, i32);

impl Santa {
    fn new() -> Self {
        Self(0, 0)
    }

    fn step(&mut self, c: char) -> (i32, i32) {
        let Self(ref mut y, ref mut x) = self;
        match c {
            '^' => *y += 1,
            '>' => *x += 1,
            'v' => *y -= 1,
            '<' => *x -= 1,
            '\n' => (),
            _ => panic!("Unknown character {}", c),
        }
        (self.0, self.1)
    }
}

fn run(s: &str, num_santas: usize) -> usize {
    let mut houses: HashSet<_> = [(0, 0)].iter().cloned().collect();
    let mut santas = vec![Santa::new(); num_santas];

    for (i, c) in s.chars().enumerate() {
        let santa = &mut santas[i % num_santas];
        houses.insert(santa.step(c));
    }

    houses.len()
}

fn main() {
    let input = adventofcode::read_input_file();
    println!("{}", run(&input, 1));
    println!("{}", run(&input, 2));
}

#[cfg(test)]
mod tests {
    use super::*;
    use adventofcode::tests;

    tests! {
        test1 {
            step(">", 2);
            square("^>v<", 4);
            back_and_forth("^v^v^v^v^v", 2);
        }
        test2 {
            step_for_each("^v", 3);
            two_back_and_forth("^>v<", 3);
            different_directions("^v^v^v^v^v", 11);
        }
    }

    fn test1(s: &str, expect: usize) {
        assert_eq!(run(s, 1), expect);
    }

    fn test2(s: &str, expect: usize) {
        assert_eq!(run(s, 2), expect);
    }
}

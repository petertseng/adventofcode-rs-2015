struct Combinations<'a, T> {
    k: usize,
    is: Vec<usize>,
    xs: &'a [T],
}

impl<'a, T> Combinations<'a, T> {
    fn new(xs: &'a [T], k: usize) -> Self {
        let is = (0..k).collect();
        Self { is, k, xs }
    }

    fn current(&self) -> Option<Vec<&'a T>> {
        if self.is[self.k - 1] < self.xs.len() {
            Some(self.is.iter().map(|&i| &self.xs[i]).collect())
        } else {
            None
        }
    }

    fn advance(&mut self) {
        let mut movable = self.k - 1;

        // Last index is movable if it won't fall off the end.
        if movable > 0 && self.is[movable] + 1 >= self.xs.len() {
            // If last index isn't movable, try to find another.
            // This would be any index that doesn't have another to its immediate right.
            movable -= 1;
            while movable > 0 && self.is[movable] + 1 >= self.is[movable + 1] {
                movable -= 1;
            }
        }
        // Move that index, and all indices beyond it to its immediate right.
        // For an example list of size 6:
        // 0 1 2 -> 0 1 3
        // 0 1 5 -> 0 2 3
        // 0 4 5 -> 1 2 3
        // 3 4 5 -> 4 5 6 (and will return None next time)
        self.is[movable] += 1;
        for i in (movable + 1)..self.k {
            self.is[i] = self.is[i - 1] + 1;
        }
    }
}

impl<'a, T> Iterator for Combinations<'a, T> {
    type Item = Vec<&'a T>;

    fn next(&mut self) -> Option<Self::Item> {
        self.current().map(|v| {
            self.advance();
            v
        })
    }
}

fn best_partition(packages: &[u32], groups: u8) -> Option<u64> {
    let sum: u32 = packages.iter().sum();
    if sum % u32::from(groups) != 0 {
        return None;
    }
    let per_group = sum / u32::from(groups);

    for n in 1..packages.len() {
        let mut winners = Vec::new();
        for group in Combinations::new(packages, n) {
            if group.iter().cloned().sum::<u32>() != per_group {
                continue;
            }

            let qe = group.iter().fold(1_u64, |a, &&x| a * u64::from(x));
            winners.push((qe, group.clone()));
        }

        if !winners.is_empty() {
            winners.sort_unstable();
            for (qe, winner) in winners {
                let without =
                    array_minus(packages, &winner.into_iter().cloned().collect::<Vec<_>>());
                if can_partition(&without, groups - 1, per_group) {
                    return Some(qe);
                }
            }
        }
    }

    None
}

fn can_partition(packages: &[u32], groups: u8, per_group: u32) -> bool {
    if groups == 0 {
        return packages.is_empty();
    }

    for n in 1..=packages.len() {
        for group in Combinations::new(packages, n) {
            if group.iter().cloned().sum::<u32>() != per_group {
                continue;
            }
            let without = array_minus(packages, &group.into_iter().cloned().collect::<Vec<_>>());
            if can_partition(&without, groups - 1, per_group) {
                return true;
            }
        }
    }

    false
}

fn array_minus<T: Clone + Eq + std::hash::Hash>(v: &[T], sub: &[T]) -> Vec<T> {
    let mut freq = std::collections::HashMap::new();
    for sub_e in sub {
        *freq.entry(sub_e).or_insert(0) += 1;
    }

    v.iter()
        .filter(|e| match freq.get_mut(e) {
            Some(n) => {
                if *n == 0 {
                    true
                } else {
                    *n -= 1;
                    false
                }
            }
            None => true,
        })
        .cloned()
        .collect()
}

fn main() {
    let packages: Vec<_> = adventofcode::read_input_file()
        .lines()
        .map(|line| line.parse::<u32>().expect("can't parse integer"))
        .collect();

    for &n in &[3, 4] {
        match best_partition(&packages, n) {
            Some(qe) => println!("{}", qe),
            None => println!("impossible!"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use adventofcode::tests;

    tests! {
        test_minus {
            sub_something(&[1, 2], &[2], &[1]);
            sub_nothing(&[1, 2], &[], &[1, 2]);
            sub_from_nothing(&[], &[1], &[]);
            sub_only_a_few(&[1, 2, 2], &[2], &[1, 2]);
            sub_more(&[1, 2, 2], &[2, 2, 2], &[1]);
        }
        test_can_partition {
            all_eq(&[1, 1], 2, 1, true);
            one_and_two(&[2, 1, 1], 2, 2, true);
            two_and_two(&[1, 1, 1, 1], 2, 2, true);
            two_and_two_unequal(&[1, 3, 7, 9], 2, 10, true);
            nope(&[11, 9], 2, 10, false);
            nope2(&[4, 7, 4, 5], 2, 10, false);
        }
    }

    fn test_minus(v: &[i32], sub: &[i32], expect: &[i32]) {
        assert_eq!(array_minus(v, sub), expect);
    }

    fn test_can_partition(v: &[u32], groups: u8, per_group: u32, expect: bool) {
        assert_eq!(can_partition(v, groups, per_group), expect);
    }
}

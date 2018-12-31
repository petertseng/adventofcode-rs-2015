// Euler-Mascheroni constant
#[allow(clippy::unreadable_literal)]
const GAMMA: f64 = 0.5772156649015329;

fn gifts(house: usize, limit: Option<usize>) -> usize {
    let mut given = 0;
    for i in 1..house {
        if i * i > house {
            break;
        }
        if house % i != 0 {
            continue;
        }
        let factor1 = i;
        let factor2 = house / i;
        if limit.map_or(true, |l| factor2 <= l) {
            given += factor1;
        }
        if factor1 != factor2 && limit.map_or(true, |l| factor1 <= l) {
            given += factor2;
        }
    }
    given
}

fn house_upper_bound(target: usize, limit: Option<usize>) -> usize {
    // smallest greater factorial:
    let mut bound = 2;
    let mut n = 2;
    while gifts(bound, limit) < target {
        n += 1;
        bound *= n;
    }

    // Decrease each factor
    for from in (2..=n).rev() {
        let bound_without = bound / from;
        for to in 1..from {
            let bound_with = bound_without * to;
            if gifts(bound_with, limit) >= target {
                bound = bound_with;
                break;
            }
        }
    }

    bound
}

fn house_lower_bound(target: usize) -> usize {
    // Robin's inequality:
    // \sigma(n) < e^\gamma n \log \log n
    // n \log \log n > \frac{T}{e^\gamma}
    // lower bound, so \log \log n can be increased to \log \log T
    // n > \frac{T}{e^\gamma \log \log T}

    let n = (target as f64 / (GAMMA.exp() * (target as f64).ln().ln())).ceil() as usize;

    // TODO: This was approximate (since we changed an n for T) and we can do better,
    // but the improvement is unlikely to be significant (704242 -> 733346, 641725 -> 668446).

    // Robin's inequality doesn't hold for n <= 5040.
    if n > 5040 {
        n
    } else {
        1
    }
}

fn first_house(target: usize, per_elf: u8, limit: Option<usize>) -> usize {
    let elf_factor_needed = target / usize::from(per_elf);
    let lower_bound = house_lower_bound(elf_factor_needed);
    let upper_bound = house_upper_bound(elf_factor_needed, limit);
    let mut presents = vec![usize::from(per_elf); upper_bound + 1 - lower_bound];

    for elf in 1..=upper_bound {
        let default_mult = upper_bound / elf;
        let max_mult = limit.map_or(default_mult, |l| std::cmp::min(l, default_mult));

        let skipped = if elf < lower_bound {
            (lower_bound - 1) / elf
        } else {
            0
        };

        for mult in (skipped + 1)..=max_mult {
            let house = elf * mult;
            presents[house - lower_bound] += elf;
            if mult == 1 && presents[house - lower_bound] >= elf_factor_needed {
                return house;
            }
        }
    }

    unreachable!("upper bound is wrong");
}

fn main() {
    let target = std::env::args()
        .nth(1)
        .unwrap_or_else(adventofcode::read_input_file)
        .parse::<usize>()
        .expect("not an integer");

    println!("{}", first_house(target, 10, None));
    println!("{}", first_house(target, 11, Some(50)));
}

#[cfg(test)]
mod tests {
    use super::*;
    use adventofcode::tests;

    tests! {
        test_gifts {
            // 1 + 2 + 3 + 6 + 9 + 18
            no_limit(18, None, 39);
            // 2 + 3 + 6 + 9 + 18
            limit(18, Some(10), 38);
            // same as above because 18 is elf 2's 9th house.
            limit_exact(18, Some(9), 38);
        }
    }

    fn test_gifts(house: usize, limit: Option<usize>, expect: usize) {
        assert_eq!(gifts(house, limit), expect);
    }
}

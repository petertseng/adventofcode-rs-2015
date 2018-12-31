// Euler-Mascheroni constant
#[allow(clippy::unreadable_literal)]
const GAMMA: f64 = 0.5772156649015329;

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
    let upper_bound = elf_factor_needed;
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

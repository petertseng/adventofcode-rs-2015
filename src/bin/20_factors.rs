fn first_house(target: usize, per_elf: u8, limit: Option<usize>) -> usize {
    let elf_factor_needed = target / usize::from(per_elf);
    let lower_bound = 1;
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

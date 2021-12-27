use std::collections::HashMap;

const PRIMES: [u32; 15] = [47, 43, 41, 37, 31, 29, 23, 19, 17, 13, 11, 7, 5, 3, 2];

// askalski's tip:
// https://www.reddit.com/r/adventofcode/comments/po1zel/comment/hd1esc2
fn sum_exceeds(goal: u32, primes: &[u32], cache: &mut HashMap<(u32, u32), u32>) -> u32 {
    if primes.is_empty() {
        return goal;
    }

    let prime = primes[0];
    if let Some(&cached) = cache.get(&(goal, prime)) {
        return cached;
    }

    let mut best = sum_exceeds(goal, &primes[1..], cache);

    let mut prime_power = 1;
    let mut prime_sum = 1;

    while prime_sum < goal {
        prime_power *= prime;
        prime_sum += prime_power;

        // subproblem: ceil(goal/prime_sum) using only primes less than prime
        let subgoal = (goal + prime_sum - 1) / prime_sum;
        best = std::cmp::min(
            best,
            prime_power * sum_exceeds(subgoal, &primes[1..], cache),
        );
    }

    cache.insert((goal, prime), best);

    best
}

fn good2(target: u32, house: u32) -> bool {
    11 * (1..50)
        .into_iter()
        .map(|d| if house % d == 0 { house / d } else { 0 })
        .sum::<u32>()
        >= target
}

fn main() {
    let target = std::env::args()
        .nth(1)
        .unwrap_or_else(adventofcode::read_input_file)
        .parse::<u32>()
        .expect("not an integer");

    let mut cache = HashMap::new();
    let house1 = sum_exceeds(target / 10, &PRIMES, &mut cache);
    println!("{}", house1);
    let lower_bound = if good2(target, house1) { 0 } else { house1 };
    println!(
        "{}",
        (lower_bound..target)
            .into_iter()
            .step_by(2 * 3 * 5 * 7)
            .find(|&x| good2(target, x))
            .unwrap()
    );
}

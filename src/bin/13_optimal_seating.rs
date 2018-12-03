fn main() {
    use std::collections::{HashMap, HashSet};

    let input = adventofcode::read_input_file();

    let mut dists = HashMap::<(&str, &str), i32>::new();
    let mut places = HashSet::<&str>::new();

    for line in input.lines() {
        let words: Vec<&str> = line.split_whitespace().collect();
        match words[..] {
            [a, "would", gain_or_lose, n, "happiness", "units", "by", "sitting", "next", "to", b_with_period] =>
            {
                let abs_dist = n.parse::<i32>().expect("not an integer");
                let b = b_with_period.split_terminator('.').next().unwrap();
                let dist = match gain_or_lose {
                    "gain" => abs_dist,
                    "lose" => -abs_dist,
                    _ => panic!("Unknown word {}", gain_or_lose),
                };
                *dists.entry((a, b)).or_insert(0) += dist;
                *dists.entry((b, a)).or_insert(0) += dist;
                places.insert(a);
                places.insert(b);
            }
            _ => panic!("Unrecognized line {:?}", line),
        }
    }

    let mut max_cycle = std::i32::MIN;
    let mut max_path = std::i32::MIN;

    let mut places: Vec<_> = places.iter().collect();

    adventofcode::each_perm(&mut places, |path| {
        let dist = path
            .iter()
            .zip(path.iter().skip(1))
            .map(|(&&a, &&b)| dists[&(a, b)])
            .sum::<i32>();

        if dist > max_path {
            max_path = dist;
        }

        let dist = dist + dists[&(*path[0], *path[path.len() - 1])];

        if dist > max_cycle {
            max_cycle = dist;
        }
    });

    println!("{}", max_cycle);
    println!("{}", max_path);
}

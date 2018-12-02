fn main() {
    use std::collections::{HashMap, HashSet};

    let input = adventofcode::read_input_file();

    let mut dists = HashMap::<(&str, &str), u32>::new();
    let mut places = HashSet::<&str>::new();

    for line in input.lines() {
        let words: Vec<&str> = line.split_whitespace().collect();
        match words[..] {
            [a, "to", b, "=", dist] => {
                let dist = dist.parse::<u32>().expect("not an integer");
                dists.insert((a, b), dist);
                dists.insert((b, a), dist);
                places.insert(a);
                places.insert(b);
            }
            _ => panic!("Unrecognized line {:?}", line),
        }
    }

    let mut min_dist = std::u32::MAX;
    let mut max_dist = 0;

    let mut places: Vec<_> = places.iter().collect();

    adventofcode::each_perm(&mut places, |path| {
        let dist = path
            .iter()
            .zip(path.iter().skip(1))
            .map(|(&&a, &&b)| dists[&(a, b)])
            .sum::<u32>();
        if dist < min_dist {
            min_dist = dist;
        }
        if dist > max_dist {
            max_dist = dist;
        }
    });

    println!("{}", min_dist);
    println!("{}", max_dist);
}

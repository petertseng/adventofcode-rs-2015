use std::collections::HashMap;

fn ways(containers: &[i32], weight: i32, min_index: usize, num_used: u32) -> HashMap<u32, u32> {
    if weight == 0 {
        let mut hm = HashMap::new();
        hm.insert(num_used, 1);
        return hm;
    }
    if min_index >= containers.len() {
        return HashMap::new();
    }
    if weight < 0 {
        return HashMap::new();
    }

    let ways_without = ways(containers, weight, min_index + 1, num_used);
    let mut ways_with = ways(
        containers,
        weight - containers[min_index],
        min_index + 1,
        num_used + 1,
    );

    for (&k, v) in ways_without.iter() {
        *ways_with.entry(k).or_insert(0) += v;
    }

    ways_with
}

fn main() {
    let mut containers =
        adventofcode::read_input_lines(|line| line.parse::<i32>().expect("can't parse integer"));
    containers.sort_unstable();
    containers.reverse();

    let ways = ways(&containers, 150, 0, 0);

    println!("{}", ways.values().sum::<u32>());
    println!("{}", ways[ways.keys().min().unwrap()]);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn make_25() {
        let mut hm = HashMap::new();
        hm.insert(3, 1);
        hm.insert(2, 3);
        assert_eq!(ways(&[20, 15, 10, 5, 5], 25, 0, 0), hm);
    }
}

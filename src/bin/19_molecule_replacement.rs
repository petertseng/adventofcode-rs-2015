fn main() {
    let input = adventofcode::read_input_file();
    let mut rules = std::collections::HashMap::new();
    let mut molecule = "";

    for line in input.lines() {
        let words: Vec<_> = line.split_whitespace().collect();
        match &words[..] {
            [l, "=>", r] => rules.entry(*l).or_insert_with(Vec::new).push(*r),
            [m] => molecule = m,
            [] => (),
            _ => panic!("Unknown line {}", line),
        }
    }

    let rules = rules;
    let molecule = molecule;
    let max_e = rules["e"]
        .iter()
        .map(|r| r.chars().filter(|&c| ('A'..='Z').contains(&c)).count())
        .max();

    let mut seen = std::collections::HashSet::new();
    for (l, rs) in rules.iter() {
        for (i, _) in molecule.match_indices(l) {
            for r in rs.iter() {
                let mut s = String::new();
                s.push_str(&molecule[0..i]);
                s.push_str(r);
                s.push_str(&molecule[(i + l.len())..molecule.len()]);
                seen.insert(s);
            }
        }
    }
    println!("{}", seen.len());

    let elements = molecule
        .chars()
        .filter(|&c| ('A'..='Z').contains(&c))
        .count();
    let rn = molecule.matches("Rn").count();
    let y = molecule.matches('Y').count();
    let ar = molecule.matches("Ar").count();
    assert_eq!(rn, ar);

    println!("{}", elements - (max_e.unwrap_or(0) - 1) - rn - ar - y * 2);
}

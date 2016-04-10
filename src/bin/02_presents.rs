fn paper(dim: &[u32]) -> u32 {
    let sides = vec![dim[0] * dim[1], dim[0] * dim[2], dim[1] * dim[2]];
    sides.iter().sum::<u32>() * 2 + sides.iter().min().unwrap()
}

fn ribbon(dim: &[u32]) -> u32 {
    (dim[0] + dim[1]) * 2 + dim.iter().product::<u32>()
}

fn main() {
    let dimensions = adventofcode::read_input_lines(|line| {
        let mut dimensions = adventofcode::numbers(line);
        dimensions.sort_unstable();
        dimensions
    });

    let sum_by = |f: fn(&[u32]) -> u32| dimensions.iter().map(|v| f(&v)).sum::<u32>();

    println!("{}", sum_by(paper));
    println!("{}", sum_by(ribbon));
}

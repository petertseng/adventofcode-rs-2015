fn add_score(base: &[i32], v: &[i32], n: u8) -> Vec<i32> {
    base.iter()
        .zip(v.iter())
        .map(|(&a, &b)| a + b * i32::from(n))
        .collect()
}

fn score(components: &[i32]) -> (u32, i32) {
    let mut product = 1;
    let calories = components[components.len() - 1];
    for &score in &components[0..(components.len() - 1)] {
        if score <= 0 {
            return (0, calories);
        }
        product *= score;
    }
    (product as u32, calories)
}

fn cookies(
    ingredients: &[Vec<i32>],
    remain: u8,
    score_without: &[i32],
    best: &mut u32,
    best500: &mut u32,
) {
    if ingredients.is_empty() {
        panic!("Shouldn't be called with no ingredients");
    }
    let ingredient = &ingredients[0];

    if ingredients.len() == 1 {
        let score_with = add_score(score_without, ingredient, remain);
        let (score, calories) = score(&score_with);
        if calories == 500 && score > *best500 {
            *best500 = score;
        }
        if score > *best {
            *best = score;
        }
        return;
    }

    for i in 0..=remain {
        let score_with = add_score(score_without, ingredient, i);
        cookies(
            &ingredients[1..ingredients.len()],
            remain - i,
            &score_with,
            best,
            best500,
        );
    }
}

fn main() {
    let ingredients = adventofcode::read_input_lines(adventofcode::numbers);
    let len = ingredients[0].len();

    let mut best = 0;
    let mut best500 = 0;

    cookies(&ingredients, 100, &vec![0; len], &mut best, &mut best500);

    println!("{}", best);
    println!("{}", best500);
}

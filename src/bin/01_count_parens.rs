fn main() {
    let mut floor = 0;
    let mut first_basement = None;

    for (i, c) in adventofcode::read_input_file().chars().enumerate() {
        if c == '(' {
            floor += 1;
        } else if c == ')' {
            floor -= 1;
            if floor == -1 && first_basement.is_none() {
                first_basement = Some(i + 1);
            }
        }
    }

    println!("{}", floor);
    match first_basement {
        Some(index) => println!("{}", index),
        None => println!("never!"),
    }
}

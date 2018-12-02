enum Command {
    TurnOn,
    TurnOff,
    Toggle,
}

fn parse_coord(s: &str) -> (usize, usize) {
    let coords: Vec<_> = s
        .split(',')
        .map(|d| d.parse::<usize>().expect("can't parse integer"))
        .collect();
    match coords.as_slice() {
        [a, b] => (*a, *b),
        _ => panic!("Unrecognized coordinate {:?}", s),
    }
}

fn sum(a: &[[u32; 1000]; 1000]) -> u32 {
    a.iter().map(|row| row.iter().sum::<u32>()).sum::<u32>()
}

fn main() {
    let commands = adventofcode::read_input_lines(|l| {
        let words: Vec<&str> = l.split_whitespace().collect();
        let (command, command_len) = match words[0..2] {
            ["turn", "on"] => (Command::TurnOn, 2),
            ["turn", "off"] => (Command::TurnOff, 2),
            ["toggle", _] => (Command::Toggle, 1),
            _ => panic!("Unrecognized command {:?}", words),
        };
        let (c1, c2) = match words[command_len..] {
            [a, "through", b] => (parse_coord(a), parse_coord(b)),
            _ => panic!("Unrecognized args {:?}", words),
        };
        (command, c1, c2)
    });

    let mut on_off = [[0u32; 1000]; 1000];
    let mut brightness = [[0u32; 1000]; 1000];

    for (command, (start_y, start_x), (end_y, end_x)) in commands {
        for y in start_y..=end_y {
            for x in start_x..=end_x {
                // I would have preferred to express these as functions,
                // but functions do not have compatible types with each other.
                // Luckily the compiler seems smart enough to optimise these.
                match command {
                    Command::TurnOn => {
                        on_off[y][x] = 1;
                        brightness[y][x] += 1;
                    }
                    Command::TurnOff => {
                        on_off[y][x] = 0;
                        if brightness[y][x] > 0 {
                            brightness[y][x] -= 1;
                        }
                    }
                    Command::Toggle => {
                        on_off[y][x] = 1 - on_off[y][x];
                        brightness[y][x] += 2;
                    }
                }
            }
        }
    }

    println!("{}", sum(&on_off));
    println!("{}", sum(&brightness));
}

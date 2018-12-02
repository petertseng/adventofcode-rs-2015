use std::collections::HashMap;

enum Input<'a> {
    Num(u16),
    Wire(&'a str),
}

impl<'a> Input<'a> {
    fn new(s: &'a str) -> Self {
        if let Ok(i) = s.parse::<u16>() {
            Input::Num(i)
        } else {
            Input::Wire(s)
        }
    }

    fn value(&self, gates: &'a HashMap<&str, Gate>, cache: &mut HashMap<&'a str, u16>) -> u16 {
        match *self {
            Input::Num(i) => i,
            Input::Wire(s) => value(gates, s, cache),
        }
    }
}

#[derive(Clone, Copy)]
enum Binop {
    And,
    Or,
    LShift,
    RShift,
}

impl<'a> From<&'a str> for Binop {
    fn from(s: &str) -> Binop {
        match s {
            "AND" => Binop::And,
            "OR" => Binop::Or,
            "LSHIFT" => Binop::LShift,
            "RSHIFT" => Binop::RShift,
            _ => panic!("Unknown op {}", s),
        }
    }
}

enum Gate<'a> {
    Unary(Input<'a>, bool),
    Binary(Binop, Input<'a>, Input<'a>),
}

fn value<'a>(
    gates: &'a HashMap<&str, Gate>,
    id: &'a str,
    cache: &mut HashMap<&'a str, u16>,
) -> u16 {
    if let Some(val) = cache.get(&id) {
        return *val;
    }

    let ans = match &gates[id] {
        Gate::Unary(input, invert) => {
            let v1 = input.value(gates, cache);
            if *invert {
                !v1
            } else {
                v1
            }
        }
        Gate::Binary(op, lhs, rhs) => {
            let v1 = lhs.value(gates, cache);
            let v2 = rhs.value(gates, cache);
            match op {
                Binop::And => v1 & v2,
                Binop::Or => v1 | v2,
                Binop::LShift => v1 << v2,
                Binop::RShift => v1 >> v2,
            }
        }
    };

    cache.insert(id, ans);
    ans
}

fn circuit(input: &str) -> HashMap<&str, Gate> {
    let mut gates = HashMap::new();

    for line in input.lines() {
        let words: Vec<&str> = line.split_whitespace().collect();
        let (gate, dest) = match words[..] {
            [a, "->", b] => (Gate::Unary(Input::new(a), false), b),
            ["NOT", a, "->", b] => (Gate::Unary(Input::new(a), true), b),
            [a, op, b, "->", c] => (
                Gate::Binary(Binop::from(op), Input::new(a), Input::new(b)),
                c,
            ),
            _ => panic!("Unrecognized gate {:?}", words),
        };
        gates.insert(dest, gate);
    }

    gates
}

fn main() {
    let input = adventofcode::read_input_file();
    let mut gates = circuit(&input);

    let part1 = value(&gates, "a", &mut HashMap::new());
    println!("{}", part1);
    gates.insert("b", Gate::Unary(Input::Num(part1), false));
    println!("{}", value(&gates, "a", &mut HashMap::new()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple() {
        let s = vec![
            "123 -> x",
            "456 -> y",
            "x AND y -> d",
            "x OR y -> e",
            "x LSHIFT 2 -> f",
            "y RSHIFT 2 -> g",
            "NOT x -> h",
            "NOT y -> i",
        ]
        .join("\n");
        let gates = circuit(&s);
        let exps = vec![
            ("d", 72),
            ("e", 507),
            ("f", 492),
            ("g", 114),
            ("h", 65412),
            ("i", 65079),
            ("x", 123),
            ("y", 456),
        ];
        let wrongs: Vec<_> = exps
            .iter()
            .filter_map(|(wire, exp)| {
                let obs = value(&gates, wire, &mut HashMap::new());
                if *exp != obs {
                    Some((wire, "expected", *exp, "observed", obs))
                } else {
                    None
                }
            })
            .collect();
        assert!(wrongs.is_empty(), "wrong wires: {:?}", wrongs);
    }
}

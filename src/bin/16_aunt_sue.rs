fn main() {
    use std::cmp::Ordering;

    let mut real_sue = std::collections::HashMap::new();
    real_sue.insert("children", (3, Ordering::Equal));
    real_sue.insert("cats", (7, Ordering::Greater));
    real_sue.insert("samoyeds", (2, Ordering::Equal));
    real_sue.insert("pomeranians", (3, Ordering::Less));
    real_sue.insert("akitas", (0, Ordering::Equal));
    real_sue.insert("vizslas", (0, Ordering::Equal));
    real_sue.insert("goldfish", (5, Ordering::Less));
    real_sue.insert("trees", (3, Ordering::Greater));
    real_sue.insert("cars", (2, Ordering::Equal));
    real_sue.insert("perfumes", (1, Ordering::Equal));
    let real_sue = real_sue;

    let mut goods1 = Vec::new();
    let mut goods2 = Vec::new();

    let input = adventofcode::read_input_file();
    for line in input.lines() {
        let attrs: Vec<_> = line.split(|c| c == ':' || c == ' ' || c == ',').collect();
        let sue = attrs[1].parse::<u32>().expect("sue ID not a number");
        let num_attrs = (attrs.len() - 2) / 4;
        let mut good1 = true;
        let mut good2 = true;
        for i in 0..num_attrs {
            let attr_name = attrs[i * 4 + 3];
            let attr_val = attrs[i * 4 + 5]
                .parse::<u32>()
                .expect("attr_val not a value");
            let (expected, cmp_result) = real_sue[attr_name];
            if attr_val != expected {
                good1 = false;
            }
            if attr_val.cmp(&expected) != cmp_result {
                good2 = false;
            }
        }
        if good1 {
            goods1.push(sue);
        }
        if good2 {
            goods2.push(sue);
        }
    }

    for g in goods1 {
        println!("{}", g);
    }
    for g in goods2 {
        println!("{}", g);
    }
}

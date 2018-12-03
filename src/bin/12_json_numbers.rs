enum Nestable {
    Object { red: bool },
    Array,
}

// Error for possible but malformed input strings, like } or ]
//
// expect/panic for invalid states that no input string,
// well-formed or malformed, should ever cause

#[derive(PartialEq, Debug)]
enum Error {
    TooManySums(Vec<i32>),
    NotAnInteger(<i32 as std::str::FromStr>::Err),
    UnexpectedClose(char),
    UnexpectedChar(char),
}

fn parse_number(
    num_buf: &mut Vec<char>,
    total: &mut i32,
    objs: &mut Vec<i32>,
) -> Result<(), Error> {
    if num_buf.is_empty() {
        return Ok(());
    }
    let s: String = num_buf.iter().collect();
    let v = s.parse::<i32>().map_err(Error::NotAnInteger)?;
    let prev = objs.last_mut().expect("always 1+ objs");
    *total += v;
    *prev += v;
    num_buf.clear();
    Ok(())
}

fn sums(json: &str) -> Result<(i32, i32), Error> {
    let mut nestables = Vec::new();
    let mut objs = vec![0];
    let mut total = 0;
    let mut in_string = false;

    let mut num_buf = Vec::new();

    let mut red_start = 0;
    let mut red_good = 0;

    for (i, c) in json.chars().enumerate() {
        if in_string {
            match c {
                'r' => {
                    if red_start + 1 == i {
                        red_good += 1
                    }
                }
                'e' => {
                    if red_start + 2 == i {
                        red_good += 1
                    }
                }
                'd' => {
                    if red_start + 3 == i {
                        red_good += 1
                    }
                }
                '"' => {
                    let was_obj = matches!(nestables.last(), Some(Nestable::Object { .. }));
                    // Assumes that there is never a key "red"
                    if was_obj && red_good == 3 && red_start + 4 == i {
                        nestables.pop();
                        nestables.push(Nestable::Object { red: true });
                    }
                    in_string = false;
                }
                _ => (),
            }
            continue;
        }

        // not in_string
        match c {
            '{' => {
                nestables.push(Nestable::Object { red: false });
                objs.push(0);
            }
            '}' => {
                parse_number(&mut num_buf, &mut total, &mut objs)?;
                let was_red = match nestables.pop() {
                    Some(Nestable::Object { red: r }) => r,
                    _ => return Err(Error::UnexpectedClose(c)),
                };
                let obj1 = objs.pop().expect("always 2+ objs when closing obj");
                let obj2 = objs.last_mut().expect("always 2+ objs when closing obj");
                if !was_red {
                    *obj2 += obj1;
                }
            }
            '[' => nestables.push(Nestable::Array),
            ']' => {
                parse_number(&mut num_buf, &mut total, &mut objs)?;
                match nestables.pop() {
                    Some(Nestable::Array) => (),
                    _ => return Err(Error::UnexpectedClose(c)),
                };
            }
            '"' => {
                in_string = true;
                red_good = 0;
                red_start = i;
            }
            ',' => parse_number(&mut num_buf, &mut total, &mut objs)?,
            '-' | '0'..='9' => num_buf.push(c),
            ':' | '\n' => (),
            _ => return Err(Error::UnexpectedChar(c)),
        }
    }

    match &objs[..] {
        &[ans] => Ok((total, ans)),
        &[] => unreachable!("always 1+ sums"),
        v => Err(Error::TooManySums(v.to_vec())),
    }
}

fn main() {
    let input = adventofcode::read_input_file();
    match sums(&input) {
        Ok((sum1, sum2)) => {
            println!("{}", sum1);
            println!("{}", sum2);
        }
        Err(s) => println!("{:?}", s),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use adventofcode::tests;

    tests! {
        test1 {
            arr1(r#"[1,2,3]"#, 6);
            obj1(r#"{"a":2,"b":4}"#, 6);
            nested_arr(r#"[[[3]]]"#, 3);
            nested_obj(r#"{"a":{"b":4},"c":-1}"#, 3);
            arr_in_obj(r#"{"a":[-1,1]}"#, 0);
            obj_in_arr(r#"[-1,{"a":1}]"#, 0);
            empty_arr("[]", 0);
            empty_obj("[]", 0);
        }
        test2 {
            ignore_red(r#"[1,{"c":"red","b":2},3]"#, 4);
            ignore_all(r#"{"d":"red","e":[1,2,3,4],"f":5}"#, 0);
            ok_in_array(r#"[1,"red",5]"#, 6);
        }
    }

    fn test1(s: &str, expect: i32) {
        assert_eq!(sums(s), Ok((expect, expect)));
    }

    fn test2(s: &str, expect: i32) {
        // This might be a good time for a Result test,
        // but I don't have support for it in the macro.
        assert_eq!(sums(s).unwrap().1, expect);
    }
}

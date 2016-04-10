use std::env;
use std::fs;

pub fn read_input_file() -> String {
    let filename = env::args()
        .nth(1)
        .unwrap_or_else(|| "/dev/stdin".to_string());
    fs::read_to_string(filename).expect("couldn't read file")
}

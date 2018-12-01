fn main() {
    let input = std::env::args()
        .nth(1)
        .unwrap_or_else(adventofcode::read_input_file);
    let mut zeroes = 5;
    let mut digits = 0;
    let mut md5 = adventofcode::md5::MD5::new();

    let mut input_bytes: Vec<u8> = input.bytes().collect();
    let keylen = input_bytes.len();
    input_bytes.push(b'0');

    for i in 1.. {
        for neg_j in 0..digits {
            let j = keylen + digits - 1 - neg_j;
            if input_bytes[j] == b'9' {
                input_bytes[j] = b'0';
            } else {
                input_bytes[j] += 1;
                break;
            }
        }
        if input_bytes[keylen] == b'0' {
            if digits > 0 {
                input_bytes.push(b'0');
            }
            input_bytes[keylen] = b'1';
            digits += 1;
        }

        if md5.digest_has_zeroes(&input_bytes, zeroes) {
            println!("{}", i);
            if zeroes >= 6 {
                break;
            }
            zeroes += 1;
        }
    }
}

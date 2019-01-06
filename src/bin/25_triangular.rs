#[allow(clippy::unreadable_literal)]
const SEED: u64 = 20151125;
#[allow(clippy::unreadable_literal)]
const BASE: u64 = 252533;
#[allow(clippy::unreadable_literal)]
const MODULUS: u64 = 33554393;

fn iterations(row: u32, col: u32) -> u32 {
    let diagonal = row + col - 1;
    (diagonal * diagonal + diagonal) / 2 - row
}

fn code(row: u32, col: u32) -> u64 {
    let mut n = SEED;

    for _ in 0..iterations(row, col) {
        n = (n * BASE) % MODULUS;
    }

    n
}

fn main() {
    let nums = adventofcode::read_n_numbers(2);
    println!("{}", code(nums[0], nums[1]));
}

#[cfg(test)]
mod tests {
    use super::*;
    use adventofcode::tests;

    tests! {
        test_iterations {
            it0(1, 1, 0);
            it1(2, 1, 1);
            it2(1, 2, 2);
            it3(3, 1, 3);
            it4(2, 2, 4);
            it5(1, 3, 5);
            it6(4, 1, 6);
            it7(3, 2, 7);
            it8(2, 3, 8);
            it9(1, 4, 9);
            it10(5, 1, 10);
            it11(4, 2, 11);
            it12(3, 3, 12);
            it13(2, 4, 13);
            it14(1, 5, 14);
            it15(6, 1, 15);
            it16(5, 2, 16);
            it17(4, 3, 17);
            it18(3, 4, 18);
            it19(2, 5, 19);
            it20(1, 6, 20);
        }
        test_code {
            c_1_1(1, 1, 20151125);
            c_2_1(2, 1, 31916031);
            c_6_6(6, 6, 27995004);
        }
    }

    fn test_iterations(r: u32, c: u32, expect: u32) {
        assert_eq!(iterations(r, c), expect);
    }

    fn test_code(r: u32, c: u32, expect: u64) {
        assert_eq!(code(r, c), expect);
    }
}

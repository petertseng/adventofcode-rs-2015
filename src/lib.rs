pub mod md5;

use std::env;
use std::fs;

#[macro_export]
macro_rules! tests {
    (
        $($test_func:ident {
            $( $test_name:ident( $( $param:expr ),* ); )+
        })+
    ) => {
        $(
            $(
                #[test]
                fn $test_name() {
                    $test_func($( $param ),* )
                }
            )+
        )+
    }
}

pub fn each_perm<F, T>(v: &mut [T], mut f: F)
where
    F: FnMut(&[T]),
{
    each_perm_gen(v.len(), v, &mut f);
}

fn each_perm_gen<F, T>(k: usize, v: &mut [T], f: &mut F)
where
    F: FnMut(&[T]),
{
    if k == 1 {
        f(v);
        return;
    }

    each_perm_gen(k - 1, v, f);
    for i in 0..(k - 1) {
        if k % 2 == 0 {
            v.swap(i, k - 1);
        } else {
            v.swap(0, k - 1);
        }
        each_perm_gen(k - 1, v, f);
    }
}

pub fn numbers<T>(s: &str) -> Vec<T>
where
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    s.split(|c: char| c != '-' && !c.is_digit(10))
        .filter_map(|d| {
            if d.is_empty() {
                None
            } else {
                Some(d.parse::<T>().expect("can't parse integer"))
            }
        })
        .collect()
}

pub fn read_input_lines<T, F>(f: F) -> Vec<T>
where
    F: FnMut(&str) -> T,
{
    read_input_file().lines().map(f).collect()
}

pub fn read_input_file() -> String {
    let filename = env::args()
        .nth(1)
        .unwrap_or_else(|| "/dev/stdin".to_string());
    fs::read_to_string(filename).expect("couldn't read file")
}

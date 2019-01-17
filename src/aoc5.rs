use std::fs;
use std::str;

pub fn run() {
    let input = fs::read_to_string("day5.txt").unwrap();

    // remove last \n
    println!("5:1 {}", run_1(input.clone()).len() - 1);
    println!("5:2 {}", run_2(input.clone()) - 1);
}

fn run_1(input: String) -> String {
    let mut var = input.into_bytes();

    let mut i = 0;
    // println!("var.len() {}", var.len());
    while (i+1) < var.len() {
        let c1 = var[i] as i32;
        let c2 = var[i+1] as i32;
        if c1 - c2 == 32 || c2 - c1 == 32 {
            // println!("1: {}", str::from_utf8(&var).unwrap());
            var.remove(i+1);
            var.remove(i);
            // println!("2: {}", str::from_utf8(&var).unwrap());
            // println!("i: {}", i);
            if i > 0 {
                i -= 1;
            }
        }
        else {
            i += 1;
        }
    }

    str::from_utf8(&var).unwrap().to_string()
}

fn run_2(input: String) -> usize {
    let letters = "abcdefghijklmnopqrstuvwxyz";
    let mut min_len = input.len();
    for c in letters.chars() {
        let filter_input : String = input.chars().filter(|ic| (*ic != c) & (*ic != c.to_ascii_uppercase())).collect();
        let l = run_1(filter_input).len();
        if l < min_len {
            min_len = l;
        }
    }
    min_len
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn aoc5_run_1() {
        assert_eq!("dabCBAcaDA", run_1("dabAcCaCBAcCcaDA".to_string()));
        assert_eq!("ac", run_1("abBc".to_string()));
        assert_eq!("ac", run_1("aBbc".to_string()));
        assert_eq!("", run_1("CaBbAc".to_string()));
    }

    #[test]
    fn aoc5_run_2() {
        assert_eq!(4, run_2("dabAcCaCBAcCcaDA".to_string()));
    }
}

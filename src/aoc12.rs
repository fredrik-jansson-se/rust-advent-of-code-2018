use std::fs;
use std::ops::{Index, IndexMut};

use nom::types::CompleteStr;
use nom::*;

pub fn run() {
    let input = fs::read_to_string("day12.txt").unwrap();
    println!("12:1 {}", run_1(&input, 20));
}

#[derive(Debug, PartialEq, Eq)]
struct Rule {
    mtch: Vec<bool>,
    output: bool,
}

impl Rule {
    fn is_match(&self, input: &[bool]) -> bool {
        input[0] == self.mtch[0]
            && input[1] == self.mtch[1]
            && input[2] == self.mtch[2]
            && input[3] == self.mtch[3]
            && input[4] == self.mtch[4]
    }
}

#[derive(Debug)]
struct OffsetVec<T> {
    offset: isize,
    default: T,
    vec: Vec<T>,
}

impl<T> OffsetVec<T>
where
    T: Clone,
{
    fn new(init: &[T], default: T) -> Self {
        OffsetVec {
            offset: 0,
            default: default,
            vec: init.to_vec(),
        }
    }

    fn len(&self) -> usize {
        self.vec.len()
    }
}

impl<T> Index<isize> for OffsetVec<T>
where
    T: Clone,
{
    type Output = T;

    fn index(&self, index: isize) -> &Self::Output {
        let real_index = index - self.offset;

        if real_index < 0 || (real_index as usize) >= self.vec.len() {
            &self.default
        } else {
            &self.vec[(index - self.offset) as usize]
        }
    }
}

impl<T> IndexMut<isize> for OffsetVec<T>
where
    T: Clone,
{
    fn index_mut(&mut self, index: isize) -> &mut T {
        if index < self.offset {
            let diff = (self.offset - index) as usize;
            let mut new_vec = vec![self.default.clone(); self.vec.len() + diff];
            new_vec[diff..].clone_from_slice(&self.vec);

            self.vec = new_vec;
            self.offset = index;
        } else if (index - self.offset) as usize >= self.len() {
            self.vec
                .resize((index - self.offset) as usize + 1, self.default.clone());
        }

        &mut self.vec[(index - self.offset) as usize]
    }
}

#[derive(Debug)]
struct Input {
    initial_state: OffsetVec<bool>,
    rules: Vec<Rule>,
}

named!(boolean<CompleteStr, bool>, do_parse!(
        t: alt!(tag!(".") | tag!("#")) >>
        (match t {
            CompleteStr(".") => false,
            _ => true,
        })
        ));

named!(mtch<CompleteStr, Vec<bool> >, many_m_n!(5, 5, boolean));

// ...## => #
named!(rule<CompleteStr, Rule>, do_parse!(
        opt!(space) >>
        m: mtch >>
        space >>
        tag!("=>") >>
        space >>
        o: boolean >>
        opt!(tag!("\n")) >>
        ( Rule {
            mtch: m,
            output: o
        })
        ));

fn create_vec(init: &[bool]) -> Vec<bool> {
    let mut res = vec![false; init.len() + 2];

    res[2..].copy_from_slice(init);
    res.push(false);
    res.push(false);
    res
}

named!(parse<CompleteStr, Input>, do_parse!(
        tag!("initial state:") >>
        space >>
        initial_state: many1!(boolean) >>
        tag!("\n") >>
        tag!("\n") >>
        rules: many1!(rule) >>
        ( Input { initial_state: OffsetVec::new(&initial_state, false), rules: rules } )
        ));

fn print_state(state: &[bool]) {
    for i in 2..(state.len() - 2) {
        if state[i] {
            print!("#");
        } else {
            print!(".");
        }
    }
    println!("");
}

fn run_1(_input_str: &str, _iterations: usize) -> usize {
    // let (_, input) = parse(CompleteStr(input_str)).unwrap();
    // let mut state = input.initial_state;
    // for _ in 0..iterations {
    //     let mut new_v = state.clone();
    //     for i in 2..(state.len()-2) {
    //         for r in &input.rules {
    //             if r.is_match(&state[i-2..i+3]) {
    //                 new_v[i+2] = r.output;
    //                 break;
    //             }
    //         }
    //     }
    //     state = new_v;
    //     print_state(&state);
    // }

    let mut idx_sum = 0;
    // for (i,v) in state[2..].iter().enumerate() {
    //     if *v {
    //         println!("i = {}", i);
    //         idx_sum += i;
    //     }
    // }
    idx_sum
}

#[cfg(test)]
mod tests {
    use super::*;

    fn init_data() -> &'static str {
        r#"initial state: #..#.#..##......###...###

        ...## => #
        ..#.. => #
        .#... => #
        .#.#. => #
        .#.## => #
        .##.. => #
        .#### => #
        #.#.# => #
        #.### => #
        ##.#. => #
        ##.## => #
        ###.. => #
        ###.# => #
        ####. => #"#
    }

    #[test]
    fn aoc12_parse() {
        assert_eq!(boolean(CompleteStr(".")), Ok((CompleteStr(""), false)));
        assert_eq!(boolean(CompleteStr("#")), Ok((CompleteStr(""), true)));
        assert_eq!(
            mtch(CompleteStr(".#.#.")),
            Ok((CompleteStr(""), vec![false, true, false, true, false]))
        );
        assert_eq!(
            rule(CompleteStr(".###. => #")),
            Ok((
                CompleteStr(""),
                Rule {
                    mtch: vec![false, true, true, true, false],
                    output: true
                }
            ))
        );
        assert_eq!(
            rule(CompleteStr("#.#.# => .")),
            Ok((
                CompleteStr(""),
                Rule {
                    mtch: vec![true, false, true, false, true],
                    output: false
                }
            ))
        );

        let parse_res = parse(CompleteStr(init_data())).unwrap();
        assert_eq!(parse_res.0, CompleteStr(""));
        assert_eq!(parse_res.1.rules.len(), 14);
        assert_eq!(parse_res.1.initial_state.len(), 25);
        assert_eq!(parse_res.1.initial_state[0], true);
        assert_eq!(parse_res.1.initial_state[1], false);
        assert_eq!(parse_res.1.initial_state[24], true);
    }

    #[test]
    fn aoc12_offset_vec() {
        let mut v = OffsetVec::new(&[], false);
        assert_eq!(v.len(), 0);

        v[-2] = true;
        assert_eq!(v.len(), 2);
        assert_eq!(v.vec, [true, false]);

        v[0] = true;
        assert_eq!(v.len(), 3);
        assert_eq!(v.vec, [true, false, true]);

        v[2] = true;
        assert_eq!(v.len(), 5);
        assert_eq!(v.vec, [true, false, true, false, true]);

        assert_eq!(v[-3], false);
        assert_eq!(v[2], true);
        assert_eq!(v[3], false);
    }

    #[test]
    fn aoc12_run_1() {
        assert_eq!(run_1(init_data(), 20), 325);
    }
}

use nom::types::CompleteStr;
use nom::*;
use std::fs;

use crate::helper::usize_val;
use crate::opcodes;

pub fn run() {
    let input = fs::read_to_string("day19.txt").unwrap();
    println!("19:1 - {}", run_1(&input, 0));
    println!("19:2 - {}", run_1(&input, 1));
}

type Op = Box<Fn(&mut opcodes::Registers) -> ()>;

named!(parse_ip<CompleteStr, usize>, do_parse!(
        tag!("#ip") >>
        space >>
        ip: usize_val >>
        tag!("\n") >>
        (ip)
        ));

named!(parse_instruction<CompleteStr, (String, Vec<usize>)>, do_parse!(
        instruction: alphanumeric >>
        space >>
        vals: separated_list!(tag!(" "), usize_val) >>
        tag!("\n") >>
        ((instruction.to_string(), vals))
        ));

named!(parse<CompleteStr, (usize, Vec<(String, Vec<usize>)>)>, pair!(parse_ip, many1!(parse_instruction)));

fn run_1(input: &str, reg0_start: usize) -> usize {
    let (_, (ip, instructions)) = parse(CompleteStr(input)).unwrap();
    let mut regs = opcodes::Registers::new();

    // Init ip register to 0
    regs.insert(0, reg0_start);
    regs.insert(1, 0);
    regs.insert(2, 0);
    regs.insert(3, 0);
    regs.insert(4, 0);
    regs.insert(5, 0);
    regs.insert(ip, 0);

    let mut iptr = 0;
    while iptr < instructions.len() {
        let (i, v) = &instructions[iptr];
        // println!("iptr: {} - {:?}", iptr, v);
        match i.as_ref() {
            "addi" => opcodes::addi(&mut regs, v[0], v[1], v[2]),
            "addr" => opcodes::addr(&mut regs, v[0], v[1], v[2]),
            "eqrr" => opcodes::eqrr(&mut regs, v[0], v[1], v[2]),
            "gtrr" => opcodes::gtrr(&mut regs, v[0], v[1], v[2]),
            "muli" => opcodes::muli(&mut regs, v[0], v[1], v[2]),
            "mulr" => opcodes::mulr(&mut regs, v[0], v[1], v[2]),
            "seti" => opcodes::seti(&mut regs, v[0], v[1], v[2]),
            "setr" => opcodes::setr(&mut regs, v[0], v[1], v[2]),
            i => panic!("Unhandled instruction {}", i),
        }
        iptr = regs[&ip];
        iptr += 1;
        regs.insert(ip, iptr);
    }

    // println!("regs: {:?}", regs);
    regs[&0]
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn aoc19_parse() {
        assert_eq!(
            parse_instruction(CompleteStr("addi 2 16 2\n")),
            Ok((CompleteStr(""), ("addi".to_string(), vec![2, 16, 2])))
        );
        assert_eq!(
            parse_instruction(CompleteStr("seti 1 0 4\n")),
            Ok((CompleteStr(""), ("seti".to_string(), vec![1, 0, 4])))
        );
    }
}

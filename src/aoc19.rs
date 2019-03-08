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
    regs = vec![0; 6];
    regs[0] = reg0_start;
    regs[ip] = 0;

    while regs[ip] < instructions.len() {
        let (i, v) = &instructions[regs[ip]];
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
        regs[ip] += 1;
    }

    // println!("regs: {:?}", regs);
    regs[0]
}

/*
#ip 2
R0 = 1
00  addi 2 16 2   # Add 16 to R2 => jmp to 17
01  seti 1 0 4
02  seti 1 5 5
03  mulr 4 5 1
04  eqrr 1 3 1
05  addr 1 2 2
06  addi 2 1 2
07  addr 4 0 0
08  addi 5 1 5
09  gtrr 5 3 1
10  addr 2 1 2
11  seti 2 6 2
12  addi 4 1 4
13  gtrr 4 3 1
14  addr 1 2 2
15  seti 1 7 2
16  mulr 2 2 2
17  addi 3 2 3    # R3 += 2
18  mulr 3 3 3    # R3 = R3*R3
19  mulr 2 3 3    # R3 = R2*R3
20  muli 3 11 3   # R3 = 11*R3
21  addi 1 6 1    # R1 += 6
22  mulr 1 2 1    # R1 = R1*R2 = R1*22
23  addi 1 6 1    # R1 += 6
24  addr 3 1 3    # R3 += R1
25  addr 2 0 2    # R2 += R0 (jmp to R0 + 1)
26  seti 0 3 2    # R2 = 0 (jmp to 0 + 1)
27  setr 2 3 1    # R1 = R2
28  mulr 1 2 1    # R1 *= R2 = line b4 sets R1 = RA => R1 = R1*R1
29  addr 2 1 1    # R1 += R2
30  mulr 2 1 1    # R1 *= R2
31  muli 1 14 1
32  mulr 1 2 1
33  addr 3 1 3
34  seti 0 9 0
35  seti 0 5 2
*/

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

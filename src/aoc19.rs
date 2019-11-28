use nom::bytes::complete::tag;
use nom::character::complete::{alphanumeric1, space1};
use nom::multi::{many1, separated_list};
use nom::sequence::pair;
use nom::*;
use std::fs;

use crate::helper::usize_val;
use crate::opcodes;

pub fn run() {
    let input = fs::read_to_string("day19.txt").unwrap();
    println!("19:1 - {}", run_1(&input, 0));
    println!("19:2 - {}", run_2(&input, 1));
}

fn parse_ip(i: &str) -> IResult<&str, usize> {
    let (i, _) = tag("#ip")(i)?;
    let (i, _) = space1(i)?;
    let (i, ip) = usize_val(i)?;
    let (i, _) = tag("\n")(i)?;
    Ok((i, ip))
}

fn parse_instruction(i: &str) -> IResult<&str, (String, Vec<usize>)> {
    let (i, instruction) = alphanumeric1(i)?;
    let (i, _) = space1(i)?;
    let (i, vals) = separated_list(tag(" "), usize_val)(i)?;
    let (i, _) = tag("\n")(i)?;
    Ok((i, (instruction.to_string(), vals)))
}

fn parse(i: &str) -> IResult<&str, (usize, Vec<(String, Vec<usize>)>)> {
    pair(parse_ip, many1(parse_instruction))(i)
}

fn run_1(input: &str, reg0_start: usize) -> usize {
    let (_, (ip, instructions)) = parse(input).unwrap();

    // Init ip register to 0
    let mut regs = vec![0; 6];
    regs[0] = reg0_start;
    regs[ip] = 0;

    let mut last_r0 = 0;
    let mut fst = true;
    while regs[ip] < instructions.len() {
        println!("{} - {:?}", regs[ip], regs);
        if regs[ip] > 11 {}
        if regs[0] != last_r0 {
            println!("{} - {:?}", regs[ip], regs);
            last_r0 = regs[0];
        }
        if fst && regs[ip] == 2 {
            fst = false;
            // 3 => 4
            // 4 =-> 7
            // 5 => 6
            // 6 => 12
            // 8 => 15
            // 10 => 18
            // 20 => 42
            // 40
            // regs[3] = 4; 7
            // regs[3] = 8; 15
            // regs[3] = 16; 31;
            // regs[3] = 31; 32;
            // regs[3] = 32; 63;
            // regs[3] = 33; 48
            regs[3] = 33;
            // regs[3] = 64; 127;
            // regs[3] = 40; 90
            // regs[3] = 41; 42
            // regs[3] = 45; 78
            // regs[3] = 100; 217
            // regs[3] = 1000; 2340;
            // regs[3] = 20000; 49203;
            // regs[3] = 40000; 99187;
        }
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

fn run_2(_input: &str, _reg0_start: usize) -> usize {
    // let mut regs = vec![0, 10550400, 1, 10551374, 0, 0];

    // regs[0]
    0
}

/*
#ip 2
R0 = 1
00  addi 2 16 2   # Add 16 to R2 => jmp to 17
01  seti 1 0 4    # R4 = 1
02  seti 1 5 5    # R5 = 1
03  mulr 4 5 1    # R1 = R4*R5
04  eqrr 1 3 1    # R1 = 1 if R1==R3
05  addr 1 2 2    # R2 += R1 --- this is a jump to 7 if R1==R3
06  addi 2 1 2    # R2 += 1  - jmp to 8
07  addr 4 0 0    # R0 += R4
08  addi 5 1 5    # R5 += 1
09  gtrr 5 3 1    # R1 = 1 if R5 > R3
10  addr 2 1 2    # R2 += R1
11  seti 2 6 2    # R2 = 2 => jmp to 3
12  addi 4 1 4    # R4 += 1
13  gtrr 4 3 1    # R1 = 1 if R4 > R3
14  addr 1 2 2    # R2 += R1
15  seti 1 7 2    # R2 = 1
16  mulr 2 2 2    # EXIT
17  addi 3 2 3    # R3 += 2
18  mulr 3 3 3    # R3 = R3*R3
19  mulr 2 3 3    # R3 = R2*R3 => R3 *= 19
20  muli 3 11 3   # R3 = 11*R3
21  addi 1 6 1    # R1 += 6
22  mulr 1 2 1    # R1 = R1*R2 = R1*22
23  addi 1 6 1    # R1 += 6
24  addr 3 1 3    # R3 += R1
25  addr 2 0 2    # R2 += R0 (jmp to R0 + 1)
26  seti 0 3 2    # R2 = 0 (jmp to 0 + 1)
27  setr 2 3 1    # R1 = R2
28  mulr 1 2 1    # R1 *= R2  R1 *= 28
29  addr 2 1 1    # R1 += R2  R1 += 29
30  mulr 2 1 1    # R1 *= R2  R1 *= 30
31  muli 1 14 1   # R1 *= 14
32  mulr 1 2 1    # R1 *= R2 = R1*=32
33  addr 3 1 3    # R3 += R1
34  seti 0 9 0    # R0 = 0
35  seti 0 5 2    # jmp 1
*/

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn aoc19_parse() {
        assert_eq!(
            parse_instruction("addi 2 16 2\n"),
            Ok(("", ("addi".to_string(), vec![2, 16, 2])))
        );
        assert_eq!(
            parse_instruction("seti 1 0 4\n"),
            Ok(("", ("seti".to_string(), vec![1, 0, 4])))
        );
    }
}

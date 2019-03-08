use nom::types::CompleteStr;
use nom::*;
use std::collections::HashMap;
use std::fs;

use super::helper::usize_val;
use super::opcodes::*;

pub fn run() {
    let input = fs::read_to_string("day16.txt").unwrap();

    println!("16:1 - {}", run_1(&input));
    println!("16:2 - {}", run_2(&input));
}

fn run_1(input: &str) -> usize {
    let (_, samples) = parse_samples(CompleteStr(input)).unwrap();

    let ops = [
        addr, addi, mulr, muli, banr, bani, borr, bori, setr, seti, gtir, gtri, gtrr, eqir, eqri,
        eqrr,
    ];

    let mut cnt = 0;
    for sample in samples {
        let mut sample_cnt = 0;
        for op in &ops {
            let mut regs = sample.before.clone();
            op(
                &mut regs,
                sample.instruction.a,
                sample.instruction.b,
                sample.instruction.c,
            );

            if regs == sample.after {
                sample_cnt += 1;
            }
        }
        if sample_cnt >= 3 {
            cnt += 1;
        }
    }

    cnt
}

type OP = fn(&mut Registers, usize, usize, usize);

fn run_2(input: &str) -> usize {
    let (_, program) = parse_program(CompleteStr(input)).unwrap();

    let ops = [
        addr, addi, mulr, muli, banr, bani, borr, bori, setr, seti, gtir, gtri, gtrr, eqir, eqri,
        eqrr,
    ];

    let mut op_lookup: HashMap<usize, Vec<usize>> = HashMap::new();

    for sample in program.samples {
        if !op_lookup.contains_key(&sample.instruction.opcode) {
            op_lookup.insert(sample.instruction.opcode, (0..ops.len()).collect());
        }

        op_lookup
            .get_mut(&sample.instruction.opcode)
            .unwrap()
            .retain(|op| is_match(&ops[*op], &sample));
    }

    let mut code_to_op: HashMap<usize, usize> = HashMap::new();
    while !op_lookup.is_empty() {
        let mut singulars: Vec<usize> = Vec::new();
        op_lookup.retain(|k, v| {
            if v.len() == 1 {
                singulars.push(v[0]);
                code_to_op.insert(*k, v[0]);
            }
            v.len() != 1
        });

        for (_, v) in op_lookup.iter_mut() {
            v.retain(|op| !singulars.contains(op));
        }
    }

    let mut regs = vec![0, 0, 0, 0];

    for i in program.instructions {
        ops[code_to_op[&i.opcode]](&mut regs, i.a, i.b, i.c);
    }

    regs[0]
}

fn is_match(op: &OP, sample: &Sample) -> bool {
    let mut r = sample.before.clone();
    op(
        &mut r,
        sample.instruction.a,
        sample.instruction.b,
        sample.instruction.c,
    );
    r == sample.after
}

named!(space_usize_val<CompleteStr, usize>, do_parse!(
        opt!(space) >>
        v: usize_val >>
        (v)
        ));

named!(parse_regs<CompleteStr, Registers>, do_parse!(
        tag!("[") >>
        vals: separated_list!(tag!(","), space_usize_val) >>
        tag!("]") >>
        (
            vals
            // vals.iter().enumerate().fold(Registers::new(),
            // |mut r, (i,v)| {r.insert(i, *v as usize); r} )
            )
        ));

// Before: [3, 2, 1, 1]
named!(parse_before<CompleteStr, Registers>, do_parse!(
    opt!(space) >>
    tag!("Before:") >>
    opt!(space) >>
    regs: parse_regs >>
    tag!("\n") >>
    (regs)
));

// After: [3, 2, 1, 1]
named!(parse_after<CompleteStr, Registers>, do_parse!(
    opt!(space) >>
    tag!("After:") >>
    opt!(space) >>
    regs: parse_regs >>
    tag!("\n") >>
    (regs)
));

#[derive(Debug, PartialEq)]
struct Instruction {
    opcode: usize,
    a: usize,
    b: usize,
    c: usize,
}

named!(
    parse_instruction < CompleteStr, Instruction>, do_parse!(
        vals: separated_list!(tag!(" "), usize_val) >>
        tag!("\n") >>
        (Instruction {
            opcode: vals[0],
            a: vals[1],
            b: vals[2],
            c: vals[3],
        })
    ));

#[derive(Debug)]
struct Sample {
    before: Registers,
    instruction: Instruction,
    after: Registers,
}

named!(parse_sample<CompleteStr, Sample>, do_parse!(
        before: parse_before >>
        instruction: parse_instruction >>
        after: parse_after >>
        tag!("\n") >>
        ( Sample {
            before,
            instruction,
            after
        })));

named!(
    parse_samples<CompleteStr, Vec<Sample>>,
    many1!(parse_sample)
);

struct Program {
    samples: Vec<Sample>,
    instructions: Vec<Instruction>,
}

named!(parse_program<CompleteStr, Program>, do_parse!(
        samples: parse_samples >>
        tag!("\n") >>
        tag!("\n") >>
        instructions: many1!(parse_instruction) >>
        ( Program { samples, instructions } )
        ));

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn aoc16_parse() {
        assert_eq!(
            space_usize_val(CompleteStr(" 123")),
            Ok((CompleteStr(""), 123))
        );

        let (_, regs) = parse_regs(CompleteStr("[3, 2, 1, 1]")).unwrap();
        assert_eq!(regs[0], 3);
        assert_eq!(regs[1], 2);
        assert_eq!(regs[2], 1);
        assert_eq!(regs[3], 1);

        let (_, bregs) = parse_before(CompleteStr("  Before: [3, 2, 1, 1]\n")).unwrap();
        assert_eq!(regs, bregs);

        let (_, aregs) = parse_after(CompleteStr("After: [3, 2, 1, 1]\n")).unwrap();
        assert_eq!(regs, aregs);

        let (_, insts) = parse_instruction(CompleteStr("1 2 3 4\n")).unwrap();
        assert_eq!(
            insts,
            Instruction {
                opcode: 1,
                a: 2,
                b: 3,
                c: 4
            }
        );

        let (_, sample) = parse_sample(CompleteStr(
            "Before: [3, 2, 1, 1]\n9 2 1 2\nAfter:  [3, 2, 2, 1]\n\n",
        ))
        .unwrap();
    }

    #[test]
    fn aoc16_run_1() {
        assert_eq!(
            run_1("Before: [3, 2, 1, 1]\n9 2 1 2\nAfter:  [3, 2, 2, 1]\n\n"),
            1
        );
    }
}

// use std::collections::HashMap;

pub type Registers = Vec<usize>;

pub fn addr(regs: &mut Registers, a: usize, b: usize, c: usize) {
    let v = regs[a] + regs[b];
    regs[c] = v;
}

pub fn addi(regs: &mut Registers, a: usize, b: usize, c: usize) {
    let v = regs[a] + b;
    regs[c] = v;
}

pub fn mulr(regs: &mut Registers, a: usize, b: usize, c: usize) {
    regs[c] = regs[a] * regs[b];
}

pub fn muli(regs: &mut Registers, a: usize, b: usize, c: usize) {
    let v = regs[a] * b;
    regs[c] = v;
}

pub fn banr(regs: &mut Registers, a: usize, b: usize, c: usize) {
    let v = regs[a] & regs[b];
    regs[c] = v;
}

pub fn bani(regs: &mut Registers, a: usize, b: usize, c: usize) {
    let v = regs[a] & b;
    regs[c] = v;
}

pub fn borr(regs: &mut Registers, a: usize, b: usize, c: usize) {
    let v = regs[a] | regs[b];
    regs[c] = v;
}

pub fn bori(regs: &mut Registers, a: usize, b: usize, c: usize) {
    let v = regs[a] | b;
    regs[c] = v;
}

pub fn setr(regs: &mut Registers, a: usize, _: usize, c: usize) {
    regs[c] = regs[a];
}

pub fn seti(regs: &mut Registers, a: usize, _: usize, c: usize) {
    regs[c] = a;
}

pub fn gtir(regs: &mut Registers, a: usize, b: usize, c: usize) {
    let v = if a > regs[b] { 1 } else { 0 };
    regs[c] = v;
}

pub fn gtri(regs: &mut Registers, a: usize, b: usize, c: usize) {
    let v = if regs[a] > b { 1 } else { 0 };
    regs[c] = v;
}

pub fn gtrr(regs: &mut Registers, a: usize, b: usize, c: usize) {
    regs[c] = if regs[a] > regs[b] { 1 } else { 0 };
}

pub fn eqir(regs: &mut Registers, a: usize, b: usize, c: usize) {
    let v = if a == regs[b] { 1 } else { 0 };
    regs[c] = v;
}

pub fn eqri(regs: &mut Registers, a: usize, b: usize, c: usize) {
    let v = if regs[a] == b { 1 } else { 0 };
    regs[c] = v;
}

pub fn eqrr(regs: &mut Registers, a: usize, b: usize, c: usize) {
    let v = if regs[a] == regs[b] { 1 } else { 0 };
    regs[c] = v;
}

// pub fn neqr(regs: &mut Registers, a: usize, b: usize, c: usize) {
//     let v = if regs[a] != regs[b] { 1 } else { 0 };
//     regs[c] = v;
// }

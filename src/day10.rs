use std::collections::VecDeque;

enum Instruction {
    AddX(i32),
    NoOp,
}

fn parse(str: &str) -> VecDeque<Instruction> {
     str.lines()
        .map(|l| l.split_ascii_whitespace())
        .map(|mut spl| match spl.next() {
            Some("addx") => Instruction::AddX(spl.next().expect("addx without a value?").parse().expect("failed to parse addx value to i32")),
            Some("noop") => Instruction::NoOp,
            _ => unreachable!()})
        .fold(VecDeque::<Instruction>::new(), |mut acc, inst| {
            match inst {
                Instruction::AddX(_) => { 
                    acc.push_back(Instruction::NoOp);
                    acc.push_back(inst);
                }
                Instruction::NoOp => acc.push_back(inst)
            }
            acc
        })
}

fn eval_program(program: &mut VecDeque<Instruction>) -> (usize, [[char;40];6]) {
    let cycle_checks = [20, 60, 100, 140, 180, 220];
    let mut img = [['.';40];6];
    let mut acc_signal_strength = 0;
    let mut x: i32 = 1;
    let mut cycle = 0;
    loop {
        let crt = cycle % img[0].len();
        if x.abs_diff(crt as i32) <= 1 {
            img[cycle.div_euclid(img[0].len())][crt] = '#';
        }
        cycle += 1;
        if cycle_checks.contains(&cycle) {
            acc_signal_strength += cycle as i32 * x;
        }
        match program.pop_front() {
            Some(Instruction::AddX(val)) => x += val,
            Some(Instruction::NoOp) => continue,
            None => break
        }
    }
    (acc_signal_strength.try_into().unwrap(), img)
}

fn print_img(img: &[[char;40];6]) {
    for x in img.iter() {
        for y in x.iter() {
            print!("{}",y);
        }
        println!();
    }
}

pub fn solve(str: &str) -> (usize, usize) {
    let mut program = parse(str);
    let (acc_signal_strength, img) = eval_program(&mut program);
    print_img(&img);
    (acc_signal_strength, 0)
}
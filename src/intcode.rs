use crate::day::Day;
use std::error::Error;
use std::io::prelude::*;
use std::collections::VecDeque;



pub fn read_program<D: Day>(day: &D) -> Vec<i32> {
    let path = day.input();
    let lines = day.read_input_lines_string(&path);
    let program = lines[0].split(',').map(|t| t.parse::<i32>().unwrap()).collect();
    program
}

#[derive(PartialEq, Debug)]
pub enum Opcode {
    Add,
    Mul,
    Input,
    Output,
    Jnz, 
    Jz,
    Le,
    Eq,
    Halt(i32)
}

#[derive(PartialEq, Debug)]
pub enum ParamMode {
    Position,
    Immediate
}

pub struct IntCodePC<'i, 'o> {
    program: Vec<i32>,
    pc: usize,
    modes: VecDeque<ParamMode>,
    i: &'i mut dyn BufRead,
    o: &'o mut dyn Write
}

impl<'i, 'o> IntCodePC<'i, 'o> {

    pub fn new<I: BufRead, O: Write>(program: Vec<i32>, i: &'i mut I, o: &'o mut O) -> IntCodePC<'i, 'o> {
        IntCodePC {program, pc:0, modes: VecDeque::new(), i, o}
    }

    pub fn alert1202(&mut self) {
        self.init(12, 02);
    }

    pub fn init(&mut self, noun: i32, verb: i32) {
        self.program[1] = noun;
        self.program[2] = verb;
    }

    pub fn reset(&mut self, program: Vec<i32>) {
        self.program = program;
        self.pc = 0;
        self.modes.clear();
    }

    pub fn run(&mut self) -> i32 {
        loop {
            match self.step() {
                Opcode::Halt(val) => return val,
                _ => {}
            }
        }
    }

    pub fn step(&mut self) -> Opcode {
        let op = self.op();
        match op {
            Opcode::Add => self.add(),
            Opcode::Mul => self.mul(),
            Opcode::Input => self.input(),
            Opcode::Output => self.output(),
            Opcode::Jnz => self.jnz(),
            Opcode::Jz => self.jz(),
            Opcode::Le => self.le(),
            Opcode::Eq => self.eq(),
            Opcode::Halt(_) => {}
        }
        op
    }

    pub fn op(&mut self) -> Opcode {
        let ins = self.read_imm();
        let mut modes = ins / 100;
        while modes > 0 {
            if modes % 10 == 0 {
                self.modes.push_back(ParamMode::Position);
            } 
            else {
                self.modes.push_back(ParamMode::Immediate);
            }

            modes /= 10;
        }

        match ins % 100 {
            1 => Opcode::Add,
            2 => Opcode::Mul,
            3 => Opcode::Input,
            4 => Opcode::Output,
            5 => Opcode::Jnz,
            6 => Opcode::Jz,
            7 => Opcode::Le,
            8 => Opcode::Eq,
            99 => Opcode::Halt(self.halt()),
            _ => panic!("invalid opcode")
        }
    }

    pub fn add(&mut self) {
        let sum = self.read() + self.read();
        self.write(sum);
    }

    pub fn mul(&mut self) {
        let prod = self.read() * self.read();
        self.write(prod);
    }

    pub fn input(&mut self) {
        let pos = self.read_imm();
        let mut buf = String::new();
        match self.i.read_line(&mut buf) {
            Ok(_) => {
                match buf.trim().parse::<i32>() {
                    Ok(val) => {
                        self.program[pos as usize] = val;
                    },
                    Err(e) => {
                        panic!(format!("NaN: {} [{}]", e.description(), buf))}
                }
            },
            Err(e) => panic!(format!("Failed to read from cin: {}", e.description()))
        }

    }

    pub fn output(&mut self) {
       let val = self.read(); 
       match writeln!(self.o, "{}", val) {
          Ok(_) => {},
          Err(e) => panic!(format!("Failed to write to stdout: {}", e.description()))
       }
    }

    pub fn jnz(&mut self) {
        let cond = self.read();
        let pos = self.read();
        if cond != 0 {
            self.pc = pos as usize;
        }
    }

    pub fn jz(&mut self) {
        let cond = self.read();
        let pos = self.read();
        if cond == 0 {
            self.pc = pos as usize;
        }
    }

    pub fn le(&mut self) {
        let lhs = self.read();
        let rhs = self.read();
        if lhs < rhs {
            self.write(1);
        } else {
            self.write(0);
        }
    }

    pub fn eq(&mut self) {
        let lhs = self.read();
        let rhs = self.read();

        if rhs == lhs {
            self.write(1);
        } else {
            self.write(0);
        }
    }

    pub fn halt(&self) -> i32 {
        self.program[0]
    }

    pub fn mode(&mut self) -> ParamMode {
        match self.modes.pop_front() {
            Some(m) => m,
            None => ParamMode::Position
        }
    }

    pub fn read(&mut self) -> i32 {
        match self.mode() {
            ParamMode::Position => {
                self.read_pos()
            },
            ParamMode::Immediate => {
                self.read_imm()
            }
        }
    }

    pub fn read_pos(&mut self) -> i32 {
        let pos = self.program[self.pc];
        self.pc += 1;
        self.program[pos as usize]
    }

    pub fn read_imm(&mut self) -> i32 {
        let val = self.program[self.pc];
        self.pc += 1;
        val
    }

    pub fn write(&mut self, val: i32) {
        let pos = self.program[self.pc];
        self.program[pos as usize] = val;
        self.pc += 1;
    }
}


#[cfg(test)]
mod tests 
{
    use super::*;

    #[test]
    fn add() {
        let mut o = std::io::sink();
        let mut i = std::io::empty();
        let mut pc = IntCodePC::new(vec![1,9,10,3,2,3,11,0,99,30,40,50], &mut i, &mut o);

        assert_eq!(pc.step(), Opcode::Add);
        assert_eq!(pc.program, vec![1,9,10,70,2,3,11,0,99,30,40,50]);
        assert_eq!(pc.pc, 4);
    }


    #[test]
    fn mul() {

        let mut o = std::io::sink();
        let mut i = std::io::empty();
        let mut pc = IntCodePC::new(vec![1,9,10,70,2,3,11,0,99,30,40,50], &mut i, &mut o);
        pc.pc = 4;

        assert_eq!(pc.step(), Opcode::Mul);
        assert_eq!(pc.program, vec![3500,9,10,70,2,3,11,0,99,30,40,50]);
        assert_eq!(pc.pc, 8);
    }

    #[test]
    fn input() {

        let buf = b"15";
        let mut i = &buf[..];
        let mut o = std::io::sink();
        let mut pc = IntCodePC::new(vec![3,2,0], &mut i, &mut o);
        assert_eq!(pc.step(), Opcode::Input);
        assert_eq!(vec![3, 2, 15], pc.program);
    }

    #[test]
    fn output() {

        let mut i = std::io::empty();
        let mut o: Vec<u8> = Vec::new();
        let mut pc = IntCodePC::new(vec![4,2,15], &mut i, &mut o);
        assert_eq!(pc.step(), Opcode::Output);
        let s = String::from_utf8(o).unwrap();
        assert_eq!(s, "15\n");
    }

    #[test] 
    fn jnz() {
        let mut o = std::io::sink();
        let mut i = std::io::empty();
        let mut pc = IntCodePC::new(vec![1105,1,9], &mut i, &mut o);
        assert_eq!(pc.step(), Opcode::Jnz);
        assert_eq!(pc.pc, 9);

        pc.reset(vec![1005,2, 0]);
        assert_eq!(pc.step(), Opcode::Jnz);
        assert_eq!(pc.pc, 3);
    }

    #[test] 
    fn jz() {
        let mut o = std::io::sink();
        let mut i = std::io::empty();
        let mut pc = IntCodePC::new(vec![1106,1,9], &mut i, &mut o);
        assert_eq!(pc.step(), Opcode::Jz);
        assert_eq!(pc.pc, 3);

        pc.reset(vec![1006,2, 0]);
        assert_eq!(pc.step(), Opcode::Jz);
        assert_eq!(pc.pc, 0);
    }

    #[test] 
    fn le() {
        let mut o = std::io::sink();
        let mut i = std::io::empty();
        let mut pc = IntCodePC::new(vec![11107,1,9, 3], &mut i, &mut o);
        assert_eq!(pc.step(), Opcode::Le);
        assert_eq!(pc.program, vec![11107,1,9, 1]);

        pc.reset(vec![00007,4, 5, 6, 55, 66, 0]);
        assert_eq!(pc.step(), Opcode::Le);
        assert_eq!(pc.program, vec![00007,4, 5, 6, 55, 66, 1]);
    }

    #[test] 
    fn eq() {
        let mut o = std::io::sink();
        let mut i = std::io::empty();
        let mut pc = IntCodePC::new(vec![11108, 1, 1, 3], &mut i, &mut o);
        assert_eq!(pc.step(), Opcode::Eq);
        assert_eq!(pc.program, vec![11108, 1, 1, 1]);

        pc.reset(vec![00008, 4, 5, 6, 55, 55, 0]);
        assert_eq!(pc.step(), Opcode::Eq);
        assert_eq!(pc.program, vec![00008, 4, 5, 6, 55, 55, 1]);
    }

    #[test]
    fn halt() {

        let mut o = std::io::sink();
        let mut i = std::io::empty();
        let mut pc = IntCodePC::new(vec![3500,9,10,70,2,3,11,0,99,30,40,50], &mut i, &mut o);
        pc.pc = 8;

        assert_eq!(pc.step(), Opcode::Halt(3500));
    }

    #[test]
    fn run() {

        let mut i = std::io::empty();
        let mut o = std::io::sink();

        let mut pc = IntCodePC::new(vec![1,9,10,3,2,3,11,0,99,30,40,50], &mut i, &mut o);
        assert_eq!(pc.run(), 3500);

        let mut pc = IntCodePC::new(vec![1,0,0,0,99], &mut i, &mut o);
        assert_eq!(pc.run(), 2);

        let mut pc = IntCodePC::new(vec![2,3,0,3,99], &mut i, &mut o);
        assert_eq!(pc.run(), 2);

        let mut pc = IntCodePC::new(vec![2,4,4,5,99,0], &mut i, &mut o);
        assert_eq!(pc.run(), 2);

        let mut pc = IntCodePC::new(vec![1,1,1,4,99,5,6,0,99], &mut i, &mut o);
        assert_eq!(pc.run(), 30);
    }

    #[test]
    pub fn modes() {
        let mut i = std::io::empty();
        let mut o = std::io::sink();
        let mut pc = IntCodePC::new(vec![1002,4,3,4,33], &mut i, &mut o);
        assert_eq!(Opcode::Mul, pc.step());
        assert_eq!(vec![1002,4,3,4,99], pc.program);
    }
}
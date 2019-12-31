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

    pub fn halt(&self) -> i32 {
        self.program[0]
    }

    pub fn read(&mut self) -> i32 {
        let mode = self.mode();
        match mode {
            ParamMode::Position => {
                let pos = self.program[self.pc + 1];
                self.pc += 1;
                self.program[pos as usize]
            },
            ParamMode::Immediate => {
                self.pc += 1;
                self.program[self.pc]
            }
        }
    }

    pub fn write(&mut self, val: i32) {
        let pos = self.program[self.pc + 1];
        self.pc += 1;
        self.program[pos as usize] = val;
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
        let pos = self.read();
        let mut buf = String::new();
        match self.i.read_line(&mut buf) {
            Ok(_) => {
                match buf.trim().parse::<i32>() {
                    Ok(val) => {
                        self.program[pos as usize] = val;
                    },
                    Err(e) => {
                        panic!(format!("NaN: {}", e.description()))}
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

    pub fn alert1202(&mut self) {
        self.program[1] = 12;
        self.program[2] = 02;
    }

    pub fn reset(&mut self, program: Vec<i32>) {
        self.program = program;
        self.pc = 0;
    }

    pub fn init(&mut self, noun: i32, verb: i32) {
        self.program[1] = noun;
        self.program[2] = verb;
    }

    pub fn mode(&mut self) -> ParamMode {
        match self.modes.pop_front() {
            Some(m) => m,
            None => ParamMode::Position
        }
    }

    pub fn op(&mut self) -> Opcode {
        let ins = self.program[self.pc];
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
            3 => {
                self.modes.push_back(ParamMode::Immediate);
                Opcode::Input
            },
            4 => Opcode::Output,
            99 => Opcode::Halt(self.halt()),
            _ => panic!("invalid opcode")
        }
    }

    pub fn step(&mut self) -> Opcode {
        let op = self.op();
        match op {
            Opcode::Add => self.add(),
            Opcode::Mul => self.mul(),
            Opcode::Input => self.input(),
            Opcode::Output => self.output(),
            Opcode::Halt(_) => {}
        }

        self.pc += 1;

        op
    }

    pub fn run(&mut self) -> i32 {
        loop {
            match self.step() {
                Opcode::Halt(val) => return val,
                _ => {}
            }
        }
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
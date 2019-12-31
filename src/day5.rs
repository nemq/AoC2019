use crate::day::Day;
use crate::intcode::{IntCodePC, read_program};

pub struct Day5 {
}

impl Day for Day5 {
    fn first_puzzle(&self) -> String {
        let program = read_program(self);
        let buf = b"1";
        let mut i = &buf[..];
        let mut o: Vec<u8> = Vec::new();
        let mut pc = IntCodePC::new(program, &mut i, &mut o);
        pc.run();

        let lines = String::from_utf8(o).unwrap();
        let code = lines.split_whitespace().last().unwrap();

        format!("{}", code)
    }

    fn second_puzzle(&self) -> String {
        let program = read_program(self);
        let buf = b"5";
        let mut i = &buf[..];
        let mut o: Vec<u8> = Vec::new();
        let mut pc = IntCodePC::new(program, &mut i, &mut o);
        pc.run();

        let lines = String::from_utf8(o).unwrap();
        let code = lines.split_whitespace().last().unwrap();

        format!("{}", code)
    }

    fn number(&self) -> u8 {
        5
    }
}


#[cfg(test)]
mod tests 
{
    use super::*;

    static DAY5: Day5 = Day5 {};

    #[test]
    fn read_bounds() {
    }
    
}

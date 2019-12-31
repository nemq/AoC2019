
use crate::day::Day;
use crate::intcode::{IntCodePC, read_program};

pub struct Day2 {

}

impl Day for Day2 {
    fn first_puzzle(&self) -> String {
        let program = read_program(self);
        let mut i = std::io::empty();
        let mut o = std::io::sink();
        let mut pc = IntCodePC::new(program, &mut i, &mut o);
        pc.alert1202();
        let ret = pc.run();
        format!("{}", ret)
    }

    fn second_puzzle(&self) -> String {

        let program = read_program(self);

        let mut i = std::io::empty();
        let mut o = std::io::sink();
        let mut pc = IntCodePC::new(program.clone(), &mut i, &mut o);

        for noun in 0..99 {
            for verb in 0 .. 99 {
                pc.init(noun, verb);                
                if pc.run() == 19690720 {
                    let ret = 100 * noun + verb;
                    return format!("{}", ret);
                } else {
                    pc.reset(program.clone());
                }
            }
        }

        panic!("19690720 not found");
    }

    fn number(&self) -> u8 {
        2
    }
}


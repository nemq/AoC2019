
use crate::day::Day;
use crate::intcode::IntCodePC;

pub struct Day2 {

}

impl Day2 {

    pub fn read_program(&self) -> Vec<usize> {
        let path = self.input();
        let lines = self.read_input_lines_string(&path);
        let program = lines[0].split(',').map(|t| t.parse::<usize>().unwrap()).collect();
        program
    }
}

impl Day for Day2 {
    fn first_puzzle(&self) -> String {
        let program = self.read_program();
        let mut pc = IntCodePC::new(program);
        pc.alert1202();
        let ret = pc.run();
        format!("{}", ret)
    }

    fn second_puzzle(&self) -> String {

        let program = self.read_program();
        let mut pc = IntCodePC::new(program.clone());

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


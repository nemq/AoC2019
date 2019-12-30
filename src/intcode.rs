
#[derive(PartialEq, Debug)]
pub enum Opcode {
    Add,
    Mul,
    Halt(usize)
}

pub struct IntCodePC {
    program: Vec<usize>,
    pc: usize
}

impl IntCodePC {
    pub fn new(program: Vec<usize>) -> IntCodePC {
        IntCodePC {program, pc: 0}
    }

    pub fn halt(&self) -> usize {
        self.program[0]
    }

    pub fn read(&mut self) -> usize {
        let pos = self.program[self.pc + 1];
        self.pc += 1;
        self.program[pos]
    }

    pub fn write(&mut self, val: usize) {
        let pos = self.program[self.pc + 1];
        self.pc += 1;
        self.program[pos] = val;
    }

    pub fn add(&mut self) {
        let sum = self.read() + self.read();
        self.write(sum);
        self.pc += 1; 
    }

    pub fn mul(&mut self) {
        let prod = self.read() * self.read();
        self.write(prod);
        self.pc += 1; 
    }

    pub fn alert1202(&mut self) {
        self.program[1] = 12;
        self.program[2] = 02;
    }

    pub fn reset(&mut self, program: Vec<usize>) {
        self.program = program;
        self.pc = 0;
    }

    pub fn init(&mut self, noun: usize, verb: usize) {
        self.program[1] = noun;
        self.program[2] = verb;
    }

    pub fn step(&mut self) -> Opcode {
        let op = self.program[self.pc];
        match op {
            1 => {
                self.add();
                Opcode::Add
            },
            2 => {
                self.mul();
                Opcode::Mul
            },
            99 => {
                Opcode::Halt(self.halt())
            },
            _ => {
                panic!("invalid opcode");
            }
        }
    }

    pub fn run(&mut self) -> usize {
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
        let mut pc = IntCodePC::new(vec![1,9,10,3,2,3,11,0,99,30,40,50]);

        assert_eq!(pc.step(), Opcode::Add);
        assert_eq!(pc.program, vec![1,9,10,70,2,3,11,0,99,30,40,50]);
        assert_eq!(pc.pc, 4);
    }


    #[test]
    fn mul() {
        let mut pc = IntCodePC::new(vec![1,9,10,70,2,3,11,0,99,30,40,50]);
        pc.pc = 4;

        assert_eq!(pc.step(), Opcode::Mul);
        assert_eq!(pc.program, vec![3500,9,10,70,2,3,11,0,99,30,40,50]);
        assert_eq!(pc.pc, 8);
    }

    #[test]
    fn halt() {
        let mut pc = IntCodePC::new(vec![3500,9,10,70,2,3,11,0,99,30,40,50]);
        pc.pc = 8;

        assert_eq!(pc.step(), Opcode::Halt(3500));
    }

    #[test]
    fn run() {
        let mut pc = IntCodePC::new(vec![1,9,10,3,2,3,11,0,99,30,40,50]);
        assert_eq!(pc.run(), 3500);

        let mut pc = IntCodePC::new(vec![1,0,0,0,99]);
        assert_eq!(pc.run(), 2);

        let mut pc = IntCodePC::new(vec![2,3,0,3,99]);
        assert_eq!(pc.run(), 2);

        let mut pc = IntCodePC::new(vec![2,4,4,5,99,0]);
        assert_eq!(pc.run(), 2);

        let mut pc = IntCodePC::new(vec![1,1,1,4,99,5,6,0,99]);
        assert_eq!(pc.run(), 30);
    }

}
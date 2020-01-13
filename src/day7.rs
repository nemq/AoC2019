extern crate permutohedron;

use crate::day::Day;
use crate::intcode::{IntCodePC, read_program};
use permutohedron::LexicalPermutation;
use std::process::*;
use std::io::prelude::*;


pub struct Day7 {
}

impl Day7 {

    pub fn run_prog(&self, prog: &Vec<i32>, input: Vec<u8>) -> Vec<u8> {
       let mut i = input.as_slice(); 
       let mut o: Vec<u8> = Vec::new();
       let mut pc = IntCodePC::new(prog.clone(), &mut i, &mut o);
       pc.run();
       o
    }

    pub fn run_amplifiers(&self, prog: &Vec<i32>, phases: &[u8]) -> i32 {

        let a_in = vec![phases[0], b'\n', b'0', b'\n'];
        let mut b_in = vec![phases[1], b'\n'];
        let mut c_in = vec![phases[2], b'\n'];
        let mut d_in = vec![phases[3], b'\n'];
        let mut e_in = vec![phases[4], b'\n'];

        b_in.append(&mut self.run_prog(&prog, a_in));
        c_in.append(&mut self.run_prog(&prog, b_in));
        d_in.append(&mut self.run_prog(&prog, c_in));
        e_in.append(&mut self.run_prog(&prog, d_in));

        let e_out = self.run_prog(&prog, e_in);

        let fin_out = String::from_utf8(e_out).unwrap();
        fin_out.trim_end().parse::<i32>().unwrap()
    }

    pub fn run_amplifiers_chained(&self, prog_path: &String, phases: &[u8]) -> i32 {

        let mut result = String::new();
        let mut children = Vec::new();
        for (id, p) in phases.iter().enumerate() {

            let mut child = Command::new(r"target\release\intcodepc.exe")
                .arg(prog_path.clone())
                .stdin(Stdio::piped())
                .stdout(Stdio::piped())
                .spawn()
                .expect(&format!("failed to start intcodepc.exe [{}]", id));

            {
                let stdin = child.stdin.as_mut().expect(&format!("failed stdin [{}]", id));
                if id == 0 {
                    stdin.write_all(&[*p, b'\n', b'0', b'\n']).expect(&format!("failed to write to stdin [{}]", id));
                } else {
                    stdin.write_all(&[*p, b'\n']).expect(&format!("failed to write to stdin [{}]", id))
                }
            }

            children.push(child);
        }

        let mut buffer = vec![0; 255];
        let mut read;
        let mut it = (0 .. phases.len()).cycle();
        loop {
            let src_idx = it.next().unwrap();
            let dst_idx = if src_idx + 1 < phases.len() {
                src_idx + 1
            } else {
                0
            };

            {
                let source = &mut children[src_idx];
                match source.try_wait() {
                    Ok(Some(_)) => break,
                    Ok(None) => {},
                    Err(e) => panic!(e)
                }

                let stdout = source.stdout.as_mut().unwrap();
                buffer.resize(255, 0);
                read = stdout.read(&mut buffer).unwrap();
                buffer.resize(read, 0);

                if read > 0 && src_idx == children.len() -1 {
                    result = String::from_utf8(buffer.clone()).unwrap();
                }
            }

            {
                let dest = &mut children[dst_idx];
                match dest.try_wait() {
                    Ok(Some(_)) => break ,
                    Ok(None) => {},
                    Err(e) => panic!(e)
                }
                let stdin = dest.stdin.as_mut().unwrap();
                stdin.write_all(&buffer).unwrap();
            }
        }

        for mut child in children.into_iter() {
            child.wait().unwrap();
        }

        result.trim().parse().unwrap()
    }
}

impl Day for Day7 {
    fn first_puzzle(&self) -> String {

        let prog = read_program(self);
        let mut phases = [b'0', b'1', b'2', b'3', b'4'];
        let mut max_signal = self.run_amplifiers(&prog, &phases);
        while phases.next_permutation() {
            let next_signal = self.run_amplifiers(&prog, &phases);
            max_signal = i32::max(max_signal, next_signal);
        }

        format!("{}", max_signal)
    }

    fn second_puzzle(&self) -> String {

        let prog_path = String::from(self.input().to_str().unwrap());
        let mut phases = [b'5', b'6', b'7', b'8', b'9'];
        
        let mut max_signal = self.run_amplifiers_chained(&prog_path, &phases);
        while phases.next_permutation() {
            let next_signal = self.run_amplifiers_chained(&prog_path, &phases);
            max_signal = i32::max(max_signal, next_signal);
        }

        format!("{}", max_signal)
    }

    fn number(&self) -> u8 {
        7
    }
}

#[cfg(test)]
mod tests 
{
    use super::*;

    static DAY7: Day7 = Day7{};

    #[test]
    fn run_amplifiers() {

        let prog: Vec<i32> = vec![3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0];
        let phases: [u8; 5] = [b'4', b'3', b'2', b'1', b'0'];
        let output = DAY7.run_amplifiers(&prog, &phases);
        assert_eq!(output, 43210);

        let prog: Vec<i32> = vec![3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0];
        let phases: [u8; 5] = [b'0', b'1', b'2', b'3', b'4'];
        let output = DAY7.run_amplifiers(&prog, &phases);
        assert_eq!(output, 54321);

        let prog: Vec<i32> = vec![3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0];
        let phases: [u8; 5] = [b'1', b'0', b'4', b'3', b'2'];
        let output = DAY7.run_amplifiers(&prog, &phases);
        assert_eq!(output, 65210);
    }
}
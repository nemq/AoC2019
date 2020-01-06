extern crate aoc2019;

use aoc2019::intcode::IntCodePC;
use std::env::args;
use std::process::exit;
use std::fs::File;
use std::io::*;



fn usage() {
    println!("Usage: intcodepc.exe C:\\Path\\to\\program.txt");
}

fn execute(prog: Vec<i32>) -> i32 {
    
    
    let sin = stdin();
    let mut sin_lock = sin.lock();
    let mut sout = stdout();

    let mut pc = IntCodePC::new(prog, &mut sin_lock, &mut sout);
    pc.run()
}

fn read_prog(path: &str) -> Vec<i32> {

    let mut file = File::open(path).unwrap();
    let mut buf = String::new();
    file.read_to_string(&mut buf).unwrap();

    let mut prog = Vec::new();
    for i in buf.split(',').map(|s| s.trim()) {
        prog.push(i.parse().unwrap())
    }

    prog
}

fn main() {
    let mut args = args();
    match args.nth(1) {
        Some(path) => {
            let prog = read_prog(&path);
            let status = execute(prog);
            exit(status)
        },
        None =>  {
            usage();
            exit(-1);
        }
    }
}
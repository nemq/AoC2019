use std::path::{Path, PathBuf};
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

pub trait Day {
    fn first_puzzle(&self) -> String {
        String::new()
    }

    fn second_puzzle(&self) -> String {
        String::new()
    }

    fn number(&self) -> u8;

    fn first_input(&self) -> PathBuf {
        let dir = self.input_dir();
        dir.join("first.txt")
    }

    fn second_input(&self) -> PathBuf {
        let dir = self.input_dir();
        dir.join("second.txt")
    }

    fn input_dir(&self) -> PathBuf {
        let root = env::current_dir().unwrap();
        let day = format!("day{}", self.number());
        root.join("input").join(day)
    }

    fn read_input_lines<F, T> (&self, input: &Path, map: F) -> Vec<T> 
        where F: Fn(String) -> T {

        let file = File::open(input).unwrap();
        let mut reader = BufReader::new(file);
    
        let mut lines_vec = Vec::new();
    
        loop {
            let mut line = String::new();
            match reader.read_line(&mut line).unwrap() {
                0 => break,
                _ => lines_vec.push(map(line.trim_end().to_owned()))
            }
        }
    
        lines_vec
    }

    fn read_input_lines_string(&self, input: &Path) -> Vec<String> {
        let identity = |l| l;
        self.read_input_lines(input, identity)
    }
}

#[cfg(test)]
mod tests 
{

    use super::*;
    
    struct Day0 {

    }

    impl Day for Day0 {
        fn first_puzzle(&self) -> String {
            String::new()
        }

        fn second_puzzle(&self) -> String {
            String::new()
        }

        fn number(&self) -> u8 {
            0
        }
    }

    static DAY0: Day0 = Day0 {};


    #[test]
    fn first_input_exists() {
        let path = DAY0.first_input();
        assert!(path.exists());
    }

    #[test]
    fn second_input_exists() {
        let path = DAY0.second_input();
        assert!(path.exists());
    }

    #[test]
    fn read_first_input() {
        let path = DAY0.first_input();
        let lines = DAY0.read_input_lines_string(&path);
        assert_eq!(lines, vec![
            String::from("a"), 
            String::from("b"), 
            String::from("c")
        ]);
    }

    #[test]
    fn read_second_input() {
        let path = DAY0.second_input();
        let lines = DAY0.read_input_lines(&path, |l| l.parse::<u32>().unwrap());
        assert_eq!(lines, vec![1u32, 2u32, 3u32]);
    }


}




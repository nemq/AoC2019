extern crate aoc2019;

use std::env::args;

use aoc2019::day::Day;
use aoc2019::day1::Day1;
use aoc2019::day2::Day2;
use aoc2019::day3::Day3;

fn print(day: &impl Day) {
    println!("day{0}:", day.number());
    println!("\tfirst puzzle: {0}", day.first_puzzle());
    println!("\tsecond puzzle: {0}", day.second_puzzle());
}

fn main() 
{

    match args().nth(1) {
        Some(day_arg) => {
            if let Some(day) = day_arg.parse::<usize>().ok() {
                match day {
                    1 => {
                        let d = Day1{};
                        print(&d);
                    },
                    2 => {
                        let d = Day2{};
                        print(&d)
                    },
                    3 => {
                        let d = Day3{};
                        print(&d)
                    }
                    _ => {
                        println!("invalid argument");
                    }
                }
            } else {
                println!("invalid argument");
            }
        }, 
        None => {
            println!("mising argument");
        }
    }
}
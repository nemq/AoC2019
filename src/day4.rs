use crate::day::Day;


pub struct Day4 {
}

impl Day4 {

    pub fn read_bounds(&self) -> (u32, u32) {
       let path = self.input();
       let lines: Vec<String> = self.read_input_lines_string(&path);

       if lines.len() != 1 {
           panic!("input file must contain exactly one line");
       }

       let tokens: Vec<_> = lines[0].split('-').collect();
       let min: u32 = tokens[0].parse().unwrap();
       let max: u32 = tokens[1].parse().unwrap();
       (min, max)
    }
    
    fn is_six_digits(&self, num: u32) -> bool {
        100000 <= num && num <= 999999
    }

    fn has_adjacent_digits(&self, num: u32) -> bool {
        let s = format!("{}", num);

        let mut c0 = s.chars().next().unwrap();
        for c1 in s.chars().skip(1) {
            if c0 == c1 {
                return true;
            }
            else {
                c0 = c1
            }
        }

        false
    }

    fn has_adjacent_digits_pair(&self, num: u32) -> bool {
        //111122
        let s = format!("{}", num);
        let mut count = 1;
        let mut c0 = s.chars().next().unwrap();
        for c1 in s.chars().skip(1) {
            if c0 == c1 {
                count +=1;
            }
            else {
                if count == 2 {
                    return true;
                }
                else {
                    count = 1;
                    c0 = c1
                }
            }
        }

        count == 2 
    }

    fn has_nondecreasing_digits(&self, num: u32) -> bool {

        let s = format!("{}", num);

        let mut c0 = s.chars().next().unwrap();
        for c1 in s.chars().skip(1) {
            if c0 > c1 {
                return false;
            }
            else {
                c0 = c1
            }
        }

        true
    }
}


impl Day for Day4 {
    fn first_puzzle(&self) -> String {
        let (min, max) = self.read_bounds();
        let mut count = 0;
        for n in min ..= max {
            
            if !self.is_six_digits(n) {
                break;
            }

            if !self.has_nondecreasing_digits(n) {
                continue;
            }

            if !self.has_adjacent_digits(n) {
                continue;
            }

            count += 1;
        }

        format!("{}", count)
    }

    fn second_puzzle(&self) -> String {

        let (min, max) = self.read_bounds();
        let mut count = 0;
        for n in min ..= max {
            
            if !self.is_six_digits(n) {
                break;
            }

            if !self.has_nondecreasing_digits(n) {
                continue;
            }

            if !self.has_adjacent_digits_pair(n) {
                continue;
            }

            count += 1;
        }

        format!("{}", count)
    }

    fn number(&self) -> u8 {
        4
    }
}


#[cfg(test)]
mod tests 
{
    use super::*;

    static DAY4: Day4 = Day4 {};

    #[test]
    fn read_bounds() {
        let (min, max) = DAY4.read_bounds();
        assert!(min < max);
    }

    #[test]
    fn has_adjacent_digits_pair() {
        assert_eq!(DAY4.has_adjacent_digits_pair(112233), true);
        assert_eq!(DAY4.has_adjacent_digits_pair(123444), false);
        assert_eq!(DAY4.has_adjacent_digits_pair(111122), true);
    }    
    
}


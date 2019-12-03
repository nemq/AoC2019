
use crate::day::Day;


pub struct Day1 {

}

impl Day for Day1 {
    fn first_puzzle(&self) -> String {
        let mases = self.read_mases();
        let fuel_sum: u32 = mases.iter().map(|&m| self.fuel(m)).sum();

        format!("{}", fuel_sum)
    }

    fn second_puzzle(&self) -> String {
        let mases = self.read_mases();
        let fuel_sum: u32 = mases.iter().map(|&m| self.fuel_compound(m)).sum();

        format!("{}", fuel_sum)
    }

    fn number(&self) -> u8 {
        1
    }
}

impl Day1 {

    fn read_mases(&self) -> Vec<u32> {
        let path = self.first_input();
        let mases = self.read_input_lines(&path, |l| l.parse::<u32>().unwrap());
        mases
    }

    fn fuel(&self, mass: u32) -> u32 {
       (mass / 3).saturating_sub(2)
    }

    fn fuel_compound(&self, mass: u32) -> u32 {
        let mut fc = 0;
        let mut f = self.fuel(mass);
        while f > 0 {
            fc += f;
            f = self.fuel(f);
        }

        fc
    }
}

#[cfg(test)]
mod tests 
{
    use super::*;

    static DAY1: Day1 = Day1 {};
    

    #[test]
    fn fuel() {
        assert_eq!(DAY1.fuel(12), 2);
        assert_eq!(DAY1.fuel(14), 2);
        assert_eq!(DAY1.fuel(1969), 654);
        assert_eq!(DAY1.fuel(100756), 33583);
    }

    #[test]
    fn fuel_compound() {
        assert_eq!(DAY1.fuel_compound(14), 2);
        assert_eq!(DAY1.fuel_compound(1969), 966);
        assert_eq!(DAY1.fuel_compound(100756), 50346);
    }
}
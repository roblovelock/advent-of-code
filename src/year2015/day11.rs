use crate::{Day, Result};

pub struct Problem {}

impl Day for Problem {
    fn run_part1(&self, input: &str) -> Result<String> {
        Ok(input.to_owned())
    }
    fn run_part2(&self, input: &str) -> Result<String> {
        Ok(input.to_owned())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() -> Result<()> {
        let problem = Problem {};
        assert_eq!(problem.run_part1("")?, "");
        Ok(())
    }

    #[test]
    fn part2() -> Result<()> {
        let problem = Problem {};
        assert_eq!(problem.run_part2("")?, "");
        Ok(())
    }
}
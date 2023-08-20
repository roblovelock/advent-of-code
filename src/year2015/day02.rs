use itertools::Itertools;
use nom::bytes::complete::tag;
use nom::character::complete;
use nom::character::complete::line_ending;
use nom::error::Error;
use nom::multi::separated_list1;

use crate::{Day, Result};
use crate::AoCError::NomError;

pub struct Problem {}

impl Day for Problem {
    fn run_part1(&self, input: &str) -> Result<String> {
        Ok(get_dimensions(input)?
            .iter()
            .map(get_sides)
            .map(get_required_paper)
            .sum::<i32>()
            .to_string())
    }
    fn run_part2(&self, input: &str) -> Result<String> {
        Ok(get_dimensions(input)?
            .iter()
            .map(get_required_ribbon)
            .sum::<i32>()
            .to_string())
    }
}


fn get_dimensions(input: &str) -> Result<Vec<Vec<i32>>> {
    match separated_list1(line_ending::<&str, Error<&str>>, separated_list1(tag("x"), complete::i32))(input) {
        Ok(res) => Ok(res.1),
        Err(err) => Err(NomError(err.to_string())),
    }
}

fn get_sides(d: &Vec<i32>) -> Vec<i32> {
    vec!(d[0] * d[1], d[1] * d[2], d[2] * d[0])
}

fn get_required_ribbon(d: &Vec<i32>) -> i32 {
    d.iter().sorted().take(2).map(|x| x * 2).sum::<i32>() +
        d.iter().fold(1, |acc, x| acc * x)
}

fn get_required_paper(d: Vec<i32>) -> i32 {
    d.iter().map(|v| v * 2).sum::<i32>() + d.iter().min().unwrap_or(&0)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() -> Result<()> {
        let problem = Problem {};
        assert_eq!(problem.run_part1("2x3x4")?, "58");
        assert_eq!(problem.run_part1("1x1x10")?, "43");
        assert_eq!(problem.run_part1("1x1x10\n2x3x4")?, "101");
        Ok(())
    }

    #[test]
    fn part2() -> Result<()> {
        let problem = Problem {};
        assert_eq!(problem.run_part2("2x3x4")?, "34");
        assert_eq!(problem.run_part2("1x1x10")?, "14");
        assert_eq!(problem.run_part2("1x1x10\n2x3x4")?, "48");
        Ok(())
    }
}
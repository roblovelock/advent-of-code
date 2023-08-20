use itertools::FoldWhile::{Continue, Done};
use crate::{Day, Result};
use itertools::Itertools;

pub struct Problem {}

impl Day for Problem {
    fn run_part1(&self, input: &str) -> Result<String> {
        let count: i32 = input.chars().map(map_char).sum();
        Ok(count.to_string())
    }
    fn run_part2(&self, input: &str) -> Result<String> {
        Ok(input.chars().map(map_char).fold_while((0, 0), |acc, x|
            if acc.1 + x < 0 { Done((acc.0 + 1, acc.1 + x)) } else { Continue((acc.0 + 1, acc.1 + x)) },
        ).into_inner().0.to_string())
    }
}

fn map_char(c: char) -> i32 {
    if c == '(' {
        return 1;
    } else if c == ')' {
        return -1;
    }
    return 0;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() -> Result<()> {
        let day = Problem {};
        assert_eq!(day.run_part1("(())")?, "0");
        assert_eq!(day.run_part1("()()")?, "0");
        assert_eq!(day.run_part1("(((")?, "3");
        assert_eq!(day.run_part1("(()(()(")?, "3");
        assert_eq!(day.run_part1("))(((((")?, "3");
        assert_eq!(day.run_part1("())")?, "-1");
        assert_eq!(day.run_part1("))(")?, "-1");
        assert_eq!(day.run_part1(")))")?, "-3");
        assert_eq!(day.run_part1(")())())")?, "-3");
        Ok(())
    }

    #[test]
    fn part2() -> Result<()> {
        let day = Problem {};
        assert_eq!(day.run_part2(")")?, "1");
        assert_eq!(day.run_part2("()())")?, "5");
        Ok(())
    }
}
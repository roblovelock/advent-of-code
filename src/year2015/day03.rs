use std::collections::HashMap;

use crate::{Day, Result};

pub struct Problem {}


impl Day for Problem {
    fn run_part1(&self, input: &str) -> Result<String> {
        let deltas = input.chars().map(get_delta);
        let mut pos = (0, 0);
        let mut visits = HashMap::from([(pos, 1)]);

        deltas.for_each(|x| {
            pos = (pos.0 + x.0, pos.1 + x.1);
            visits.entry(pos).and_modify(|v| *v += 1).or_insert(1);
        });

        Ok(visits.len().to_string())
    }
    fn run_part2(&self, input: &str) -> Result<String> {
        let deltas = input.chars().map(get_delta);
        let mut pos = [(0, 0),(0, 0)];
        let mut visits = HashMap::from([(pos[0], 1)]);

        deltas.enumerate().for_each(|(i,x)| {
            let posi = i%2;
            pos[posi] = (pos[posi].0 + x.0, pos[posi].1 + x.1);
            visits.entry(pos[posi]).and_modify(|v| *v += 1).or_insert(1);
        });

        Ok(visits.len().to_string())
    }
}

fn get_delta(c: char) -> (i32, i32) {
    match c {
        '>' => (1, 0),
        '<' => (-1, 0),
        '^' => (0, 1),
        'v' => (0, -1),
        _ => (0, 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() -> Result<()> {
        let problem = Problem {};
        assert_eq!(problem.run_part1(">")?, "2");
        assert_eq!(problem.run_part1("^>v<")?, "4");
        assert_eq!(problem.run_part1("^v^v^v^v^v")?, "2");
        Ok(())
    }

    #[test]
    fn part2() -> Result<()> {
        let problem = Problem {};
        assert_eq!(problem.run_part2("^v")?, "3");
        assert_eq!(problem.run_part2("^>v<")?, "3");
        assert_eq!(problem.run_part2("^v^v^v^v^v")?, "11");
        Ok(())
    }
}
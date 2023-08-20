use itertools::Itertools;
use crate::{Day, Result};

pub struct Problem {}

impl Day for Problem {
    fn run_part1(&self, input: &str) -> Result<String> {
        Ok(count_nice_string(input, is_nice_string1).to_string())
    }
    fn run_part2(&self, input: &str) -> Result<String> {
        Ok(count_nice_string(input, |line| match_letter_x_letter(line) && repeat_letter_letter(line)).to_string())
    }
}

fn count_nice_string(input: &str, f: fn(&&str) -> bool) -> usize {
    input.lines().filter(f).count()
}

fn match_letter_x_letter(s: &&str) -> bool {
    for v in s.chars().tuple_windows::<(_, _, _)>() {
        if v.0 == v.2 {
            return true;
        }
    }
    return false;
}

fn repeat_letter_letter(s: &&str) -> bool {
    let mut found: Vec<(char, char)> = vec![];
    for v in s.chars().tuple_windows::<(_, _)>() {
        if found.len() > 0 {
            for f in &found[0..found.len() - 1] {
                if *f == v {
                    return true;
                }
            }
        }
        found.push(v);
    }
    return false;
}

fn is_nice_string1(s: &&str) -> bool {
    const VOWELS: [char; 5] = ['a', 'e', 'i', 'o', 'u'];
    const BAD_STRINGS: [(char, char); 4] = [('a', 'b'), ('c', 'd'), ('p', 'q'), ('x', 'y')];
    let mut vowels = 0;
    let mut repeat = 0;

    for v in s.chars().tuple_windows::<(_, _)>() {
        for bad_string in BAD_STRINGS {
            if v == bad_string {
                return false;
            }
        }
        for vowel in VOWELS {
            if v.0 == vowel {
                vowels += 1;
                break;
            }
        }

        if v.0 == v.1 {
            repeat += 1;
        }
    }

    for vowel in VOWELS {
        if s.chars().last().unwrap() == vowel {
            vowels += 1;
            break;
        }
    }

    return vowels >= 3 && repeat > 0;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() -> Result<()> {
        let problem = Problem {};
        assert_eq!(problem.run_part1("ugknbfddgicrmopn")?, "1");
        assert_eq!(problem.run_part1("aaa")?, "1");
        assert_eq!(problem.run_part1("jchzalrnumimnmhp")?, "0");
        assert_eq!(problem.run_part1("haegwjzuvuyypxyu")?, "0");
        assert_eq!(problem.run_part1("dvszwmarrgswjxmb")?, "0");
        Ok(())
    }

    #[test]
    fn part2() -> Result<()> {
        let problem = Problem {};
        assert_eq!(problem.run_part2("qjhvhtzxzqqjkmpb")?, "1");
        assert_eq!(problem.run_part2("xxyxx")?, "1");
        assert_eq!(problem.run_part2("uurcxstgmygtbstg")?, "0");
        assert_eq!(problem.run_part2("ieodomkazucvgmuy")?, "0");
        Ok(())
    }
}
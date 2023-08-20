use crate::{Day, Result};

pub struct Problem {}

impl Day for Problem {
    fn run_part1(&self, input: &str) -> Result<String> {
        const MASK: [u8; 3] = [0x00, 0x00, 0x0F];
        Ok(calc_md5_number(input, &MASK).to_string())
    }
    fn run_part2(&self, input: &str) -> Result<String> {
        const MASK: [u8; 3] = [0x00, 0x00, 0x00];
        Ok(calc_md5_number(input, &MASK).to_string())
    }
}

fn calc_md5_number(prefix: &str, mask: &[u8; 3]) -> u32 {
    for i in 1..u32::MAX {
        let digest = md5::compute(format!("{}{}", prefix, i).as_bytes());
        if digest.iter().take(3).zip(mask.iter()).filter(|(x1, x2)| (*x1 | *x2) == **x2).count() == 3 {
            return i;
        }
    }
    return 0;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() -> Result<()> {
        let problem = Problem {};
        assert_eq!(problem.run_part1("abcdef")?, "609043");
        assert_eq!(problem.run_part1("pqrstuv")?, "1048970");
        Ok(())
    }
}
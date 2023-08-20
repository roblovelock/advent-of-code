use std::{fs, io, result};
use thiserror::Error;

mod year2015;

#[derive(Error, Debug)]
pub enum AoCError {
    #[error("Invalid argument: {0}={1}")]
    ArgError(&'static str, String),
    #[error("Could not read: {0}")]
    IoError(#[from] io::Error),
    #[error("Nom error: {0}")]
    NomError(String),
}

pub type Result<T> = result::Result<T, AoCError>;

pub trait Year {
    fn get_day(&self, day: &str) -> Result<Box<dyn Day>>;
}

pub trait Day {
    fn run_part1(&self, input: &str) -> Result<String>;
    fn run_part2(&self, input: &str) -> Result<String>;
}

fn get_year(year: &str) -> Result<Box<dyn Year>> {
    match year {
        "2015" => Ok(Box::new(year2015::Year2015 {})),
        _ => Err(AoCError::ArgError("year", year.to_string())),
    }
}

fn get_input(year: &str, day: &str) -> Result<String> {
    fs::read_to_string(format!("./inputs/{}/day{:0>2}.dat", year, day))
        .map_err(|err| AoCError::IoError(err))
}

pub fn run(year: &str, day: &str, part: &str) -> Result<String> {
    let d = get_year(year)?.get_day(day)?;
    let input = get_input(year, day)?;
    match part {
        "1" => d.run_part1(&input),
        "2" => d.run_part2(&input),
        _ => Err(AoCError::ArgError("part", part.to_string()))
    }
}
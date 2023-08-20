use crate::{AoCError, Day, Result, Year};

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;

pub struct Year2015 {}

impl Year for Year2015 {
    fn get_day(&self, day: &str) -> Result<Box<dyn Day>> {
        match day {
            "1" => Ok(Box::new(day01::Problem {})),
            "2" => Ok(Box::new(day02::Problem {})),
            "3" => Ok(Box::new(day03::Problem {})),
            "4" => Ok(Box::new(day04::Problem {})),
            "5" => Ok(Box::new(day05::Problem {})),
            "6" => Ok(Box::new(day06::Problem {})),

            _ => Err(AoCError::ArgError("day", day.to_string())),
        }
    }
}
use std::env::args;
use std::process;
use advent_of_code::run;

fn main() {
    let a: Vec<String> = args().collect();
    if a.len() != 4 {
        println!("Expected 3 arguments: <year> <day> <part>");
        process::exit(1)
    }

    match run(&a[1], &a[2], &a[3]) {
        Ok(r) => println!("Result: {r}"),
        Err(err) => println!("Failed: {err}"),
    }
}

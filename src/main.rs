use std::error::Error;
use crate::day_one::day_one;
use crate::day_three::day_three;
use crate::day_two::day_two;

mod day_one;
mod day_two;
mod day_three;

fn main() -> Result<(), Box<dyn Error>> {
    println!("Advent of Code 2024");
    println!("------------------------------");
    day_one()?;
    day_two()?;
    day_three()?;
    println!("------------------------------");
    Ok(())
}

use std::error::Error;
use regex::Regex;

pub fn day_three() -> Result<(), Box<dyn Error>> {
    let content = std::fs::read_to_string("inputs/day_three.txt")?;
    
    let part_one = part_one(&content)?;
    let part_two = part_two(&content)?;
    println!("Day 3");
    println!("  Part 1: {part_one}");
    println!("  Part 2: {part_two}");
    Ok(())
}

fn part_one(content: &str) -> Result<i64, Box<dyn Error>> {
    let regex = Regex::new(r#"mul\((\d+),(\d+)\)"#)?;
    
    let mut sum = 0i64;
    
    for (_, [op_one, op_two]) in regex.captures_iter(content).map(|c| c.extract()) {
        sum += op_one.parse::<i64>()? * op_two.parse::<i64>()?;
    }
    
    Ok(sum)
}

fn part_two(content: &str) -> Result<i64, Box<dyn Error>> {
    let regex = Regex::new(r#"(do|don't|mul)\((?:(\d+),(\d+))?\)"#)?;

    let mut sum = 0i64;
    let mut mul_enabled = true;
    
    for captures in regex.captures_iter(content) {
        let fn_name = captures.get(1).map(|m| m.as_str()).unwrap_or("");
        let op_one = captures.get(2).map(|m| m.as_str()).unwrap_or("");
        let op_two = captures.get(3).map(|m| m.as_str()).unwrap_or("");
        
        match fn_name {
            "do" => mul_enabled = true,
            "don't" => mul_enabled = false,
            "mul" => {
                if !mul_enabled {
                    continue;
                }
                if let (Ok(op_one), Ok(op_two)) = (op_one.parse::<i64>(), op_two.parse::<i64>()) {
                    sum += op_one * op_two;
                }
            }
            _ => {}
        }
    }
    Ok(sum)
}
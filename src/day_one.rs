use std::error::Error;

pub fn day_one() -> Result<(), Box<dyn Error>> {
    let content = std::fs::read_to_string("inputs/day_one.txt")?;
    let (left, right) = parse_input(content)?;
    
    let part_one = part_one(left.clone(), right.clone());
    let part_two = part_two(left, right);

    println!("Day 1");
    println!("  Part 1: {part_one}");
    println!("  Part 2: {part_two}");
    Ok(())
}

fn part_one(mut left: Vec<i32>, mut right: Vec<i32>) -> i32 {
    left.sort();
    right.sort();
    
    left.iter()
        .zip(right.iter())
        .map(|(l, r)| (l - r).abs()) // Calculate the absolute difference directly
        .sum()
}

fn part_two(left: Vec<i32>, right: Vec<i32>) -> i32 {
    let mut similarity_score = 0;

    for left_number in left {
        let count_left_in_right = right.iter().filter(|x| **x == left_number).count();
        similarity_score += count_left_in_right as i32 * left_number;
    }

    similarity_score
}

fn parse_input(content: String) -> Result<(Vec<i32>, Vec<i32>), Box<dyn Error>> {
    let regex = regex::Regex::new(r#"(\d+)\s+(\d+)"#)?;
    let lines = content.lines();
    let mut vec_left: Vec<i32> = Vec::new();
    let mut vec_right: Vec<i32> = Vec::new();

    for line in lines {
        for (_, [left, right]) in regex.captures_iter(line).map(|c| c.extract()) {
            vec_left.push(left.parse()?);
            vec_right.push(right.parse()?);
        }
    }

    Ok((vec_left, vec_right))
}
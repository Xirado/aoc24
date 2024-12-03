use std::error::Error;

pub fn day_two() -> Result<(), Box<dyn Error>> {
    let content = std::fs::read_to_string("inputs/day_two.txt")?;
    let input = parse_input(content)?;

    let part_one = part_one(&input);
    let part_two = part_two(&input);

    println!("Day 2");
    println!("  Part 1: {part_one}");
    println!("  Part 2: {part_two}");
    Ok(())
}

fn part_one(reactors: &Vec<Vec<i32>>) -> u32 {
    let mut safe = 0;
    for levels in reactors {
        if is_reactor_safe(levels) {
            safe += 1;
        }
    }

    safe
}

fn part_two(reactors: &Vec<Vec<i32>>) -> u32 {
    let mut safe = 0;
    'outer: for levels in reactors {
        if is_reactor_safe(levels) {
            safe += 1;
            continue;
        }

        for index in 0..levels.len() {
            let removed = [&levels[..index], &levels[index + 1..]].concat();
            if is_reactor_safe(&removed) {
                safe += 1;
                continue 'outer;
            }
        }
    }

    safe
}

fn is_reactor_safe(levels: &[i32]) -> bool {
    let mut ascending: Option<bool> = None;
    for window in levels.windows(2) {
        let first = window[0];
        let second = window[1];
        let diff = (first - second).abs();

        if !(1..=3).contains(&diff) {
            return false
        }

        match ascending {
            None => ascending = Some(second > first),
            Some(ascending) => {
                if ascending && first > second || !ascending && second > first {
                    return false;
                }
            }
        }
    }

    true
}

fn parse_input(content: String) -> Result<Vec<Vec<i32>>, Box<dyn Error>> {
    content
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|num| num.parse::<i32>().map_err(From::from))
                .collect()
        })
        .collect()
}
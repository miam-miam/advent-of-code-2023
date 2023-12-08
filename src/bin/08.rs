use std::collections::HashMap;
use prse::{Parse, parse};

#[derive(Parse, Debug, PartialOrd, PartialEq)]
enum Direction {
    #[prse = "L"]
    Left,
    #[prse = "R"]
    Right
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut iter = input.lines();
    let input = iter.next()?;
    let directions: Vec<Direction> = parse!(input, "{::}");
    iter.next()?;

    let hash: HashMap<&str, (&str, &str)> = iter.map(|l| {
        let (input, left, right): (&str, &str, &str) = parse!(l, "{} = ({}, {})");
        (input, (left, right))
    }).collect();

    let mut current_pos = "AAA";
    for (steps, dir) in directions.iter().cycle().enumerate() {
        if current_pos == "ZZZ" {
            return Some(steps);
        }
        current_pos = match dir {
            Direction::Left => {hash.get(current_pos)?.0}
            Direction::Right => {hash.get(current_pos)?.1}
        }
    }
    None
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut iter = input.lines();
    let input = iter.next()?;
    let directions: Vec<Direction> = parse!(input, "{::}");
    iter.next()?;

    let hash: HashMap<&str, (&str, &str)> = iter.map(|l| {
        let (input, left, right): (&str, &str, &str) = parse!(l, "{} = ({}, {})");
        (input, (left, right))
    }).collect();

    Some(hash.keys().filter(|s| s.ends_with('A')).copied().map(|pos| {
        let mut current_pos = pos;
        for (steps, dir) in directions.iter().cycle().enumerate() {
            if current_pos.ends_with('Z') {
                return steps / directions.len();
            }
            current_pos = match dir {
                Direction::Left => {hash.get(current_pos).unwrap().0}
                Direction::Right => {hash.get(current_pos).unwrap().1}
            }
        }
        0_usize
    }).product::<usize>() * directions.len())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 8);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_one(&input), Some(6));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_two(&input), Some(6));
    }
}

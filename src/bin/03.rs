use itertools::Itertools;

const DIAGONALS: [(i32, i32); 8] = [(-1, 0), (1, 0), (0, -1), (0, 1), (-1, -1), (1, 1), (1, -1) ,(-1, 1)];

#[derive(Ord, PartialOrd, Eq, PartialEq)]
enum Character {
    Number(usize),
    Dot,
    Gear,
    Symbol,
}

fn next_to_symbol(grid: &Vec<Vec<Character>>, x: i32, y: i32) -> bool {
    if 0 <= x && x < grid.len() as i32 && 0 <= y && y < grid[0].len() as i32 {
        grid[x as usize][y as usize] == Character::Symbol
    } else {
        false
    }
}

fn next_to_gear(grid: &Vec<Vec<Character>>, x: i32, y: i32) -> Option<usize> {
    if 0 <= x && x < grid.len() as i32 && 0 <= y && y < grid[0].len() as i32 {
        match grid[x as usize][y as usize]  {
            Character::Number(num) => Some(num),
            _ => None
        }
    } else {
        None
    }
}


pub fn part_one(input: &str) -> Option<u32> {
    let mut nums = vec![];
    let grid = input.lines().map(|l| {
        let mut num = 0;
        let v = l.chars().map(|c| {
            if c.is_ascii_digit() {
                num = num * 10 + (c as u32 - '0' as u32);
                Character::Number(nums.len())
            } else {
                if num != 0 {
                    nums.push(num);
                    num = 0
                }
                if c == '.' {
                    Character::Dot
                } else {
                    Character::Symbol
                }
            }
        }).collect_vec();
        if num != 0 {
            nums.push(num);
        }
        v
    }).collect_vec();

    let g = &grid;

    Some(grid.iter().enumerate().flat_map(|(x, l)| {
        l.iter().enumerate().filter_map(move |(y, c)| {
            let (x, y) = (x as i32, y as i32);
            match c {
                Character::Number(idx) => {
                    if DIAGONALS.iter().any(|(dx, dy)| next_to_symbol(g, x + dx, y + dy)) {
                        Some(idx)
                    } else {
                        None
                    }
                }
                _ => None
            }
        })
    }).unique().map(|idx| nums[*idx]).sum())
}

pub fn part_two(input: &str) -> Option<u32> {

    let mut nums = vec![];
    let grid = input.lines().map(|l| {
        let mut num = 0;
        let v = l.chars().map(|c| {
            if c.is_ascii_digit() {
                num = num * 10 + (c as u32 - '0' as u32);
                Character::Number(nums.len())
            } else {
                if num != 0 {
                    nums.push(num);
                    num = 0
                }
                if c == '.' {
                    Character::Dot
                } else if c == '*' {
                    Character::Gear
                } else {
                    Character::Symbol
                }
            }
        }).collect_vec();
        if num != 0 {
            nums.push(num);
        }
        v
    }).collect_vec();

    let g = &grid;
    let n = &nums;

    Some(grid.iter().enumerate().flat_map(|(x, l)| {
        l.iter().enumerate().filter_map(move |(y, c)| {
            let (x, y) = (x as i32, y as i32);
            match c {
                Character::Gear => {
                    let (num1, num2) = DIAGONALS.iter().filter_map(|(dx, dy)| {
                        next_to_gear(g, x + dx, y + dy)
                    }).unique().collect_tuple()?;
                    Some(n[num1] * n[num2])
                }
                _ => None
            }
        })
    }).sum())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 3);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_one(&input), Some(4361));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_two(&input), Some(467835));
    }
}

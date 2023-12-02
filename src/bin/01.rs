pub fn part_one(input: &str) -> Option<u32> {
    Some(input.lines().map(|l| {
        let first = l.chars().find(char::is_ascii_digit).unwrap() as u32 - '0' as u32;
        let last = l.chars().rfind(char::is_ascii_digit).unwrap() as u32 - '0' as u32;
        first * 10 + last
    }).sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut input = input.replace("one", "o1e");
    input = input.replace("two", "t2o");
    input = input.replace("three", "t3e");
    input = input.replace("four", "f4r");
    input = input.replace("five", "f5e");
    input= input.replace("six", "s6x");
    input = input.replace("seven", "s7n");
    input = input.replace("eight", "e8t");
    input = input.replace("nine", "n9e");
    part_one(&input)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 1);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        assert_eq!(part_one("1abc2\npqr3stu8vwx\na1b2c3d4e5f\ntreb7uchet"), Some(142));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_two(&input), Some(281));
    }
}

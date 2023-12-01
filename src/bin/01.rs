pub fn part_one(input: &str) -> Option<u32> {
    Some(input.lines().map(|l| {
        let first = l.chars().find(char::is_ascii_digit).unwrap() as u32 - '0' as u32;
        let last = l.chars().rfind(char::is_ascii_digit).unwrap() as u32 - '0' as u32;
        first * 10 + last
    }).sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut input = input.replace("one", "one1one");
    input = input.replace("two", "two2two");
    input = input.replace("three", "three3three");
    input = input.replace("four", "four4four");
    input = input.replace("five", "five5five");
    input= input.replace("six", "six6six");
    input = input.replace("seven", "seven7seven");
    input = input.replace("eight", "eight8eight");
    input = input.replace("nine", "nine9nine");
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

use prse::parse;

pub fn part_one(input: &str) -> Option<u32> {
    Some(input.replace("   ", " ").replace("  ", " ").lines().map(|l| {
        let (_game, winning, numbers): (u32, Vec<u32>, _) = parse!(l, "Card {}: {: :} | {: :0}");
        let count = numbers.map(|n| n.unwrap()).filter(|n| winning.contains(n)).count();
        if count == 0 {
            0
        } else {
            2_u32.pow(count as u32 - 1)
        }
    }).sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut cards = vec![1; input.lines().count()];
    input.replace("   ", " ").replace("  ", " ").lines().for_each(|l| {
        let (card, winning, numbers): (usize, Vec<u32>, _) = parse!(l, "Card {}: {: :} | {: :0}");
        let count = numbers.map(|n| n.unwrap()).filter(|n| winning.contains(n)).count();
        if count != 0 {
            for idx in card..(count + card) {
                cards[idx] += cards[card - 1]
            }
        }
    });
    Some(cards.iter().sum())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 4);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_one(&input), Some(13));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_two(&input), Some(30));
    }
}

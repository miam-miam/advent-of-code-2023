use std::cmp::max;
use prse::{Parse, parse};


#[derive(Parse)]
enum Colour {
    #[prse = "red"]
    Red,
    #[prse = "green"]
    Green,
    #[prse = "blue"]
    Blue,
}

#[derive(Parse)]
#[prse = "{count} {colour}"]
struct Cube {
    count: u32,
    colour: Colour
}

#[derive(Parse)]
#[prse = "{0:, :}"]
struct Round(Vec<Cube>);

pub fn part_one(input: &str) -> Option<u32> {
    Some(input.lines().filter_map(|g| {
        let (game, mut rounds): (u32, _) = parse!(g, "Game {}: {:; :0}");
        if rounds.any(|r: Result<Round, _>| r.unwrap().0.into_iter().any(|c| {
            let limit = match c.colour {
                Colour::Red => 12,
                Colour::Green => 13,
                Colour::Blue => 14,
            };
            c.count > limit
        })) {
            return None;
        }
        Some(game)
    }).sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(input.lines().map(|g| {
        let (_game, rounds): (u32, _) = parse!(g, "Game {}: {:; :0}");
        let totals = rounds.into_iter().fold((0, 0, 0), |mut round_totals, r: Result<Round, _>| {
            let colours = r.unwrap().0.into_iter().fold((0, 0, 0), |mut acc, c| {
                match c.colour {
                    Colour::Red => acc.0 = c.count,
                    Colour::Green => acc.1 = c.count,
                    Colour::Blue => acc.2 = c.count,
                };
                acc
            });
            round_totals.0 = max(round_totals.0, colours.0);
            round_totals.1 = max(round_totals.1, colours.1);
            round_totals.2 = max(round_totals.2, colours.2);
            round_totals
        });
        totals.0 * totals.1 * totals.2
    }).sum())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 2);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_one(&input), Some(8));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_two(&input), Some(2286));
    }
}

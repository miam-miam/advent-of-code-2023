use itertools::Itertools;
use prse::{Parse, parse};

#[derive(Parse)]
#[prse = "{destination} {source} {length}"]
struct Range {
    destination: i64,
    source: i64,
    length: i64,
}

#[derive(Parse)]
#[prse = "{_input}-to-{_output} map:\n{ranges:\n:}"]
struct Map {
    _input: String,
    _output: String,
    ranges: Vec<Range>
}


pub fn part_one(input: &str) -> Option<u32> {
    let (seeds, maps): (Vec<i64>, Vec<Map>) = parse!(input, "seeds: {: :}\n\n{:\n\n:}");
    seeds.iter().map(|s| {
        let mut new_seed = *s;
        for map in maps.iter() {
            new_seed = map.ranges.iter().find_map(|r| if r.source <= new_seed && r.source + r.length > new_seed {
                Some(new_seed - r.source + r.destination)
            } else {None}).unwrap_or(new_seed);
        }
        new_seed as u32
    }).min()
}

pub fn part_two(input: &str) -> Option<u32> {
    let (seeds, maps): (Vec<i64>, Vec<Map>) = parse!(input, "seeds: {: :}\n\n{:\n\n:}");
    let seed_ranges: Vec<(_, _)> = seeds.iter().tuples().collect_vec();
    (0..).find(|s| {
        let mut new_seed = *s;
        for map in maps.iter().rev() {
            new_seed = map.ranges.iter().find_map(|r| if r.destination <= new_seed && r.destination + r.length > new_seed {
                Some(new_seed - r.destination + r.source)
            } else {None}).unwrap_or(new_seed);
        }
        seed_ranges.iter().any(|(start, length)| {
            **start <= new_seed && *start + *length > new_seed
        })
    }).map(|s| s as u32)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 5);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_one(&input), Some(35));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_two(&input), Some(46));
    }
}

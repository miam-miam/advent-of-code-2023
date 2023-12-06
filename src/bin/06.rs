use prse::parse;
use regex::Regex;


const TINY_BITS: u64 = 0x1; // Smallest positive f64.
const NEG_TINY_BITS: u64 = 0x8000_0000_0000_0001; // Smallest (in magnitude) negative f64.
const CLEAR_SIGN_MASK: u64 = 0x7fff_ffff_ffff_ffff;

// From unstable std feature
pub fn next_int_up(f: f64) -> f64 {
    let bits = f.to_bits();
    let abs = bits & CLEAR_SIGN_MASK;
    let next_bits = if abs == 0 {
        TINY_BITS
    } else if bits == abs {
        bits + 1
    } else {
        bits - 1
    };
    f64::from_bits(next_bits).ceil()
}

// From unstable std feature
pub fn next_int_down(f: f64) -> f64 {
    let bits = f.to_bits();
    let abs = bits & CLEAR_SIGN_MASK;
    let next_bits = if abs == 0 {
        NEG_TINY_BITS
    } else if bits == abs {
        bits - 1
    } else {
        bits + 1
    };
    f64::from_bits(next_bits).floor()
}

pub fn part_one(input: &str) -> Option<u32> {
    let input = Regex::new(" +").ok()?.replace_all(input, " ");
    let (time, distance) = parse!(input, "Time: {: :0}\nDistance: {: :0}\n");
    time.zip(distance).map(|(t, d)| (t.unwrap(), d.unwrap())).map(|(t, d) : (u32, u32)| {
        let a = -1_f64;
        let b = t as f64;
        let c = -(d as f64);
        let delta = (b * b) - (4.0 * a * c);
        let x1 = ((-b) + f64::sqrt(delta)) / (2.0 * a);
        let x2 = ((-b) - f64::sqrt(delta)) / (2.0 * a);
        let num = (next_int_down(x2) as i32 - next_int_up(x1) as i32) + 1;
        num as u32
    }).reduce(|m, n| m * n)
}

pub fn part_two(input: &str) -> Option<u32> {
    let input = Regex::new(" +").ok()?.replace_all(input, " ");
    let (time, distance): (&str, &str) = parse!(input, "Time: {}\nDistance: {}\n");
    let t: f64 = parse!(time.replace(' ', ""), "{}");
    let d: f64 = parse!(distance.replace(' ', ""), "{}");
    let a = -1_f64;
    let b = t;
    let c = -d;
    let delta = (b * b) - (4.0 * a * c);
    let x1 = ((-b) + f64::sqrt(delta)) / (2.0 * a);
    let x2 = ((-b) - f64::sqrt(delta)) / (2.0 * a);
    let num = (next_int_down(x2) as i32 - next_int_up(x1) as i32) + 1;
    Some(num as u32)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 6);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_one(&input), Some(288));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_two(&input), Some(71503));
    }
}

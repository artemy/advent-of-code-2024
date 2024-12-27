use regex::Regex;
use std::fs;

pub(crate) fn run() {
    let input =
        fs::read_to_string("input/day_03.txt").expect("Something went wrong reading the file");
    println!("Day 03");
    println!("->Part 01: {}", part_01(&input));
    println!("->Part 02: {}", part_02(&input));
}

fn part_01(input: &str) -> i32 {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    re.captures_iter(input)
        .map(|c| c.extract())
        .map(|(_, [left, right])| left.parse::<i32>().unwrap() * right.parse::<i32>().unwrap())
        .sum()
}

fn part_02(input: &str) -> i32 {
    let re = Regex::new(r"mul\((\d+),(\d+)\)|do\(\)|don't\(\)").unwrap();
    let mut enabled = true;
    re.captures_iter(input)
        .map(|caps| {
            match (&caps[0], enabled) {
                ("do()", _) => {
                    enabled = true;
                    0
                }
                ("don't()", _) => {
                    enabled = false;
                    0
                }
                (_, true) => {
                    let left = caps.get(1).unwrap().as_str();
                    let right = caps.get(2).unwrap().as_str();
                    left.parse::<i32>().unwrap() * right.parse::<i32>().unwrap()
                }
                _ => 0,
            }
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_01: &str =
        "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
    const INPUT_02: &str =
        "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

    #[test]
    fn test_part_01() {
        assert_eq!(part_01(INPUT_01), 161);
    }
    #[test]
    fn test_part_02() {
        assert_eq!(part_02(INPUT_02), 48);
    }
}

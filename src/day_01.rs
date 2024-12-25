use std::collections::HashMap;
use std::fs;
use std::iter::zip;

pub(crate) fn run() {
    let input =
        fs::read_to_string("input/day_01.txt").expect("Something went wrong reading the file");
    println!("Part 01: {}", part_01(input.to_string()));
    println!("Part 02: {}", part_02(input.to_string()));
}

fn parse_input(input: &str) -> (Vec<i32>, Vec<i32>) {
    let numbers = input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|x| x.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>();
    let first = numbers.iter().map(|x| x[0]).collect::<Vec<i32>>();
    let second = numbers.iter().map(|x| x[1]).collect::<Vec<i32>>();
    (first, second)
}

fn part_01(input: String) -> i32 {
    let (mut first, mut second) = parse_input(&input);

    first.sort();
    second.sort();

    zip(first, second).fold(0, |acc, (f, s)| acc + i32::abs(f - s))
}

fn part_02(input: String) -> i32 {
    let (first, second) = parse_input(&input);

    let mut count_numbers = HashMap::new();

    for &num in &second {
        *count_numbers.entry(num).or_insert(0) += 1;
    }

    first
        .into_iter()
        .fold(0, |acc, num| acc + count_numbers.get(&num).unwrap_or(&0) * num)
}
#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "3   4
4   3
2   5
1   3
3   9
3   3";

    #[test]
    fn test_part_01() {
        assert_eq!(part_01(INPUT.to_string()), 11);
    }
    #[test]
    fn test_part_02() {
        assert_eq!(part_02(INPUT.to_string()), 31);
    }
}

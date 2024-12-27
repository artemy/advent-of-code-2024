use std::fs;

pub(crate) fn run() {
    let input =
        fs::read_to_string("input/day_02.txt").expect("Something went wrong reading the file");
    println!("Day 02");
    println!("->Part 01: {}", part_01(&input));
    println!("->Part 02: {}", part_02(&input));
}

fn parse_input(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|x| x.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>()
}

fn part_01(input: &str) -> i32 {
    parse_input(input)
        .iter()
        .map(|x| is_safe(x))
        .fold(0, |acc, x| acc + x as i32)
}

fn is_safe(v: &[i32]) -> bool {
    let differences: Vec<i32> = v.windows(2).map(|w| w[0] - w[1]).collect();
    let all_increasing = differences.iter().all(|&x| (1..=3).contains(&x));
    let all_decreasing = differences.iter().all(|&x| (-3..=-1).contains(&x));
    all_increasing || all_decreasing
}

fn part_02(input: &str) -> i32 {
    parse_input(input)
        .iter()
        .map(|v| {
            if is_safe(v) {
                return true;
            }

            for i in 0..v.len() {
                let mut new_v = v.clone();
                new_v.remove(i);
                if is_safe(&new_v) {
                    return true;
                }
            }

            false
        })
        .filter(|&is_safe| is_safe)
        .count() as i32
}
#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

    #[test]
    fn test_part_01() {
        assert_eq!(part_01(INPUT), 2);
    }
    #[test]
    fn test_part_02() {
        assert_eq!(part_02(INPUT), 4);
    }
}

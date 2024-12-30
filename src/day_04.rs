use std::cmp::{max, min};
use std::fs;

pub(crate) fn run() {
    let input =
        fs::read_to_string("input/day_04.txt").expect("Something went wrong reading the file");
    println!("Day 04");
    println!("->Part 01: {}", part_01(&input));
    println!("->Part 02: {}", part_02(&input));
}

fn parse_input(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn generate_coords_p01(x_pos: i32, y_pos: i32, x_step: i32, y_step: i32) -> Vec<(i32, i32)> {
    (1..=3)
        .map(|i| (x_pos + i * x_step, y_pos + i * y_step))
        .collect()
}

fn find_adjacent_p01(data: &[Vec<char>], x_pos: i32, y_pos: i32) -> Vec<Vec<char>> {
    let max_y = data.len() as i32;
    let max_x = data.iter().map(|x| x.len() as i32).max().unwrap();

    [
        |x_pos: i32, y_pos: i32| generate_coords_p01(x_pos, y_pos, 1, 0),
        |x_pos: i32, y_pos: i32| generate_coords_p01(x_pos, y_pos, -1, 0),
        |x_pos: i32, y_pos: i32| generate_coords_p01(x_pos, y_pos, 0, 1),
        |x_pos: i32, y_pos: i32| generate_coords_p01(x_pos, y_pos, 0, -1),
        |x_pos: i32, y_pos: i32| generate_coords_p01(x_pos, y_pos, 1, -1),
        |x_pos: i32, y_pos: i32| generate_coords_p01(x_pos, y_pos, 1, 1),
        |x_pos: i32, y_pos: i32| generate_coords_p01(x_pos, y_pos, -1, 1),
        |x_pos: i32, y_pos: i32| generate_coords_p01(x_pos, y_pos, -1, -1),
    ]
    .iter()
    .map(|f| f(x_pos, y_pos))
    .filter(|coords| {
        coords
            .iter()
            .all(|&(x, y)| (0..max_x).contains(&x) && (0..max_y).contains(&y))
    })
    .map(|coords| {
        coords
            .iter()
            .map(|(x, y)| (*x as usize, *y as usize))
            .map(|(x, y)| data[x][y])
            .collect::<Vec<char>>()
    })
    .collect()
}
fn part_01(input: &str) -> i32 {
    let data = parse_input(input);

    let mut count: usize = 0;

    for x in 0..data.len() {
        for y in 0..data[x].len() {
            if data[x][y] == 'X' {
                let vec = find_adjacent_p01(&data, x as i32, y as i32);
                count += vec
                    .iter()
                    .filter(|&v| v.iter().zip(['M', 'A', 'S']).all(|(&a, b)| a == b))
                    .count();
            }
        }
    }

    count as i32
}

fn find_adjacent_p02(data: &[Vec<char>], x_pos: usize, y_pos: usize) -> [[char; 2]; 2] {
    [
        [
            min(data[x_pos - 1][y_pos - 1], data[x_pos + 1][y_pos + 1]),
            max(data[x_pos - 1][y_pos - 1], data[x_pos + 1][y_pos + 1]),
        ],
        [
            min(data[x_pos - 1][y_pos + 1], data[x_pos + 1][y_pos - 1]),
            max(data[x_pos - 1][y_pos + 1], data[x_pos + 1][y_pos - 1]),
        ],
    ]
}

fn part_02(input: &str) -> i32 {
    let data = parse_input(input);

    let mut count: usize = 0;

    for x in 1..data.len() - 1 {
        for y in 1..data[x].len() - 1 {
            if data[x][y] == 'A' {
                let vec = find_adjacent_p02(&data, x, y);
                count += vec.iter().all(|&c| c == ['M', 'S']) as usize;
            }
        }
    }

    count as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";
    #[test]
    fn test_part_01() {
        assert_eq!(part_01(INPUT), 18);
    }
    #[test]
    fn test_part_02() {
        assert_eq!(part_02(INPUT), 9);
    }
}

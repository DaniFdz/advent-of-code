use std::collections::HashSet;
use itertools::Itertools;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Point {
    x: usize,
    y: usize,
}

pub fn process(input: &str) -> u64 {
    process_data(input, 1000000)
}

fn process_data(input: &str, scale: u64) -> u64 {
    let mut points: Vec<Point> = Vec::new();
    input.lines().enumerate().for_each(|(y, line)| {
        if line.chars().any(|c| c == '#') {
            line.chars().enumerate().for_each(|(x, c)| {
                if c == '#' {
                    points.push(Point { x, y });
                }
            });
        }
    });

    let empty_rows = input
        .lines()
        .enumerate()
        .filter_map(|(i, line)| match line.chars().all(|c| c == '.') {
            true => Some(i),
            false => None,
        })
        .collect::<HashSet<usize>>();

    let empty_cols = (0..input.lines().nth(0).unwrap().len())
        .filter(|i| input.lines().all(|line| line.chars().nth(*i).unwrap() == '.'))
        .collect::<HashSet<usize>>();

    let mut total: u64 = 0;

    points.iter().tuple_combinations().for_each(|(p1, p2)| {
        for x in (p1.x.min(p2.x))..(p1.x.max(p2.x)) {
            total += match empty_cols.contains(&x) {
                true => scale,
                false => 1
            }
        }
        for y in (p1.y.min(p2.y))..(p1.y.max(p2.y)) {
            total += match empty_rows.contains(&y) {
                true => scale,
                false => 1
            }
        }
    });

    total
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_part2(){
        let result = process_data("...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....", 2);
        assert_eq!(result, 374);
        let result = process_data("...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....", 10);
        assert_eq!(result, 1030);
        let result = process_data("...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....", 100);
        assert_eq!(result, 8410);
    }
}



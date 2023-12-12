use std::collections::HashSet;
use itertools::Itertools;


#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Point {
    x: usize,
    y: usize,
}

pub fn process(input: &str) -> u32 {
    let mut points: Vec<Point> = Vec::new();
    input.lines().enumerate().for_each(|(y, line)|{
        if line.chars().any(|c| c == '#') {
            line.chars().enumerate().for_each(|(x, c)|{
                if c == '#' {
                    points.push(Point{x, y});
                }
            });
        }
    });

    let empty_rows = input.lines().enumerate().filter_map(|(i,line)| match line.chars().all(|c| c == '.') {
        true => Some(i),
        false => None,
    }).collect::<HashSet<usize>>();

    let empty_cols = (0..input.lines().nth(0).unwrap().len()).filter(|i| {
        input.lines().all(|line| line.chars().nth(*i).unwrap() == '.')
    }).collect::<HashSet<usize>>();

    let mut total = 0;
    let scale = 2;

    points.iter().tuple_combinations().for_each(|(p1,p2)|{
        for i in p1.x.min(p2.x)..p2.x.max(p1.x) {
            total += match empty_cols.contains(&i) {
                true => scale,
                false => 1,
            };
        }
        for j in p1.y.min(p2.y)..p2.y.max(p1.y) {
            total += match empty_rows.contains(&j) {
                true => scale,
                false => 1,
            };
        }
    });

    total
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_part1(){
        let result = process("...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....");
        assert_eq!(result, 374);
    }
}


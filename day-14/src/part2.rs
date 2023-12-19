use std::collections::HashSet;

use itertools::Itertools;

fn rotate(data: Vec<String>) -> Vec<String>{
    let mut result = vec![];
    for i in 0..data[0].chars().count(){
        let mut line = String::new();
        for j in 0..data.len(){
            line.push(data[j].chars().nth(i).unwrap());
        }
        result.push(line);
    }
    result
}

fn gravity(data: Vec<String>) -> Vec<String>{
    data.clone()
    .iter()
    .map(|line| line.split("#")
        .map(|x| x.chars()
            .sorted_by(|a, b| b.cmp(a)).collect::<String>()
        ).join("#").chars().rev().
        collect::<String>()
        .to_string()
    ).collect::<Vec<String>>()
}

fn count(data: Vec<String>) -> u64{
    let mut result: u64 = 0;
    for i in 0..data.len(){
        result += ((data.len() - i) * data[i].matches("O").count()) as u64;
    }
    result
}

fn cycle(data: Vec<String>) -> Vec<String>{
    let mut result = data.clone();
    for _ in 0..4{
        result = gravity(rotate(result));
    }
    result
}


pub fn process(input: &str) -> u64 {
    let mut data = input.lines()
        .map(|line| line.trim().to_string())
        .collect::<Vec<String>>();
    let mut viewed = HashSet::new();
    let mut visited = Vec::new();
    loop {
        viewed.insert(data.clone());
        visited.push(data.clone());
        data = cycle(data);
        if viewed.contains(&data){
            break;
        }
    }
    let start = visited.iter().position(|x| x == &data).unwrap();
    let end = visited.len();
    let period = end - start;
    let index = (1000000000 - start) % period + start;
    let result = visited[index].clone();
    count(result)
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_rotate(){
        let result = rotate(vec![
            "O..".to_string(),
            "O.O".to_string(),
            "...".to_string(),
        ]);

        assert_eq!(result, vec![
            "OO.".to_string(),
            "...".to_string(),
            ".O.".to_string(),
        ])
    }

    #[test]
    fn test_cycle(){
        let result = cycle(vec![
            ".....#....".to_string(),
            "....#...O#".to_string(),
            "...OO##...".to_string(),
            ".OO#......".to_string(),
            ".....OOO#.".to_string(),
            ".O#...O#.#".to_string(),
            "....O#....".to_string(),
            "......OOOO".to_string(),
            "#...O###..".to_string(),
            "#..OO#....".to_string(),
        ]);

        assert_eq!(result, vec![
            ".....#....".to_string(),
            "....#...O#".to_string(),
            ".....##...".to_string(),
            "..O#......".to_string(),
            ".....OOO#.".to_string(),
            ".O#...O#.#".to_string(),
            "....O#...O".to_string(),
            ".......OOO".to_string(),
            "#..OO###..".to_string(),
            "#.OOO#...O".to_string(),
        ]);
    }

    #[test]
    fn test_part2(){
        let result = process("O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....");
        assert_eq!(result, 64);
    }
}


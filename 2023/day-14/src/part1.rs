fn transpose(input: &str) -> Vec<Vec<char>> {
    let mut result: Vec<Vec<char>> = Vec::new();
    for x in 0..input.lines().nth(0).unwrap().len() {
        let mut row: Vec<char> = Vec::new();
        for y in 0..input.lines().count() {
            row.push(input.lines().nth(y).unwrap().chars().nth(x).unwrap());
        }
        result.push(row);
    }
    result
}

fn treat_line(input: &Vec<char>) -> u64{
    if input.len() == 0 {
        return 0;
    }

    let mut count = 0;
    let mut i = 0;
    while i < input.len() && input[i] != '#' {
        if input[i] == 'O' {
            count += 1;
        }
        i += 1;
    }
    let mut result = 0;
    if i < input.len()-1 {
        let slice = &input[i+1..].to_vec();
        result += treat_line(slice);
    }
    count = input.len() as u64 - count;
    result += (input.len()*(input.len()+1)/2) as u64 - (count*(count+1)/2) as u64;
    result
}

pub fn process(input: &str) -> u64 {
    let data = transpose(input);
    data.iter().map(|x| treat_line(x)).sum()
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_transpose(){
        let result = transpose("O.");
        assert_eq!(result, vec![vec!['O'], vec!['.']]);
        let result = transpose("O.
O.");
        assert_eq!(result, vec![vec!['O','O'],vec!['.','.']]);

    }

    #[test]
    fn test_part1(){
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
        assert_eq!(result, 136);
    }
}


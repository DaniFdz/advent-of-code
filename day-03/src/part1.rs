fn get_number(row: &str, index: usize) -> (u32, usize) {
    let mut number = String::new();
    let mut i = index;
    while i < row.len() && row.chars().nth(i).unwrap().is_digit(10) {
        number.push(row.chars().nth(i).unwrap());
        i += 1;
    }
    (number.parse::<u32>().unwrap(), i-index)
}

fn get_neighbours(input: Vec<&str>, x:usize, y:usize, lenght:usize) -> bool {
    for i in -1..(2 as i16) {
        for j in -1..(lenght as i16) + 1 {
            if i == 0 && j == 0 {
                continue;
            }
            let new_y = y as i16 + i;
            let new_x = x as i16 + j;
            if new_y < 0 || new_x < 0 {
                continue;
            }
            if new_y >= (input.len() as i16).try_into().unwrap() || new_x >= (input[new_y as usize].len() as i16).try_into().unwrap() {
                continue;
            }
            let ch = input[new_y as usize].chars().nth(new_x as usize).unwrap();
            if !ch.is_digit(10) && ch != '.' {
                return true;
            }
        }
    }
    false
}

pub fn process(input: &str) -> u32 {
    let mut sum: u32 = 0;
    let lines = input.lines().collect::<Vec<&str>>();
    for y in 0..lines.len() {
        let mut x: usize = 0;
        while x < lines[y].len() {
            if lines[y].chars().nth(x).unwrap().is_digit(10){
                let (number, length) = get_number(&lines[y], x);
                if get_neighbours(lines.clone(), x, y, length) {
                    sum += number;
                }
                x += length;
            }
            x += 1;
        }
    }
    sum
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_part1(){
        let result = process("467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..");
        assert_eq!(result, 4361);
    }
}


fn eval_reflection(reflection: &Vec<String>) -> u16 {
    for i in 1..reflection.len(){
        let first_slice: Vec<String> = reflection[..i].iter()
            .map(|x| x.to_string()).rev().collect();
        let second_slice: Vec<String> = reflection[i..].iter()
            .map(|x| x.to_string()).collect();

        if first_slice.iter().zip(second_slice.iter()).map(|(x,y)| {
            x.chars().zip(y.chars()).map(|(a,b)| match a == b {
                true => 0,
                false => 1
            }).sum::<u16>()
        }).sum::<u16>() == 1 {
            return i as u16;
        }
    }
    0
}

fn transpose(block: Vec<String>) -> Vec<String>{
    let mut transposed_block: Vec<String> = Vec::new();

    for x in 0..block.iter().nth(0).unwrap().len(){
        let mut new_str = "".to_string();
        for y in 0..block.len(){
            new_str = format!("{}{}", new_str, block.iter()
                .nth(y)
                .unwrap()
                .chars()
                .nth(x)
                .expect("No char at index")
                .to_string());
        }
        transposed_block.push(new_str)
    }
    
    transposed_block
}

pub fn process(input: &str) -> u16 {
    let mut total = 0;
    for b in input.split("\n\n") {
        let block: Vec<String> = b.split("\n").map(|x| x.to_string()).collect();
        total += eval_reflection(&block) * 100;
        let transpose_block = transpose(block);
        total += eval_reflection(&transpose_block);
    }
    total
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_part2(){
        let result = process("#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#");
        assert_eq!(result, 400);
    }
}


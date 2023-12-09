fn get_next_sucesion(sucesion: &Vec<i32>) -> Vec<i32> {
    let mut next_sucession = Vec::new();
    let mut last_number = 0;
    for (i,number) in sucesion.iter().enumerate() {
        if i == 0 {
            last_number = *number;
            continue;
        }
        next_sucession.push(number - last_number);
        last_number = *number;
    }
    next_sucession
}

fn get_prev_number(sucesion: &Vec<i32>) -> i32{
    let mut sucesions: Vec<Vec<i32>> = Vec::new();
    let mut next_sucesion = sucesion.clone();
    while next_sucesion.iter().any(|x| *x != 0){
        sucesions.push(next_sucesion.clone());
        next_sucesion = get_next_sucesion(&next_sucesion);
    }
    let mut last_number = 0;
    for sucession in sucesions.iter().rev() {
        last_number = sucession.first().unwrap().clone() - last_number;
    }
    last_number
}

pub fn process(input: &str) -> i32 {
    let sucesions = input.lines()
        .map(|line| line.split_whitespace()
            .map(|number| number.parse::<i32>().unwrap())
            .collect::<Vec<i32>>())
        .collect::<Vec<Vec<i32>>>();
    sucesions.iter()
        .map(|sucesion| get_prev_number(sucesion) as i32)
        .sum()
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn get_prev_number_test(){
        assert_eq!(get_prev_number(&vec![0,3,6,9,12,15]), -3);
        assert_eq!(get_prev_number(&vec![1,3,6,10,15,21]), 0);
        assert_eq!(get_prev_number(&vec![10,13,16,21,30,45]), 5);
    }

    #[test]
    fn test_get_next_sucesion(){
        assert_eq!(get_next_sucesion(&vec![0,3,6,9,12,15]), vec![3,3,3,3,3]);
        assert_eq!(get_next_sucesion(&vec![3,3,3,3,3]), vec![0,0,0,0]);
        assert_eq!(get_next_sucesion(&vec![1,3,6,10,15,21]), vec![2,3,4,5,6]);
        assert_eq!(get_next_sucesion(&vec![2,3,4,5,6],), vec![1,1,1,1]);
    }

    #[test]
    fn test_part2(){
        let result = process("0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45");
        assert_eq!(result, 2);
    }
}


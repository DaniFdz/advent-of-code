use std::collections::HashMap;

fn get_winning_numbers(winning_card: Vec<u16>, actual_card: Vec<u16>) -> u16{
    let mut count = 0;
    let mut wining_numbers: HashMap<u16, u16> = HashMap::new(); 
    winning_card.into_iter().for_each(|x| {
        wining_numbers.insert(x, 0);
    });
    actual_card.into_iter().for_each(|x| {
        if wining_numbers.contains_key(&x) {
            count += 1;
        }
    }); 
    count
}

pub fn process(input: &str) -> u32 {
    let mut sum: u32 = 0;
    let mut copies: HashMap<u16, u32> = HashMap::new();
    input.lines().enumerate().for_each(|(i,_)| {
        copies.insert(i as u16, 1);
    });
    
    for (i,line) in input.lines().enumerate() {
        let number_of_copies = *copies.get(&(i as u16)).unwrap();
        if number_of_copies == 0 {
            break;
        }
        let cards = line.split(": ").collect::<Vec<&str>>()[1].split(" | ").collect::<Vec<&str>>();
        let winning_card = cards[0].split(" ").filter_map(|x| x.parse::<u16>().ok()).collect::<Vec<u16>>();
        let actual_card = cards[1].split(" ").filter_map(|x| x.parse::<u16>().ok()).collect::<Vec<u16>>();
        let winning_numbers = get_winning_numbers(winning_card, actual_card);
        for j in i+1..i+1+winning_numbers as usize {
            if j == input.lines().count() {
                break;
            }
            *copies.get_mut(&(j as u16)).unwrap() += number_of_copies;
        }
    }
    for i in 0..input.lines().count() {
        sum += *copies.get(&(i as u16)).unwrap() as u32;
    }
    sum
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_part2(){
        let result = process("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11");
        assert_eq!(result, 30);
    }
}


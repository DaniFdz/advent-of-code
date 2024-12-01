use std::collections::HashMap;

#[allow(dead_code)]
#[derive(Debug)]
struct Hand<'a> {
    cards: &'a str,
    bid: u16,
    value: (u8, u128),
}

fn get_hand_value(hand: &str) -> (u8,u128) {
    let mut values = HashMap::new();
    let card_values = HashMap::from([
        ('T', 8),
        ('J', 9),
        ('Q', 10),
        ('K', 11),
        ('A', 12),
    ]);
    let mut val = 0;
    for (i,card) in hand.chars().enumerate(){
        if values.contains_key(&card){
            *values.get_mut(&card).unwrap() += 1;
        } else {
            values.insert(card, 1);
        }
        if card.is_digit(10) {
            if card == '1'{
                panic!("1 is not a valid card");
            }
            val += (card.to_digit(10).unwrap() as u128 - 2) * (13 as u128).pow(4 - i as u32);
        }
        else if card_values.contains_key(&card){
            val += *card_values.get(&card).unwrap() as u128 * (13 as u128).pow(4 - i as u32);
        } else {
            panic!("Invalid card");
        }
    }

    let mut first_max = 0;
    let mut second_max = 0;
    for (_, count) in values.iter() {
        if *count > first_max {
            second_max = first_max;
            first_max = *count;
        } else if *count > second_max {
            second_max = *count;
        }
    }

    match (first_max, second_max) {
        (5, _) => (7 as u8, val),
        (4, _) => (6 as u8, val),
        (3, 2) => (5 as u8, val),
        (3, _) => (4 as u8, val),
        (2, 2) => (3 as u8, val),
        (2, _) => (2 as u8, val),
        (1, _) => (1 as u8, val),
        _ => (0 as u8, val)
    }
}

pub fn process(input: &str) -> u32 {
    let mut results = input.lines()
        .map(|line| {
            let mut split = line.split(" ");
            let cards = split.next().unwrap();
            let bid = split.next().unwrap().parse::<u16>().unwrap();
            Hand{
                cards: cards as &str,
                bid,
                value: get_hand_value(cards),
            }
        }).collect::<Vec<Hand>>();

    results.sort_by(|a,b| {
        if a.value.0 == b.value.0 {
            a.value.1.cmp(&b.value.1)
        } else {
            a.value.0.cmp(&b.value.0)
        }
    });

    results.iter().enumerate().fold(0, |acc, (i, hand)| {
        acc + hand.bid as u32 * (i as u32 + 1)
    }) 

}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_part1(){
        let result = process("32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483");
        assert_eq!(result, 6440);
    }
}


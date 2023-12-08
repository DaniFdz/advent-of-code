use std::{cmp::Ordering, ops::Deref};
use itertools::Itertools;

#[allow(dead_code)]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Hand<'a> {
    cards: &'a str,
    bid: u16,
    value: u8,
}

fn get_hand_value(hand: &str) -> u8 {
    let counts = hand.chars().counts();
    let count_values = counts.values().sorted().join("");
    let j_s = counts.get(&'J');
    match (count_values.deref(), j_s) {
        ("5", _) => 7,
        ("14", Some(1)) => 7,
        ("14", Some(4)) => 7,
        ("14", _) => 6,
        ("23", None) => 5,
        ("23", _) => 7,
        ("113", Some(3)) => 6,
        ("113", Some(1)) => 6,
        ("113", _) => 4,
        ("122", Some(2)) => 6,
        ("122", Some(1)) => 5,
        ("122", _) => 3,
        ("1112", Some(2)) => 4,
        ("1112", Some(1)) => 4,
        ("1112", _) => 2,
        ("11111", Some(1)) => 2,
        ("11111", _) => 1,
        _ => 0
    }
}

fn get_card_value(card: char) -> u8 {
    "J23456789TQKA".find(card).unwrap() as u8 + 1
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



    results.sort_by(|a, b| {
        if a.value != b.value {
            return a.value.cmp(&b.value);
        }

        for i in 0..a.cards.chars().count() {
            let a_val = get_card_value(a.cards.chars().nth(i).unwrap());
            let b_val = get_card_value(b.cards.chars().nth(i).unwrap());

            if a_val != b_val {
                return a_val.cmp(&b_val);
            }
        }

        Ordering::Equal
    });

    results.iter().enumerate()
        .map(|(i, hand)| (i as u32 + 1) * hand.bid as u32)
        .sum()

}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_card_value(){
        assert_eq!(get_card_value('J'), 1);
        assert_eq!(get_card_value('T'), 10);
        assert_eq!(get_card_value('A'), 13);
    }

    #[test]
    fn test_part2(){
        let result = process("32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483");
        assert_eq!(result, 5905);
    }

    fn kind_test_helper(hand: &str, expected: u8){
        assert_eq!(get_hand_value(hand), expected);
    }

    #[test]
    fn test_get_hand_value(){
        /*
         * FiveOfAKind: 7
         * FourOfAKind: 6
         * FullHouse: 5
         * ThreeOfAKind: 4
         * TwoPair: 3
         * OnePair: 2
         * HighCard: 1
         * */
        kind_test_helper("AAAAA", 7);
        kind_test_helper("JJJJJ", 7);
        kind_test_helper("JJJJA", 7);
        kind_test_helper("JJJAA", 7);
        kind_test_helper("JJAAA", 7);
        kind_test_helper("JAAAA", 7);
        kind_test_helper("AAAAQ", 6);
        kind_test_helper("JJJAQ", 6);
        kind_test_helper("JJAAQ", 6);
        kind_test_helper("JAAAQ", 6);
        kind_test_helper("AAAQQ", 5);
     // kind_test_helper("JJAQQ", 5);
        kind_test_helper("JAAQQ", 5);
        kind_test_helper("AAAQK", 4);
        kind_test_helper("JJAQK", 4);
        kind_test_helper("JAAQK", 4);
        kind_test_helper("AAQQK", 3);
        kind_test_helper("AAQKT", 2);
        kind_test_helper("JAQKT", 2);
        kind_test_helper("23456", 1);
    }
}


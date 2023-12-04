pub fn process(input: &str) -> u16 {
    let mut sum: u16 = 0;
    let numbers = vec!["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    let inverse_numbers = vec!["eno", "owt", "eerht", "ruof", "evif", "xis", "neves", "thgie", "enin"];

    for line in input.lines() {
        let mut first:u8 = 0;
        let mut i = 0;
        let mut possible_values: Vec<String> = Vec::new();
        for c in line.chars(){
            if first != 0 {
                break;
            }
            i += 1;
            if c.is_digit(10){
                first = c.to_digit(10).unwrap() as u8;
                break
            }
            possible_values.push(c.to_string());
            let mut new_possible_values: Vec<String> = vec![c.to_string()];
            for possible_value in possible_values {
                let new_possible_value = format!("{}{}", possible_value, c);
                for k in 0..numbers.len() {
                    let number = numbers[k];
                    if new_possible_value == number {
                        first = k as u8 + 1;
                        break;
                    }
                    if number.starts_with(&new_possible_value) {
                        new_possible_values.push(new_possible_value);
                        break;
                    }                    
                }
            }
            possible_values = new_possible_values;
        }
        if i == line.len() {
            sum += first as u16 * 11;
            continue;
        }
        let mut j = line.len() - 1;
        let mut last = 0;
        possible_values = Vec::new();
        while j >= i {
            if last != 0 {
                break;
            }
            let ch = line.chars().nth(j).unwrap();
            j -= 1;
            if ch.is_digit(10){
                last = ch.to_digit(10).unwrap() as u8;
                break;
            }
            let mut new_possible_values: Vec<String> = vec![ch.to_string()];
            for possible_value in possible_values {
                let new_possible_value = format!("{}{}", possible_value, ch);
                for k in 0..inverse_numbers.len() {
                    let number = inverse_numbers[k];
                    if new_possible_value == number {
                        last = k as u8 + 1;
                        break;
                    }
                    if number.starts_with(&new_possible_value) {
                        new_possible_values.push(new_possible_value);
                        break;
                    }                    
                }
            }
            possible_values = new_possible_values;
        }

        if last == 0 {
            last = first;
        }

        sum += first as u16 * 10 + last as u16;
    }
    sum
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_part2(){
        let result = process("two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen");
        assert_eq!(result, 281);
    }
}


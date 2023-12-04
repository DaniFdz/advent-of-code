pub fn process(input: &str) -> u16 {
    let mut sum: u16 = 0;

    for line in input.lines() {
        let mut first:u8 = 0;
        let mut i = 0;
        for c in line.chars(){
            if c.is_digit(10){
                first = c.to_digit(10).unwrap() as u8;
                i += 1;
                break;
            }
            i += 1;
        }
        if i == line.len() {
            sum += first as u16 * 11;
            continue;
        }
        let mut j = line.len() - 1;
        let mut last = first;
        while j >= i {
            if line.chars().nth(j).unwrap().is_digit(10){
                last = line.chars().nth(j).unwrap().to_digit(10).unwrap() as u8;
                break;
            }
            j -= 1;
        }

        sum += first as u16 * 10 + last as u16;
    }
    sum
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_part1(){
        let result = process("1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet");
        assert_eq!(result, 142);
    }
}


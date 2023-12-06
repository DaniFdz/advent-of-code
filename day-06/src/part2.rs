pub fn process(input: &str) -> u64 {
    let times: u64 = input.lines()
        .nth(0).unwrap()
        .split(":")
        .nth(1).unwrap()
        .replace(" ", "")
        .parse::<u64>().unwrap();

    let distances: u64 = input.lines()
        .nth(1).unwrap()
        .split(":")
        .nth(1).unwrap()
        .replace(" ", "")
        .parse::<u64>().unwrap();

    let mut okey_times: u64 = 0;

    for possible_time in 0..times{
        if possible_time*(times - possible_time) > distances{
            okey_times += 1;
        }

    }
    okey_times
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_part2(){
        let result = process("Time:      7  15   30
Distance:  9  40  200");
        assert_eq!(result, 71503);
    }
}


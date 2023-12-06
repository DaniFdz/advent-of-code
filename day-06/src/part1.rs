pub fn process(input: &str) -> u32 {
    let mut sum = 1;
    let times = input.lines()
        .nth(0).unwrap()
        .split(":")
        .nth(1).unwrap()
        .trim()
        .split_whitespace()
        .filter_map(|x| x.parse::<u16>().ok())
        .collect::<Vec<u16>>();

    let distances = input.lines()
        .nth(1).unwrap()
        .split(":")
        .nth(1).unwrap()
        .trim()
        .split_whitespace()
        .filter_map(|x| x.parse::<u16>().ok())
        .collect::<Vec<u16>>();

    for i in 0..times.len(){
        let mut best_times = Vec::new();

        for possible_time in 0..times[i]{
            best_times.push(possible_time*(times[i] - possible_time));
        }
        sum *= best_times.iter().filter(|x| x > &&distances[i]).count() as u32;
    }
    sum
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_part1(){
        let result = process("Time:      7  15   30
Distance:  9  40  200");
        assert_eq!(result, 288);
    }
}


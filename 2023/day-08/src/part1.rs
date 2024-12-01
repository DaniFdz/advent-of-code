use std::collections::HashMap;

#[allow(dead_code)]
#[derive(Debug)]
struct Step<'a> {
    left: &'a str,
    right: &'a str,
}

pub fn process(input: &str) -> usize {
    let instructions = input.lines().next().unwrap().chars();
    let mut network = HashMap::<&str, Step>::new();

    let mut current = "AAA";

    input.lines().skip(2)
        .for_each(|line| {
            let parts = line.split(" = ").collect::<Vec<&str>>();
            let direction = parts[0];
            let mut parts = parts[1][1..parts[1].len()-1].split(", ");
            let left = parts.next().unwrap();
            let right = parts.next().unwrap();
            network.insert(direction, Step{left, right});
        });

    let mut count = 0;
    while current != "ZZZ" {
        let step = network.get(current).unwrap();
        let pos = count % instructions.clone().count();
        if instructions.clone().nth(pos).unwrap() == 'R' {
            current = step.right;
        } else {
            current = step.left;
        }
        count += 1;
    }

    count
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_part1(){
        let result = process("RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)");
        assert_eq!(result, 2);

        let result = process("LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)");
        assert_eq!(result, 6);
    }
}



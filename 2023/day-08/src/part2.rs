use std::collections::HashMap;

#[allow(dead_code)]
#[derive(Debug)]
struct Step<'a> {
    left: &'a str,
    right: &'a str,
}

fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

fn lcm(a: u64, b: u64) -> u64 {
    if a == 0 || b == 0 {
        return 0; 
    }
    (a * b) / gcd(a, b)
}


pub fn process(input: &str) -> u64 {
    let instructions = input.lines().next().unwrap().chars();
    let mut network = HashMap::<&str, Step>::new();
    let mut node_names = Vec::<&str>::new();

    input.lines().skip(2)
        .for_each(|line| {
            let parts = line.split(" = ").collect::<Vec<&str>>();
            let direction = parts[0];
            let mut parts = parts[1][1..parts[1].len()-1].split(", ");
            let left = parts.next().unwrap();
            let right = parts.next().unwrap();
            node_names.push(direction);
            network.insert(direction, Step{left, right});
        });


    let mut frequencies = Vec::<u32>::new();

    for node in node_names.iter()
        .filter(|node| node.ends_with("A")) {
        let mut current = node;
        let mut count: u32 = 0;

        loop {
            let step = network.get(current).unwrap();
            let pos = count as usize % instructions.clone().count();
            let direction = instructions.clone().nth(pos).unwrap();
            if direction == 'R' {
                current = &step.right;
            } else {
                current = &step.left;
            }
            count += 1;
            if current.chars().nth(2).unwrap() == 'Z' {
                frequencies.push(count);
                break;
            }
        }

    }

    frequencies
        .iter()
        .fold(1, |acc, x| lcm(acc, *x as u64))
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_part2(){
        let result = process("LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)");
        assert_eq!(result, 6);
    }
}


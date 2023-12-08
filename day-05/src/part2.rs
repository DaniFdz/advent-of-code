#[allow(dead_code)] // Rust doesn't know that this is used to debug
#[derive(Debug)]
struct Range{
    destination: u32,
    source: u32,
    length: u32,
}

#[allow(dead_code)] // Rust doesn't know that this is used to debug
#[derive(Debug)]
struct Pair{
    start: u32,
    end: u32,
}

fn get_new_pairs(pairs: &Vec<Vec<Pair>>, ranges: &Vec<Vec<Vec<Range>>>) -> Vec<Vec<Pair>>{
    if ranges.len() == 0{
        pairs.clone()
    }


}

pub fn process(input: &str) -> u32 {
    let pairs = input.lines()
        .next()
        .unwrap()
        .split(": ")
        .skip(1)
        .map(|x| x.split_whitespace()
            .collect::<Vec<&str>>()
            .chunks(2)
            .map(|x| Pair{
                start: x[0].parse::<u32>().unwrap(),
                end: x[1].parse::<u32>().unwrap(),
            }).collect::<Vec<Pair>>()
        ).collect::<Vec<Vec<Pair>>>();
    dbg!(&pairs);

    let ranges = input.split("\n\n")
        .skip(1)
        .map(|x| x.split(":")
            .skip(1)
            .map(|y| y.split("\n").skip(1)
                .map(|z| {
                    let range = z.split_whitespace()
                        .collect::<Vec<&str>>();
                    Range{
                        destination: range[0].parse::<u32>().unwrap(),
                        source: range[1].parse::<u32>().unwrap(),
                        length: range[2].parse::<u32>().unwrap(),
                    }
                }).collect::<Vec<Range>>()
            ).collect::<Vec<Vec<Range>>>()
        ).collect::<Vec<Vec<Vec<Range>>>>();
    dbg!(&ranges);

    1
            
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_part2(){
        let result = process("seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4");
        assert_eq!(result, 46);
    }
}


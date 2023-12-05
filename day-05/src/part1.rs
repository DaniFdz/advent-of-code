use std::collections::HashMap;

#[derive(PartialEq, Eq, Hash, Debug)]
struct Categories{
    source: String,
    destination: String,
}

#[derive(Debug)]
struct Range{
    destination: u32,
    source: u32,
    length: u32,
}

pub fn process(input: &str) -> u32 {
    let mut maps: HashMap<Categories, Vec<Range>> = HashMap::new();

    let seeds = input.lines()
                    .collect::<Vec<&str>>()[0]
                    .split(": ").collect::<Vec<&str>>()[1]
                    .split(" ").map(|x| 
                        x.parse::<u32>().expect("Should be a number")
                    ).collect::<Vec<u32>>();

    input.split("\n\n")
        .collect::<Vec<&str>>()[1..]
        .iter()
        .for_each(|x| {
            let map = x.split(" map:\n").collect::<Vec<&str>>();
            let categories = map[0].split("-to-").collect::<Vec<&str>>();
            let cat = Categories{
                source: categories[0].to_string(),
                destination: categories[1].to_string(),
            };
            let ranges = map[1].split("\n")
                    .collect::<Vec<&str>>()
                    .iter()
                    .filter(|x| !x.is_empty())
                    .map(|x| {
                        let range = x.split(" ")
                            .collect::<Vec<&str>>()
                            .iter()
                            .map(|y| {
                                y.parse::<u32>().expect("Should be a number:")
                            }).collect::<Vec<u32>>();
                        Range{
                            destination: range[0],
                            source: range[1],
                            length: range[2],
                        }
                    }).collect::<Vec<Range>>();

            maps.insert(cat, ranges);

        });
    dbg!(&maps);
    1
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_part1(){
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
        assert_eq!(result, 0);
    }
}


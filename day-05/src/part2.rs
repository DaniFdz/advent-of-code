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
    length: u32,
}

pub fn process(input: &str) -> u32 {
    let mut maps: Vec<Vec<Range>> = Vec::new();

    input.split("\n\n")
        .collect::<Vec<&str>>()[1..]
        .iter()
        .for_each(|x| {
            let mut map: Vec<Range> = Vec::new();
            x.split(" map:\n").collect::<Vec<&str>>()[1].split("\n")
            .filter(|y| !y.is_empty())
            .for_each(|row| {
                let range = row.split(" ")
                    .collect::<Vec<&str>>()
                    .iter()
                    .map(|n| {
                        n.parse::<u32>().expect("Should be a number:")
                    }).collect::<Vec<u32>>();
                map.push(Range{
                    destination: range[0],
                    source: range[1],
                    length: range[2],
                });
            });
            maps.push(map);
        });


    let pairs: Vec<Pair> = input.lines()
        .next().unwrap()
        .split(": ").nth(1).unwrap()
        .split_whitespace().collect::<Vec<&str>>().chunks(2).into_iter()
        .map(|chunk| {
            if chunk.len() == 2 {
                Some(Pair {
                    start: chunk[0].parse::<u32>().unwrap(),
                    length: chunk[1].parse::<u32>().unwrap(),
                })
            } else {
                None
            }
        }).filter(|x| x.is_some())
        .map(|x| x.unwrap())
        .collect::<Vec<Pair>>();

    let mut min = u32::MAX;
    for pair in pairs{
        for n in pair.start..pair.start+pair.length{
            let mut x = n;
            for map in &maps{
                for possible_value in map{
                    if x >= possible_value.source && x <= possible_value.source + possible_value.length{
                        x = possible_value.destination + (x - possible_value.source);
                        break;
                    }
                }
            }
            
            if x < min{
                min = x;
            }
        }
    }
    min
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


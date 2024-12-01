pub fn process(input: &str) -> u32 {
    let mut sum: u32 = 0;
    for line in input.lines() {
        let content = line.split(": ").map(|x| x.to_string()).collect::<Vec<_>>();
        let mut max_red: u8 = 0;
        let mut max_green: u8 = 0;
        let mut max_blue: u8 = 0;

        for game in content[1].split("; "){
            let mut red: u8 = 0;
            let mut green: u8 = 0;
            let mut blue: u8 = 0;
            let mut number: u8 = 0;
            for (j,word) in game.split(" ").enumerate() {
                if j % 2 == 0 {
                    number = word.parse::<u8>().unwrap();
                }
                if j % 2 == 1 {
                    match word.chars().nth(0).unwrap() {
                        'r' => red += number,
                        'g' => green += number,
                        'b' => blue += number,
                        _ => panic!("Invalid color"),
                    }
                }
            }

            if red > max_red {
                max_red = red;
            }
            if green > max_green {
                max_green = green;
            }
            if blue > max_blue {
                max_blue = blue;
            }
        }

        sum += max_red as u32 * max_green as u32 * max_blue as u32;
        
    }
    sum
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_part2(){
        let result = process("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green");
        assert_eq!(result, 2286);
    }
}


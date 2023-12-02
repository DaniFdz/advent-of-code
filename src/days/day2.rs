pub fn problem1(input: Vec<String>){
    let mut sum: u16 = 0;
    for (i,line) in input.iter().enumerate() {
        let content = line.split(": ").map(|x| x.to_string()).collect::<Vec<_>>();
        let mut possible_game = true;

        for game in content[1].split("; "){
            let mut red = 0;
            let mut green = 0;
            let mut blue = 0;
            let mut number: u16 = 0;
            for (j,word) in game.split(" ").enumerate() {
                if j % 2 == 0 {
                    number = word.parse::<u16>().unwrap();
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

            if red > 12 || green > 13 || blue > 14 {
                possible_game = false;
            }
        }

        if possible_game {
            sum += i as u16 + 1;
        }
        
    }
    println!("Day 2, Problem 1: {}", sum);
}


pub fn problem2(input: Vec<String>){
    let mut sum: u32 = 0;
    for line in input.iter() {
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
    println!("Day 2, Problem 1: {}", sum);
}

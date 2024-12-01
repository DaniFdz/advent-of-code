use day_12::part1::process;

fn main(){
    let input = include_str!("../../inputs/input1.txt");
    let output = process(input);
    let _ = !dbg!(output);
} 


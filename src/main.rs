use std::{io, env};
mod days;

fn check_args() -> io::Result<()> {
    let args = env::args().collect::<Vec<String>>();
    if args.len() != 2 {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "Invalid number of arguments"));
    }
    Ok(())
}

fn read_file() -> io::Result<Vec<String>>{
    let filename = env::args().collect::<Vec<String>>()[1].clone();
    let content = std::fs::read_to_string(filename)?;
    let lines = content.split("\n").filter(|x| x != &"").map(|x| x.to_string()).collect::<Vec<String>>();
    Ok(lines)
}

fn main() -> io::Result<()> {
    check_args()?;
    let content = read_file()?;
    days::day3::problem1(content.clone());
    // days::day2::problem2(content);
    Ok(())
}

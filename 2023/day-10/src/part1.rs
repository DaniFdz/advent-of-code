use std::fmt;

#[allow(dead_code)]
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum MazeObject {
    VerticalPipe,
    HorizontalPipe,
    NECorner,
    NWCorner,
    SWCorner,
    SECorner,
    Ground,
    Start
}

impl fmt::Display for MazeObject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MazeObject::VerticalPipe => write!(f, "|"),
            MazeObject::HorizontalPipe => write!(f, "-"),
            MazeObject::NECorner => write!(f, "L"),
            MazeObject::NWCorner => write!(f, "J"),
            MazeObject::SWCorner => write!(f, "7"),
            MazeObject::SECorner => write!(f, "F"),
            MazeObject::Ground => write!(f, "."),
            MazeObject::Start => write!(f, "S"),
        }
    }
}

#[allow(dead_code)]
#[derive(Debug)]
struct Maze{
    width: usize,
    height: usize,
    startting_point: (usize, usize),
    maze: Vec<Vec<MazeObject>>
}

impl Maze{
    fn get_startting_point(&mut self){
        for (i, row) in self.maze.iter().enumerate(){
            for (j, col) in row.iter().enumerate(){
                if let MazeObject::Start = col{
                    self.startting_point = (i, j);
                    return;
                }
            }
        }
        panic!("No starting point found");
    }

    fn get_positions(&self, position: (usize, usize), previous: (i16, i16)) -> (i16, i16) {
        let pos = self.maze[position.0][position.1];
        match pos {
            MazeObject::VerticalPipe => {
                if previous.0 == 1 { (1, 0) } else { (-1, 0) }
            }
            MazeObject::HorizontalPipe => {
                if previous.1 == 1 { (0, 1) } else { (0, -1) }
            }
            MazeObject::NECorner => {
                if previous.0 == 0 { (-1, 0) } else { (0, 1) }
            }
            MazeObject::NWCorner => {
                if previous.0 == 0 { (-1, 0) } else { (0, -1) }
            }
            MazeObject::SWCorner => {
                if previous.0 == 0 { (1,0) } else { (0, -1) }
            }
            MazeObject::SECorner => {
                if previous.0 == 0 { (1,0) } else { (0, 1) }
            }
            _ => (0, 0)
        }
    }

    fn get_startting_directions(&self) -> Vec<(i16, i16)> {
        let mut directions = Vec::new();
        for possible_direction in vec![(0, 1), (1, 0), (0, -1), (-1, 0)] {
            let (x,y) = (self.startting_point.0 as i16 + possible_direction.0 as i16, self.startting_point.1 as i16 + possible_direction.1 as i16);
            if x < 0 || y < 0 || x >= self.width as i16 || y >= self.height as i16 {
                continue;
            }
            let possible_position = (x as usize, y as usize);
            let first_it = self.get_positions(possible_position, possible_direction);
            if self.get_positions(possible_position, (-first_it.0, -first_it.1)) == (-possible_direction.0, -possible_direction.1) {
                directions.push(possible_direction);
            }
        }
        directions
    }

    fn get_steps(&self) -> u32 {
        let mut steps = 0;
        let mut positions: Vec<(usize, usize)> = Vec::new();
        let mut directions: Vec<(i16, i16)> = Vec::new();

        for next_vec in self.get_startting_directions() {
            let next_position = (self.startting_point.0 + next_vec.0 as usize, self.startting_point.1 + next_vec.1 as usize);
            positions.push(next_position);
            directions.push(next_vec);
        }

        loop{
            steps += 1;
            let position_2 = positions.pop().unwrap();
            let direction_2 = directions.pop().unwrap();
            let position_1 = positions.pop().unwrap();
            let direction_1 = directions.pop().unwrap();

            let next_direction_1 = self.get_positions(position_1, direction_1);
            let next_direction_2 = self.get_positions(position_2, direction_2);

            let new_position_1 = ((position_1.0 as i16 + next_direction_1.0) as usize, (position_1.1 as i16 + next_direction_1.1) as usize);
            let new_position_2 = ((position_2.0 as i16 + next_direction_2.0) as usize, (position_2.1 as i16 + next_direction_2.1) as usize);

            if new_position_1 == new_position_2 {
                return steps + 1;
            }

            positions.push(new_position_1);
            directions.push(next_direction_1);
            positions.push(new_position_2);
            directions.push(next_direction_2);
        }
    }
}


pub fn process(input: &str) -> u32 {
    let mut maze = Maze{
        width: input.lines().count(),
        height: input.lines().nth(0).unwrap().len(),
        maze: input.lines().map(|line| {
            line.chars().map(|c| {
                match c {
                    '|' => MazeObject::VerticalPipe,
                    '-' => MazeObject::HorizontalPipe,
                    'L' => MazeObject::NECorner,
                    'J' => MazeObject::NWCorner,
                    '7' => MazeObject::SWCorner,
                    'F' => MazeObject::SECorner,
                    '.' => MazeObject::Ground,
                    'S' => MazeObject::Start,
                    c => panic!("Unknown character {}", c)
                }
            }).collect()
        }).collect(),
        startting_point: (0, 0)
    };

    maze.get_startting_point();
    maze.get_steps()
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn get_startting_point(){
        let mut maze = Maze{
            width: 5,
            height: 5,
            maze: vec![
                vec![MazeObject::Ground, MazeObject::Ground, MazeObject::Ground, MazeObject::Ground, MazeObject::Ground],
                vec![MazeObject::Ground, MazeObject::Ground, MazeObject::Ground, MazeObject::Ground, MazeObject::Ground],
                vec![MazeObject::Ground, MazeObject::Ground, MazeObject::Start, MazeObject::Ground, MazeObject::Ground],
                vec![MazeObject::Ground, MazeObject::Ground, MazeObject::Ground, MazeObject::Ground, MazeObject::Ground],
                vec![MazeObject::Ground, MazeObject::Ground, MazeObject::Ground, MazeObject::Ground, MazeObject::Ground],
            ],
            startting_point: (0, 0)
        };

        maze.get_startting_point();
        assert_eq!(maze.startting_point, (2, 2));
    }

    #[test]
    fn get_startting_directions(){
        let mut maze = Maze{
            width: 5,
            height: 5,
            maze: vec![
                vec![MazeObject::Ground, MazeObject::Ground, MazeObject::Ground, MazeObject::Ground, MazeObject::Ground],
                vec![MazeObject::Ground, MazeObject::Ground, MazeObject::Ground, MazeObject::Ground, MazeObject::Ground],
                vec![MazeObject::Start, MazeObject::HorizontalPipe, MazeObject::HorizontalPipe, MazeObject::Ground, MazeObject::Ground],
                vec![MazeObject::VerticalPipe, MazeObject::Ground, MazeObject::NECorner, MazeObject::Ground, MazeObject::Ground],
                vec![MazeObject::Ground, MazeObject::Ground, MazeObject::Ground, MazeObject::Ground, MazeObject::Ground],
            ],
            startting_point: (0, 0)
        };

        maze.get_startting_point();
        let directions = maze.get_startting_directions().sort();
        assert_eq!(directions, vec![(0, 1), (1, 0)].sort());


        let mut maze = Maze{
            width: 5,
            height: 5,
            maze: vec![
                vec![MazeObject::Ground, MazeObject::Ground, MazeObject::SECorner, MazeObject::SWCorner, MazeObject::Ground],
                vec![MazeObject::Ground, MazeObject::SECorner, MazeObject::NWCorner, MazeObject::VerticalPipe, MazeObject::Ground],
                vec![MazeObject::Start, MazeObject::NWCorner, MazeObject::Ground, MazeObject::NECorner, MazeObject::SWCorner],
                vec![MazeObject::VerticalPipe, MazeObject::SECorner, MazeObject::HorizontalPipe, MazeObject::HorizontalPipe, MazeObject::NWCorner],
                vec![MazeObject::NECorner, MazeObject::NWCorner, MazeObject::Ground, MazeObject::Ground, MazeObject::Ground],
            ],
            startting_point: (0, 0)
        };

        maze.get_startting_point();
        let directions = maze.get_startting_directions().sort();
        assert_eq!(directions, vec![(1,0), (0,1)].sort());
    }

    #[test]
    fn test_get_positions(){
        let mut maze = Maze{
            width: 5,
            height: 5,
            maze: vec![
                vec![MazeObject::SECorner, MazeObject::SWCorner, MazeObject::Ground, MazeObject::Ground, MazeObject::Ground],
                vec![MazeObject::VerticalPipe, MazeObject::NECorner, MazeObject::Ground, MazeObject::Ground, MazeObject::Ground],
                vec![MazeObject::NECorner, MazeObject::HorizontalPipe, MazeObject::Start, MazeObject::Ground, MazeObject::Ground],
                vec![MazeObject::Ground, MazeObject::Ground, MazeObject::Ground, MazeObject::Ground, MazeObject::Ground],
                vec![MazeObject::Ground, MazeObject::Ground, MazeObject::Ground, MazeObject::Ground, MazeObject::Ground],
            ],
            startting_point: (0, 0)
        };

        maze.get_startting_point();
        assert_eq!(maze.get_positions((2,1), (0, -1)), (0, -1));
        assert_eq!(maze.get_positions((2,0), (0, -1)), (-1, 0));
        assert_eq!(maze.get_positions((1,0), (-1, 0)), (-1, 0));
        assert_eq!(maze.get_positions((0,0), (-1, 0)), (0, 1));
        assert_eq!(maze.get_positions((0,1), (0, 1)), (1, 0));
        assert_eq!(maze.get_positions((1,1), (1,0)), (0, 1));
    }

    #[test]
    fn test_part1(){
        let result = process(".....
.S-7.
.|.|.
.L-J.
.....");
        assert_eq!(result, 4);

        let result = process("..F7.
.FJ|.
SJ.L7
|F--J
LJ...");
        assert_eq!(result, 8);
    }
}


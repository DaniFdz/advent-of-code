#[derive(Debug, Eq, PartialEq, Clone)]
enum SpringState{
    Operational,
    Damaged,
    Unknown,
}

impl std::fmt::Display for SpringState {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use SpringState::*;
        match self {
            Operational => write!(f, "."),
            Damaged => write!(f, "#"),
            Unknown => write!(f, "?"),
        }
    }
}

#[derive(Debug, Clone)]
struct Row{
    state:Vec<SpringState>,
    expected:Vec<u8>
}

fn check_validity(row: Row) -> bool {
    use SpringState::*;
    let mut index = 0;
    let mut consecutive = 0;
    for i in 0..row.state.len(){
        match row.state[i]{
            Operational => {
                if consecutive != 0{
                    if index >= row.expected.len(){
                        return false;
                    }
                    if consecutive != row.expected[index]{
                        return false;
                    }
                    consecutive = 0;
                    index += 1;
                }
                if i == row.state.len() - 1{
                    if index != row.expected.len(){
                        return false;
                    }
                }
            },
            Damaged => {
                consecutive += 1;
                if i == row.state.len() - 1{
                    if index + 1 != row.expected.len(){
                        return false;
                    }
                    if consecutive != row.expected[index]{
                        return false;
                    }
                }
            },
            Unknown => {
                if index < row.expected.len(){
                    return consecutive <= row.expected[index];
                }
                else {
                    return index == row.expected.len() && consecutive == 0;
                }
            }
        }
    }
    return true;
}

fn get_posibilities(row: Row) -> u32 {
    use SpringState::*;
    match check_validity(row.clone()) {
        true =>  {
            if let Some(index) = row.state.iter().position(|s| *s == Unknown){
                let mut row1 = row.clone();
                *row1.state.get_mut(index).unwrap() = Operational;
                let mut row2 = row.clone();
                *row2.state.get_mut(index).unwrap() = Damaged;
                return get_posibilities(row1) + get_posibilities(row2);
            }
            return 1;
        },
        false => { return 0; }
    };

}

pub fn process(input: &str) -> u32 {
    input.lines()
        .map(|line| {
            let splited_line = line.split_whitespace().collect::<Vec<&str>>();
            let state = splited_line
                .iter()
                .next()
                .unwrap()
                .chars()
                .map(|c| {
                    match c{
                        '.' => SpringState::Operational,
                        '#' => SpringState::Damaged,
                        '?' => SpringState::Unknown,
                        _ => panic!("Unknown char")
                    }
                }).collect::<Vec<SpringState>>();
 
            let expected = splited_line
                .iter()
                .nth(1)
                .map(|s| s.split(",")
                     .map(|s| s.parse::<u8>().expect("Not a number"))
                )
                .unwrap()
                .collect::<Vec<u8>>();
            get_posibilities(Row{state, expected})
        }).sum()
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_check_validity(){
        use SpringState::*;

        assert_eq!(check_validity(Row{
            state: vec![Damaged, Operational, Damaged],
            expected: vec![1,1]
        }), true);

        assert_eq!(check_validity(Row{
            state: vec![Operational, Damaged, Operational, Damaged, Operational],
            expected: vec![1,1]
        }), true);

        assert_eq!(check_validity(Row{
            state: vec![Operational, Damaged, Operational, Operational, Operational, Operational, Operational, Operational, Damaged, Damaged],
            expected: vec![1,2]
        }), true);

        assert_eq!(check_validity(Row{
            state: vec![Operational, Damaged, Operational, Damaged, Damaged],
            expected: vec![1,2]
        }), true);

        assert_eq!(check_validity(Row{
            state: vec![Operational, Damaged, Damaged],
            expected: vec![1]
        }), false);

        assert_eq!(check_validity(Row{
            state: vec![Operational, Damaged, Damaged, Operational],
            expected: vec![1]
        }), false);

        assert_eq!(check_validity(Row{
            state: vec![Damaged, Damaged, Operational, Damaged],
            expected: vec![1, 1]
        }), false);

        assert_eq!(check_validity(Row{
            state: vec![Damaged, Operational, Operational],
            expected: vec![1, 1]
        }), false);

        assert_eq!(check_validity(Row{
            state: vec![Damaged, Operational, Damaged],
            expected: vec![1, 1]
        }), true);

        assert_eq!(check_validity(Row{
            state: vec![Damaged, Unknown, Damaged],
            expected: vec![1, 1]
        }), true);

        assert_eq!(check_validity(Row{
            state: vec![Damaged, Damaged, Unknown],
            expected: vec![1, 1]
        }), false);

        assert_eq!(check_validity(Row{
            state: vec![Unknown, Damaged, Unknown],
            expected: vec![1, 5]
        }), true);
    }

    #[test]
    fn test_get_posibilities(){
        use SpringState::*;

        assert_eq!(get_posibilities(Row{
            state: vec![Unknown, Damaged, Operational, Damaged],
            expected: vec![1, 1]
        }), 1);

        assert_eq!(get_posibilities(Row{
            state: vec![Damaged, Unknown, Unknown],
            expected: vec![1, 2]
        }), 0);

        assert_eq!(get_posibilities(Row{
            state: vec![Unknown, Unknown, Unknown, Operational, Damaged, Damaged, Damaged],
            expected: vec![1, 1, 3]
        }), 1);

        assert_eq!(get_posibilities(Row{
            state: vec![
                Operational, Unknown, Unknown, Operational, Operational, Unknown, Unknown,
                Operational, Operational, Operational, Unknown, Damaged, Damaged, Operational
            ],
            expected: vec![1, 1, 3]
        }), 4);

        assert_eq!(get_posibilities(Row{
            state: vec![
                Unknown, Damaged, Unknown, Damaged, Unknown, Damaged, Unknown, Damaged,
                Unknown, Damaged, Unknown, Damaged, Unknown, Damaged, Unknown,
            ],
            expected: vec![1, 3, 1, 6]
        }), 1);

        assert_eq!(get_posibilities(Row{
            state: vec![
                Unknown, Unknown, Unknown, Unknown, Operational, Damaged, Operational,
                Operational, Operational, Damaged, Operational, Operational, Operational
            ],
            expected: vec![4,1,1]
        }), 1);

        assert_eq!(get_posibilities(Row{
            state: vec![
                Unknown, Unknown, Unknown, Unknown, Operational, Damaged, Damaged,
                Damaged, Damaged, Damaged, Damaged, Operational, Operational, Damaged,
                Damaged, Damaged, Damaged, Damaged, Operational,
            ],
            expected: vec![1,6,5]
        }), 4);

        assert_eq!(get_posibilities(Row{
            state: vec![
                Unknown, Damaged, Damaged, Damaged, Unknown, Unknown, Unknown, Unknown,
                Unknown, Unknown, Unknown, Unknown,
            ],
            expected: vec![3,2,1]
        }), 10);
    }

    #[test]
    fn test_part1(){
        let result = process("???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1");
        assert_eq!(result, 21);
    }
}


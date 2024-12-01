use std::collections::HashMap;

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
struct State {
    state: String,
    expected: Vec<usize>,
}

fn process_state(state: State, cache: &mut HashMap<State, u128>) -> u128 {
    /* Check if we've already calculated this state */

    if let Some(&value) = cache.get(&state) {
        return value;
    }

    /* Base cases */

    if state.state.len() == 0 {
        match state.expected.len() {
            0 => return 1,
            _ => return 0,
        }
    }

    if state.expected.len() == 0 {
        match state.state.chars().any(|c| c == '#'){
            true => return 0,
            false => return 1,
        }
    }

    /* Recursive cases */

    let mut result = 0;

    if ".?".contains(state.state.chars().nth(0).unwrap()) {
        let new_state = State{
            state: state.state.chars().skip(1).collect(),
            expected: state.expected.clone(),
        };
        result += process_state(new_state, cache);
    }

    if "#?".contains(state.state.chars().nth(0).unwrap()) {
        if state.expected[0] <= state.state.len() &&
            state.state.chars().take(state.expected[0]).all(|c| c != '.') &&
            ( state.state.len() == state.expected[0] || state.state.chars().nth(state.expected[0]).unwrap() != '#' ){
            let new_state = State{
                state: state.state.chars().skip(state.expected[0]+1).collect(),
                expected: state.expected[1..].to_vec(),
            };

            result += process_state(new_state, cache);
        }
        else{
            result += 0;
        }
    }

    /* Cache the result */
    cache.insert(state.clone(), result);

    result
}

pub fn process(input: &str) -> u128 {
    input.lines()
        .map(|line| {
            let (_state, __expected) = line.split_at(line.find(" ").unwrap());
            let mut state = _state.to_string();
            let mut _expected = __expected.trim().to_string();
            for _ in 0..4{
                state = format!("{}?{}", state, _state);
                _expected = format!("{},{}", _expected, __expected.trim());
            }

            let expected = _expected
                .trim()
                .split(",")
                .map(|x| x.parse::<usize>().expect("Expected a number"))
                .collect::<Vec<usize>>();

            let mut cache: HashMap<State, u128> = HashMap::new();
            process_state(State{state: state.to_string(), expected}, &mut cache)

        }).sum()
}

#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn test_part2(){
        let result = process("???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1");
        assert_eq!(result, 525152);
    }
}

use std::collections::HashMap;

fn hash(input: String, cache: &mut HashMap<String, usize>) -> usize{
    let mut input_copy = input.clone();
    if let Some(last_char) = input_copy.pop(){
        let mut result: usize;

        if let Some(cached_result) = cache.get(&input_copy){
            result = *cached_result;
        }
        else {
            result = hash(input_copy, cache);
        }

        if last_char != '\n'{
            result = (result + (last_char as usize)) * 17 % 256;
            cache.insert(input, result);
        }
        return result;
    }
    else {
        return 0;
    }
}

pub fn process(input: &str) -> usize {
    let mut cache: HashMap<String, usize> = HashMap::new();
    input.split(",")
        .map(|x| hash(x.to_string(), &mut cache))
        .sum()
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_part1(){
        let result = process("rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7");
        assert_eq!(result, 1320);
    }
}


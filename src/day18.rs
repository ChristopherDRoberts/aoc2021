use std::collections::VecDeque;

pub fn part1(input: &str) -> usize {
    let mut numbers = read_input(input);
    while numbers.len() > 1 {
        let lhs = numbers.pop_front();
        let rhs = numbers.pop_front();
        let sum = add_and_reduce(lhs.unwrap(), rhs.unwrap());
        numbers.push_front(sum);
    }
    magnitude(&numbers[0])
}

pub fn part2(input: &str) -> usize {
    let numbers = read_input(input);
    let mut max_magnitude = 0;
    for i in 0..numbers.len(){
        for j in 0..numbers.len(){
            if i == j {continue};
            let lhs = numbers[i].clone();
            let rhs = numbers[j].clone();
            let sum = add_and_reduce(lhs, rhs);
            let mag = magnitude(&sum);
            if mag > max_magnitude {
                max_magnitude = mag;
            }
        }
    }
    return max_magnitude;
}

fn read_input(input: &str) -> VecDeque<Number> {
    let mut numbers = VecDeque::new();
    for line in input.lines() {
        let chars: Vec<char> = line.trim().chars().collect();
        let (number, _) = parse_number(&chars);
        numbers.push_back(number);
    }
    return numbers;
}

fn parse_number(chars: &[char]) -> (Number, usize) {
    let mut chars_read = 0;
    let c = chars[chars_read];
    chars_read += 1;
    match c {
        '[' => {
            let (left, new_chars_read) = parse_number(&chars[chars_read..]);
            chars_read += new_chars_read;
            let (right, new_chars_read) = parse_number(&chars[chars_read..]);
            chars_read += new_chars_read;
            return (
                Number::Pair(Box::new(left), Box::new(right)),
                chars_read + 1,
            );
        }
        x @ '0'..='9' => {
            let value = x.to_digit(10).unwrap() as usize;
            return (Number::Regular(value), chars_read + 1);
        }
        _ => panic!(),
    }
}

fn add_and_reduce(lhs: Number, rhs: Number) -> Number {
    let mut number = Number::new_pair(lhs, rhs);
    loop {
        if let (Some(new_number), _, _) = try_explode(&number, 0) {
            number = new_number;
            continue;
        } else if let Some(new_number) = try_split(&number) {
            number = new_number;
            continue;
        } else {
            break;
        }
    }
    return number;
}

fn format_number(number: &Number, string: &mut String) -> String {
    match number {
        Number::Regular(value) => string.push_str(&value.to_string()),
        Number::Pair(left, right) => {
            string.push('[');
            string.push_str(&format_number(left, &mut String::new()));
            string.push(',');
            string.push_str(&format_number(right, &mut String::new()));
            string.push(']');
        }
    }
    return string.to_owned();
}

fn magnitude(number: &Number) -> usize {
    match number {
        Number::Regular(value) => *value,
        Number::Pair(left, right) => 3 * magnitude(left) + 2 * magnitude(right),
    }
}

fn try_explode(
    number: &Number,
    nesting_depth: usize,
) -> (Option<Number>, Option<usize>, Option<usize>) {
    if nesting_depth == 4 {
        if let Number::Pair(left, right) = number {
            if let (Number::Regular(left_value), Number::Regular(right_value)) = (&**left, &**right)
            {
                return (
                    Some(Number::Regular(0)),
                    Some(*left_value),
                    Some(*right_value),
                );
            } else {
                panic!()
            }
        } else {
            return (None, None, None);
        }
    }
    match number {
        Number::Regular(_) => return (None, None, None),
        Number::Pair(left, right) => {
            if let (Some(number), x, Some(right_value)) = try_explode(left, nesting_depth + 1) {
                let new_right = add_to_right(right, right_value);
                let new_number = Number::Pair(Box::new(number), Box::new(new_right));
                return (Some(new_number), x, None);
            } else if let (Some(number), x, None) = try_explode(left, nesting_depth + 1) {
                let new_number = Number::Pair(Box::new(number), right.clone());
                return (Some(new_number), x, None);
            } else if let (Some(number), Some(left_value), x) =
                try_explode(right, nesting_depth + 1)
            {
                let new_left = add_to_left(left, left_value);
                let new_number = Number::Pair(Box::new(new_left), Box::new(number));
                return (Some(new_number), None, x);
            } else if let (Some(number), None, x) = try_explode(right, nesting_depth + 1) {
                let new_number = Number::Pair(left.clone(), Box::new(number));
                return (Some(new_number), None, x);
            }
        }
    }
    return (None, None, None);
}

fn add_to_left(number: &Number, value: usize) -> Number {
    match number {
        Number::Regular(x) => Number::Regular(x + value),
        Number::Pair(left, right) => {
            Number::Pair(left.clone(), Box::new(add_to_left(right, value)))
        }
    }
}

fn add_to_right(number: &Number, value: usize) -> Number {
    match number {
        Number::Regular(x) => Number::Regular(x + value),
        Number::Pair(left, right) => {
            Number::Pair(Box::new(add_to_right(left, value)), right.clone())
        }
    }
}

fn try_split(number: &Number) -> Option<Number> {
    match number {
        Number::Regular(value) => {
            if *value < 10 {
                return None;
            }
            if value % 2 == 0 {
                let left = Number::Regular(value / 2);
                let right = Number::Regular(value / 2);
                return Some(Number::Pair(Box::new(left), Box::new(right)));
            }
            let left = Number::Regular(value / 2);
            let right = Number::Regular(value / 2 + 1);
            return Some(Number::Pair(Box::new(left), Box::new(right)));
        }

        Number::Pair(left, right) => {
            if let Some(number) = try_split(left) {
                return Some(Number::Pair(Box::new(number), right.clone()));
            } else if let Some(number) = try_split(right) {
                return Some(Number::Pair(left.clone(), Box::new(number)));
            } else {
                return None;
            }
        }
    }
}

#[derive(Debug, Clone)]
enum Number {
    Regular(usize),
    Pair(Box<Number>, Box<Number>),
}

impl Number {
    fn new_pair(left: Number, right: Number) -> Self {
        Number::Pair(Box::new(left), Box::new(right))
    }
}

#[cfg(test)]
mod tests_day18 {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
        [[[5,[2,8]],4],[5,[[9,9],0]]]
        [6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
        [[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
        [[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
        [[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
        [[[[5,4],[7,7]],8],[[8,3],8]]
        [[9,3],[[9,9],[6,[4,9]]]]
        [[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
        [[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]";
        let result = part1(input);
        assert_eq!(result, 4140);
    }

    #[test]
    fn test_part2() {
        let input = "[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
        [[[5,[2,8]],4],[5,[[9,9],0]]]
        [6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
        [[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
        [[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
        [[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
        [[[[5,4],[7,7]],8],[[8,3],8]]
        [[9,3],[[9,9],[6,[4,9]]]]
        [[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
        [[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]";
        let result = part2(input);
        assert_eq!(result, 3993);
    }
}

pub fn part1(input: &str) -> usize {
    let lines = read_input(input);
    let mut illegal_chars = Vec::new();
    for line in lines {
        match parse_line(line) {
            ParseResult::Corrupted(c) => illegal_chars.push(c),
            _ => (),
        }
    }
    return score_illegal_chars(illegal_chars);
}

pub fn part2(input: &str) -> usize {
    let lines = read_input(input);
    let mut incomplete_lines = Vec::new();
    for line in lines {
        match parse_line(line) {
            ParseResult::Incomplete(chars) => incomplete_lines.push(chars),
            _ => (),
        }
    }
    let mut completions = Vec::new();
    for line in &mut incomplete_lines {
        completions.push(complete_line(line))
    }
    let x = 1;
    return score_completions(completions);
}

fn parse_line(chars: Vec<char>) -> ParseResult {
    let mut stack = Vec::new();
    let mut seen_chars = Vec::new();
    for c in chars {
        match c {
            '(' => {
                stack.push(c);
                seen_chars.push(c)
            }
            '[' => {
                stack.push(c);
                seen_chars.push(c)
            }
            '{' => {
                stack.push(c);
                seen_chars.push(c)
            }
            '<' => {
                stack.push(c);
                seen_chars.push(c)
            }
            ')' => {
                seen_chars.push(c);
                let x = stack.pop().unwrap();
                if x != '(' {
                    return ParseResult::Corrupted(c);
                }
            }
            ']' => {
                seen_chars.push(c);
                let x = stack.pop().unwrap();
                if x != '[' {
                    return ParseResult::Corrupted(c);
                }
            }
            '}' => {
                seen_chars.push(c);
                let x = stack.pop().unwrap();
                if x != '{' {
                    return ParseResult::Corrupted(c);
                }
            }
            '>' => {
                seen_chars.push(c);
                let x = stack.pop().unwrap();
                if x != '<' {
                    return ParseResult::Corrupted(c);
                }
            }
            _ => (),
        }
    }
    if stack.len() == 0 {
        return ParseResult::Complete;
    } else {
        return ParseResult::Incomplete(seen_chars);
    }
}

fn complete_line(chars: &mut Vec<char>) -> Vec<char> {
    let mut completion = Vec::new();
    let mut stack = Vec::new();
    while chars.len() > 0 {
        let c = chars.pop().unwrap();
        match c {
            ')' => stack.push(c),
            ']' => stack.push(c),
            '}' => stack.push(c),
            '>' => stack.push(c),
            '(' => {
                if let None = stack.pop() {
                    completion.push(')')
                }
            }
            '[' => {
                if let None = stack.pop() {
                    completion.push(']')
                }
            }
            '{' => {
                if let None = stack.pop() {
                    completion.push('}')
                }
            }
            '<' => {
                if let None = stack.pop() {
                    completion.push('>')
                }
            }
            _ => ()
        }
    }
    return completion;
}

enum ParseResult {
    Corrupted(char),
    Incomplete(Vec<char>),
    Complete,
}

fn score_illegal_chars(illegal_chars: Vec<char>) -> usize {
    illegal_chars.iter().fold(0, |acc, c| match c {
        ')' => acc + 3,
        ']' => acc + 57,
        '}' => acc + 1197,
        '>' => acc + 25137,
        _ => acc,
    })
}

fn score_completions(completions: Vec<Vec<char>>) -> usize {
    let mut scores = Vec::new();
    for completion in completions {
        let mut score = 0;
        for c in completion{
            match c {
                ')' => {score = score*5+1}
                ']' => {score = score*5+2}
                '}' => {score = score*5+3}
                '>' => {score = score*5+4}
                _ => ()
            }
        }
        scores.push(score);
    }
    scores.sort();
    return scores[scores.len()/2]
}

fn read_input(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|l| l.trim().chars().collect()).collect()
}

#[cfg(test)]
mod tests_day10 {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "[({(<(())[]>[[{[]{<()<>>
            [(()[<>])]({[<{<<[]>>(
            {([(<{}[<>[]}>{[]{[(<()>
            (((({<>}<{<{<>}{[]{[]{}
            [[<[([]))<([[{}[[()]]]
            [{[{({}]{}}([{[{{{}}([]
            {<[[]]>}<{[{[{[]{()[[[]
            [<(<(<(<{}))><([]([]()
            <{([([[(<>()){}]>(<<{{
            <{([{{}}[<[[[<>{}]]]>[]]";
        let result = part1(input);
        assert_eq!(result, 26397);
    }

    #[test]
    fn test_part2() {
        let input = "[({(<(())[]>[[{[]{<()<>>
            [(()[<>])]({[<{<<[]>>(
            {([(<{}[<>[]}>{[]{[(<()>
            (((({<>}<{<{<>}{[]{[]{}
            [[<[([]))<([[{}[[()]]]
            [{[{({}]{}}([{[{{{}}([]
            {<[[]]>}<{[{[{[]{()[[[]
            [<(<(<(<{}))><([]([]()
            <{([([[(<>()){}]>(<<{{
            <{([{{}}[<[[[<>{}]]]>[]]";
        let result = part2(input);
        assert_eq!(result, 288957);
    }
}

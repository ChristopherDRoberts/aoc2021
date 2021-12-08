use std::collections::HashMap;

pub fn part1(input: &str) -> usize {
    let displays = read_input(input);
    let mut count = 0;
    for display in displays {
        let unique_count = display
            .outputs
            .into_iter()
            .filter(|x| {
                let length = x.len();
                length == 2 || length == 3 || length == 4 || length == 7
            })
            .count();
        count += unique_count;
    }
    return count;
}

pub fn part2(input: &str) -> usize {
    let displays = read_input(input);
    let signatures = get_signatures();
    let mut count = 0;
    for display in displays {
        let output = decode_output(display, &signatures);
        count += output;
    }
    return count;
}

fn decode_output(display: Display, signatures: &HashMap<usize, usize>) -> usize {
    let unique_segments = get_unique_segments(&display.signals);
    let mut signals_signatures = HashMap::new();
    for signal in &display.signals {
        let signature = get_signature(signal, &unique_segments);
        signals_signatures.insert(signal, signature);
    }
    1000 * signatures
        .get(signals_signatures.get(&display.outputs[0]).unwrap())
        .unwrap()
        + 100
            * signatures
                .get(signals_signatures.get(&display.outputs[1]).unwrap())
                .unwrap()
        + 10 * signatures
            .get(signals_signatures.get(&display.outputs[2]).unwrap())
            .unwrap()
        + signatures
            .get(signals_signatures.get(&display.outputs[3]).unwrap())
            .unwrap()
}

fn get_unique_segments(signals: &Vec<String>) -> Vec<String> {
    let mut segments = vec![String::new(); 3];
    for signal in signals {
        if signal.len() == 2 {
            segments[0] = signal.clone();
        };
        if signal.len() == 4 {
            segments[1] = signal.clone();
        };
        if signal.len() == 3 {
            segments[2] = signal.clone();
        };
    }
    return segments;
}

fn get_signatures() -> HashMap<usize, usize> {
    let segments = vec![
        "abcefg", "cf", "acdeg", "acdfg", "bcdf", "abdfg", "abdefg", "acf", "abcdefg", "abcdfg",
    ];
    let unique_segments = vec![
        segments[1].to_string(),
        segments[4].to_string(),
        segments[7].to_string(),
    ];
    let mut map = HashMap::new();
    for (i, segment) in segments.iter().enumerate() {
        let signature = get_signature(segment, &unique_segments);
        map.insert(signature, i);
    }
    return map;
}

fn get_signature(segment: &str, unique_segments: &Vec<String>) -> usize {
    1000 * segment.len()
        + 100 * common_chars_count(segment, unique_segments[0].as_str())
        + 10 * common_chars_count(segment, unique_segments[1].as_str())
        + common_chars_count(segment, unique_segments[2].as_str())
}

fn common_chars_count(first: &str, second: &str) -> usize {
    if first.len() < second.len() {
        let mut count = 0;
        for c in first.chars() {
            if second.contains(c) {
                count += 1
            };
        }
        return count;
    } else {
        let mut count = 0;
        for c in second.chars() {
            if first.contains(c) {
                count += 1
            };
        }
        return count;
    }
}

fn read_input(input: &str) -> Vec<Display> {
    let mut displays = Vec::new();
    for line in input.lines() {
        let input_parts: Vec<&str> = line.split(" | ").collect();
        let signals = input_parts[0].split(" ").map(|s| sort_string(s)).collect();
        let outputs = input_parts[1].split(" ").map(|s| sort_string(s)).collect();
        displays.push(Display { signals, outputs });
    }
    return displays;
}

fn sort_string(string: &str) -> String {
    let mut chars: Vec<char> = string.chars().collect();
    chars.sort();
    let mut s = String::new();
    for c in chars {
        s.push(c);
    }
    return s;
}

struct Display {
    signals: Vec<String>,
    outputs: Vec<String>,
}

#[cfg(test)]
mod tests_day8 {
    use super::*;

    #[test]
    fn test_part1() {
        let input =
            "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
            edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
            fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
            fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
            aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
            fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
            dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
            bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
            egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
            gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";
        let result = part1(input);
        assert_eq!(result, 26);
    }

    #[test]
    fn test_part2() {
        let input =
            "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
            edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
            fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
            fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
            aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
            fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
            dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
            bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
            egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
            gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";
        let result = part2(input);
        assert_eq!(result, 61229);
    }
}

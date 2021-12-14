use std::collections::HashMap;

pub fn part1(input: &str) -> usize {
    let (mut poly, rules) = read_input(input);
    for _ in 0..10 {
        poly = insert(&poly, &rules);
    }
    score(&poly)
}

pub fn part2(input: &str) -> usize {
    let (seed, rules) = read_input(input);
    let mut digrams = get_initial_digrams_count(&seed);
    for _ in 0..40{
        digrams = evolve_digrams(digrams, &rules); 
    }
    score_digrams(&digrams, &seed)
}

fn read_input(input: &str) -> (String, HashMap<String, Vec<String>>) {
    let parts: Vec<_> = input.split("\n\n").collect();
    let seed = String::from(parts[0]);
    let mut rules = HashMap::new();
    for line in parts[1].lines() {
        let rule_parts: Vec<_> = line.trim().split(" -> ").collect();
        let digram1 = [&rule_parts[0][0..1], rule_parts[1]].join("");
        let digram2 = [rule_parts[1], &rule_parts[0][1..]].join("");
        let digrams = vec![digram1.to_string(), digram2.to_string()];
        rules.insert(rule_parts[0].to_string(), digrams);
    }
    (seed, rules)
}

fn insert(prev: &String, rules: &HashMap<String, Vec<String>>) -> String {
    let mut next = String::new();
    for i in 0..prev.len() - 1 {
        if let Some(insert) = rules.get(&prev[i..i + 2]) {
            next.push_str(&insert[0]);
        } else {
            next.push_str(&prev[i..i + 1])
        }
    }
    next.push_str(&prev[prev.len() - 1..prev.len()]);
    return next;
}

fn score(polymer: &String) -> usize {
    let mut counts = HashMap::<char, usize>::new();
    for c in polymer.chars() {
        *counts.entry(c).or_default() += 1;
    }
    let max = counts.values().max().unwrap();
    let min = counts.values().min().unwrap();
    max - min
}

fn get_initial_digrams_count(seed: &String) -> HashMap<String, usize> {
    let mut digrams = HashMap::new();
    for i in 0..seed.len() - 1 {
        *digrams.entry((&seed[i..=i + 1]).to_string()).or_default() += 1;
    }
    return digrams;
}

fn evolve_digrams(digrams_count: HashMap<String, usize>, rules: &HashMap<String, Vec<String>>) -> HashMap<String, usize>{
    let mut new_count = HashMap::new();
    for (digram, count) in digrams_count{
        let new_digrams = &rules[&digram];
        *new_count.entry(new_digrams[0].clone()).or_default() += count;
        *new_count.entry(new_digrams[1].clone()).or_default() += count;
    }
    return new_count;
}

fn score_digrams(digrams: &HashMap<String, usize>, seed: &String) -> usize{
    let mut char_counts = HashMap::<&str, usize>::new();
    for (digram, value) in digrams{
        let c = &digram[0..1];
        *char_counts.entry(c).or_default() += value;
    }
    *char_counts.entry(&seed[seed.len()-1..]).or_default() += 1;
    let max = char_counts.values().max().unwrap();
    let min = char_counts.values().min().unwrap();
    max - min
}

#[cfg(test)]
mod tests_day14 {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "NNCB

        CH -> B
        HH -> N
        CB -> H
        NH -> C
        HB -> C
        HC -> B
        HN -> C
        NN -> C
        BH -> H
        NC -> B
        NB -> B
        BN -> B
        BB -> N
        BC -> B
        CC -> N
        CN -> C";
        let result = part1(input);
        assert_eq!(result, 1588);
    }

    #[test]
    fn test_part2() {
        let input = "NNCB

        CH -> B
        HH -> N
        CB -> H
        NH -> C
        HB -> C
        HC -> B
        HN -> C
        NN -> C
        BH -> H
        NC -> B
        NB -> B
        BN -> B
        BB -> N
        BC -> B
        CC -> N
        CN -> C";
        let result = part2(input);
        assert_eq!(result, 2188189693529);
    }
}

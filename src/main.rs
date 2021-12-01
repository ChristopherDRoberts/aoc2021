use std::fs;
use aoc2021::day1;

fn main(){
    let contents = fs::read_to_string("inputs/day1.txt").unwrap();
    let part1 = day1::part1(&contents);
    let part2 = day1::part2(&contents);
    println!("Part 1: {0}\nPart 2: {1}", part1, part2);
}
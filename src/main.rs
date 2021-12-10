use aoc2021::*;
use std::fs;

fn main() {
    let contents = fs::read_to_string("inputs/day10.txt").unwrap();
    let part1 = day10::part1(&contents);
    let part2 = day10::part2(&contents);
    println!("Part 1: {0}\nPart 2: {1}", part1, part2);
}

#[cfg(test)]
mod test_solutions {
    use super::*;

    #[test]
    fn test_day1() {
        let contents = fs::read_to_string("inputs/day1.txt").unwrap();
        let part1 = day1::part1(&contents);
        let part2 = day1::part2(&contents);

        assert_eq!(part1, 1195);
        assert_eq!(part2, 1235);
    }

    #[test]
    fn test_day2() {
        let contents = fs::read_to_string("inputs/day2.txt").unwrap();
        let part1 = day2::part1(&contents);
        let part2 = day2::part2(&contents);

        assert_eq!(part1, 1882980);
        assert_eq!(part2, 1971232560);
    }

    #[test]
    fn test_day3() {
        let contents = fs::read_to_string("inputs/day3.txt").unwrap();
        let part1 = day3::part1(&contents);
        let part2 = day3::part2(&contents);

        assert_eq!(part1, 2250414);
        assert_eq!(part2, 6085575);
    }

    #[test]
    fn test_day4() {
        let contents = fs::read_to_string("inputs/day4.txt").unwrap();
        let part1 = day4::part1(&contents);
        let part2 = day4::part2(&contents);

        assert_eq!(part1, 8136);
        assert_eq!(part2, 12738);
    }

    // TODO - Speed up
    // #[test]
    // fn test_day5() {
    //     let contents = fs::read_to_string("inputs/day5.txt").unwrap();
    //     let part1 = day5::part1(&contents);
    //     let part2 = day5::part2(&contents);

    //     assert_eq!(part1, 6710);
    //     assert_eq!(part2, 20121);
    // }

    #[test]
    fn test_day6() {
        let contents = fs::read_to_string("inputs/day6.txt").unwrap();
        let part1 = day6::part1(&contents);
        let part2 = day6::part2(&contents);

        assert_eq!(part1, 380612);
        assert_eq!(part2, 1710166656900);
    }

    #[test]
    fn test_day7() {
        let contents = fs::read_to_string("inputs/day7.txt").unwrap();
        let part1 = day7::part1(&contents);
        let part2 = day7::part2(&contents);

        assert_eq!(part1, 336131);
        assert_eq!(part2, 92676646);
    }

    #[test]
    fn test_day8() {
        let contents = fs::read_to_string("inputs/day8.txt").unwrap();
        let part1 = day8::part1(&contents);
        let part2 = day8::part2(&contents);

        assert_eq!(part1, 272);
        assert_eq!(part2, 1007675);
    }

    #[test]
    fn test_day9() {
        let contents = fs::read_to_string("inputs/day9.txt").unwrap();
        let part1 = day9::part1(&contents);
        let part2 = day9::part2(&contents);

        assert_eq!(part1, 537);
        assert_eq!(part2, 1142757);
    }

    #[test]
    fn test_day10() {
        let contents = fs::read_to_string("inputs/day10.txt").unwrap();
        let part1 = day10::part1(&contents);
        let part2 = day10::part2(&contents);

        assert_eq!(part1, 344193);
        assert_eq!(part2, 3241238967);
    }
}

use aoc2021::*;
use std::fs;

fn main() {
    let contents = fs::read_to_string("inputs/day6.txt").unwrap();
    let part1 = day6::part1(&contents);
    let part2 = day6::part2(&contents);
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
}

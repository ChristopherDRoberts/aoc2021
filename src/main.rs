use aoc2021::*;
use std::fs;

fn main() {
    let contents = fs::read_to_string("inputs/day20.txt").unwrap();
    let part1 = day20::part1(&contents);
    let part2 = day20::part2(&contents);
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

    #[test]
    fn test_day11() {
        let contents = fs::read_to_string("inputs/day11.txt").unwrap();
        let part1 = day11::part1(&contents);
        let part2 = day11::part2(&contents);

        assert_eq!(part1, 1642);
        assert_eq!(part2, 320);
    }

    #[test]
    fn test_day12() {
        let contents = fs::read_to_string("inputs/day12.txt").unwrap();
        let part1 = day12::part1(&contents);
        let part2 = day12::part2(&contents);

        assert_eq!(part1, 4792);
        assert_eq!(part2, 133360);
    }

    #[test]
    fn test_day13() {
        let contents = fs::read_to_string("inputs/day13.txt").unwrap();
        let part1 = day13::part1(&contents);
        let part2 = day13::part2(&contents);

        assert_eq!(part1, 669);
        assert_eq!(part2, 221214101319); // test uses "signature" of output
    }

    #[test]
    fn test_day14() {
        let contents = fs::read_to_string("inputs/day14.txt").unwrap();
        let part1 = day14::part1(&contents);
        let part2 = day14::part2(&contents);

        assert_eq!(part1, 2587);
        assert_eq!(part2, 3318837563123);
    }

    #[test]
    fn test_day15() {
        let contents = fs::read_to_string("inputs/day15.txt").unwrap();
        let part1 = day15::part1(&contents);
        let part2 = day15::part2(&contents);

        assert_eq!(part1, 487);
        assert_eq!(part2, 2821);
    }

    #[test]
    fn test_day16() {
        let contents = fs::read_to_string("inputs/day16.txt").unwrap();
        let part1 = day16::part1(&contents);
        let part2 = day16::part2(&contents);

        assert_eq!(part1, 1014);
        assert_eq!(part2, 1922490999789);
    }

    // Add parsing for original input?
    #[test]
    fn test_day17() {
        let contents = fs::read_to_string("inputs/day17.txt").unwrap();
        let part1 = day17::part1(&contents);
        let part2 = day17::part2(&contents);

        assert_eq!(part1, 8911);
        assert_eq!(part2, 4748);
    }

    #[test]
    fn test_day18() {
        let contents = fs::read_to_string("inputs/day18.txt").unwrap();
        let part1 = day18::part1(&contents);
        let part2 = day18::part2(&contents);

        assert_eq!(part1, 3051);
        assert_eq!(part2, 4812);
    }

    // day 19

    #[test]
    fn test_day20() {
        let contents = fs::read_to_string("inputs/day20.txt").unwrap();
        let part1 = day20::part1(&contents);
        let part2 = day20::part2(&contents);

        assert_eq!(part1, 5680);
        assert_eq!(part2, 19766);
    }
}

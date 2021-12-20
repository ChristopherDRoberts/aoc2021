pub fn part1(input: &str) -> usize {
    let mut image = Image::from_input(input);
    for _ in 0..2 {
        image.enhance();
    }
    image.lit_pixels()
}

pub fn part2(input: &str) -> usize {
    let mut image = Image::from_input(input);
    for _ in 0..50 {
        image.enhance();
    }
    image.lit_pixels()
}

struct Image {
    enhancer: Vec<char>,
    pixels: Vec<Vec<char>>,
    enhance_count: usize,
}

impl Image {
    fn from_input(input: &str) -> Self {
        let parts: Vec<&str> = input.split("\n\n").collect();
        let enhancer: Vec<char> = parts[0].chars().collect();
        let pixels: Vec<Vec<char>> = parts[1].lines().map(|l| l.chars().collect()).collect();
        let mut padded_pixels = vec![vec!['.'; pixels[0].len() + 6]; pixels.len() + 6];
        for i in 0..pixels.len() {
            for j in 0..pixels[0].len() {
                padded_pixels[i + 3][j + 3] = pixels[i][j];
            }
        }
        Image {
            enhancer,
            pixels: padded_pixels,
            enhance_count: 0,
        }
    }

    fn enhance(&mut self) {
        let pad_character = if self.enhancer[0] == '.' {
            '.'
        } else {
            if self.enhance_count % 2 == 0 {
                self.enhancer[0]
            } else {
                self.enhancer[self.enhancer.len() - 1]
            }
        };
        let mut enhanced =
            vec![vec![pad_character; self.pixels[0].len() + 2]; self.pixels.len() + 2];
        for i in 1..self.pixels.len() - 1 {
            for j in 1..self.pixels[0].len() - 1 {
                let mut chars = Vec::with_capacity(9);
                for ii in [i - 1, i, i + 1] {
                    for jj in [j - 1, j, j + 1] {
                        chars.push(self.pixels[ii][jj]);
                    }
                }
                let index = Image::to_index(&chars);
                enhanced[i + 1][j + 1] = self.enhancer[index];
            }
        }
        self.pixels = enhanced;
        self.enhance_count += 1;
    }

    fn print(&self) {
        for row in &self.pixels {
            for col in row {
                print!("{}", col);
            }
            println!("");
        }
    }

    fn to_index(chars: &Vec<char>) -> usize {
        let mut index = 0;
        let mut multiplier = 1;
        for c in chars.iter().rev() {
            let bit = if *c == '#' { 1 } else { 0 };
            index += multiplier * bit;
            multiplier = multiplier << 1;
        }
        return index;
    }

    fn lit_pixels(&self) -> usize {
        self.pixels
            .iter()
            .map(|r| r.iter().filter(|p| **p == '#').count())
            .sum()
    }
}

#[cfg(test)]
mod tests_day20 {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..##\
#..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###\
.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#.\
.#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#.....\
.#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#..\
...####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.....\
..##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#

#..#.
#....
##..#
..#..
..###";
        let result = part1(input);
        assert_eq!(result, 35);
    }

    #[test]
    fn test_part2() {
        let input = "..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..##\
#..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###\
.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#.\
.#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#.....\
.#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#..\
...####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.....\
..##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#

#..#.
#....
##..#
..#..
..###";
        let result = part2(input);
        assert_eq!(result, 3351);
    }
}

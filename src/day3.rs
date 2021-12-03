pub fn part1(input: &str) -> u32 {
    let numbers = read_input(input);
    let n = numbers[0].len();
    let mut gamma = vec![0u32; n];
    let mut epsilon = vec![0u32; n];
    for i in 0..n {
        if let Some(bit) = most_common_bit(&numbers, i) {
            gamma[i] = bit;
            epsilon[i] = 1 - bit;
        }
    }
    return binary_to_decimal(&gamma) * binary_to_decimal(&epsilon);
}

pub fn part2(input: &str) -> u32 {
    let mut oxy_rating = read_input(input);
    let mut c02_rating = oxy_rating.clone();
    let n = oxy_rating[0].len();

    // find oxy_rating_number
    for i in 0..n {
        if oxy_rating.len() == 1 {
            break;
        }
        let mut temp = Vec::new();
        if let Some(common_bit) = most_common_bit(&oxy_rating, i) {
            for rating in &oxy_rating {
                if rating[i] == common_bit {
                    temp.push(rating.clone());
                }
            }
        } else {
            for rating in &oxy_rating {
                if rating[i] == 1 {
                    temp.push(rating.clone());
                }
            }
        }
        oxy_rating = temp;
    }

    // find c02_rating_number
    for i in 0..n {
        if c02_rating.len() == 1 {
            break;
        }
        let mut temp = Vec::new();
        if let Some(common_bit) = most_common_bit(&c02_rating, i) {
            for rating in &c02_rating {
                if rating[i] != common_bit {
                    temp.push(rating.clone());
                }
            }
        } else {
            for rating in &c02_rating {
                if rating[i] != 1 {
                    temp.push(rating.clone());
                }
            }
        }
        c02_rating = temp;
    }

    return binary_to_decimal(&oxy_rating[0]) * binary_to_decimal(&c02_rating[0]);
}

fn read_input(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect()
}

fn binary_to_decimal(binary_number: &Vec<u32>) -> u32 {
    let mut decimal_number = 0;
    let n = binary_number.len() as u32;
    for i in 0..n {
        decimal_number += 2u32.pow(n - 1 - i) * binary_number[i as usize];
    }
    return decimal_number;
}

fn most_common_bit(binary_numbers: &Vec<Vec<u32>>, bit_index: usize) -> Option<u32> {
    let n = binary_numbers.len();
    let mut ones_count = 0;
    for i in 0..n {
        if binary_numbers[i][bit_index] == 1 {
            ones_count += 1
        };
    }
    if ones_count > (n - ones_count) {
        return Some(1);
    }
    if ones_count < (n - ones_count) {
        return Some(0);
    }
    return None;
}

#[cfg(test)]
mod tests_day3 {
    use super::*;

    #[test]
    fn test_part1() {
        let input =
            "00100\n11110\n10110\n10111\n10101\n01111\n00111\n11100\n10000\n11001\n00010\n01010";
        let result = part1(input);
        assert_eq!(result, 198);
    }

    #[test]
    fn test_part2() {
        let input =
            "00100\n11110\n10110\n10111\n10101\n01111\n00111\n11100\n10000\n11001\n00010\n01010";
        let result = part2(input);
        assert_eq!(result, 230);
    }
}

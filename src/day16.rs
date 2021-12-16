pub fn part1(input: &str) -> usize {
    let bits = read_input(input);
    let (packet,_) = parse_packet(&bits);
    sum_version(packet)
}

pub fn part2(input: &str) -> usize {
    0
}

fn read_input(input: &str) -> Vec<usize> {
    let mut bits = Vec::with_capacity(4 * input.len());
    for c in input.chars() {
        match c {
            '0' => bits.extend_from_slice(&[0, 0, 0, 0]),
            '1' => bits.extend_from_slice(&[0, 0, 0, 1]),
            '2' => bits.extend_from_slice(&[0, 0, 1, 0]),
            '3' => bits.extend_from_slice(&[0, 0, 1, 1]),
            '4' => bits.extend_from_slice(&[0, 1, 0, 0]),
            '5' => bits.extend_from_slice(&[0, 1, 0, 1]),
            '6' => bits.extend_from_slice(&[0, 1, 1, 0]),
            '7' => bits.extend_from_slice(&[0, 1, 1, 1]),
            '8' => bits.extend_from_slice(&[1, 0, 0, 0]),
            '9' => bits.extend_from_slice(&[1, 0, 0, 1]),
            'A' => bits.extend_from_slice(&[1, 0, 1, 0]),
            'B' => bits.extend_from_slice(&[1, 0, 1, 1]),
            'C' => bits.extend_from_slice(&[1, 1, 0, 0]),
            'D' => bits.extend_from_slice(&[1, 1, 0, 1]),
            'E' => bits.extend_from_slice(&[1, 1, 1, 0]),
            'F' => bits.extend_from_slice(&[1, 1, 1, 1]),
            _ => panic!(),
        }
    }
    return bits;
}

fn sum_version(packet: Packet) -> usize {
    let mut sum = 0;
    match packet {
        Packet::Literal { version, .. } => sum += version,
        Packet::Operator {
            version,
            subpackets,
            ..
        } => {
            sum += version;
            for packet in subpackets {
                sum += sum_version(packet);
            }
        }
    }
    return sum;
}

fn parse_packet(bits: &[usize]) -> (Packet, usize) {
    let mut bits_read = 0;
    let version = to_decimal(&bits[0..3]);
    bits_read += 3;
    let id = to_decimal(&bits[bits_read..bits_read + 3]);
    bits_read += 3;
    match id {
        4 => {
            let (value, new_bits_read) = parse_literal(&bits[bits_read..]);
            bits_read += new_bits_read;
            return (Packet::Literal { version, id, value }, bits_read);
        }
        _ => {
            let (subpackets, new_bits_read) = parse_operator(&bits[bits_read..]);
            bits_read += new_bits_read;
            return (
                Packet::Operator {
                    version,
                    id,
                    subpackets,
                },
                bits_read,
            );
        }
    }
}

fn parse_literal(bits: &[usize]) -> (usize, usize) {
    let mut bits_read = 0;
    let mut literal_value = Vec::new();
    let mut more_groups = true;
    while more_groups {
        more_groups = if bits[bits_read] == 1 { true } else { false };
        bits_read += 1;
        literal_value.extend_from_slice(&bits[bits_read..bits_read + 4]);
        bits_read += 4;
    }
    let value = to_decimal(&literal_value);
    return (value, bits_read);
}

fn parse_operator(bits: &[usize]) -> (Vec<Packet>, usize) {
    let mut bits_read = 0;
    let length_type = to_decimal(&bits[bits_read..bits_read + 1]);
    bits_read += 1;
    let mut subpackets = Vec::new();
    if length_type == 0 {
        let mut length_subpackets = to_decimal(&bits[bits_read..bits_read + 15]);
        bits_read += 15;
        while length_subpackets > 0 {
            let (packet, new_bits_read) = parse_packet(&bits[bits_read..]);
            bits_read += new_bits_read;
            length_subpackets -= new_bits_read;
            subpackets.push(packet);
        }
    } else {
        let count_subpackets = to_decimal(&bits[bits_read..bits_read + 11]);
        bits_read += 11;
        for _ in 0..count_subpackets {
            let (packet, new_bits_read) = parse_packet(&bits[bits_read..]);
            bits_read += new_bits_read;
            subpackets.push(packet);
        }
    }
    return (subpackets, bits_read);
}

fn to_decimal(bits: &[usize]) -> usize {
    let mut x = 0;
    let mut multiplier = 1;
    for b in bits.iter().rev() {
        x += multiplier * b;
        multiplier = multiplier << 1;
    }
    return x;
}

enum Packet {
    Literal {
        version: usize,
        id: usize,
        value: usize,
    },
    Operator {
        version: usize,
        id: usize,
        subpackets: Vec<Packet>,
    },
}

#[cfg(test)]
mod tests_day16 {
    use super::*;

    #[test]
    fn test_read_input() {
        let input = "D2FE28";
        let bits = read_input(input);
        assert_eq![
            bits,
            [1, 1, 0, 1, 0, 0, 1, 0, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 1, 0, 1, 0, 0, 0]
        ]
    }

    #[test]
    fn test_to_decimal() {
        let bits = [1, 0, 1, 1];
        let d = to_decimal(&bits);
        assert_eq!(d, 11);
    }

    #[test]
    fn test_parse_literal_packet() {
        let input = "D2FE28";
        let bits = read_input(input);
        let (packet, bits_read) = parse_packet(&bits);

        assert_eq!(bits_read, 21);
        if let Packet::Literal { version, id, value } = packet {
            assert_eq!(version, 6);
            assert_eq!(id, 4);
            assert_eq!(value, 2021);
        } else {
            panic!()
        }
    }

    #[test]
    fn test_parse_type_0_operator_packet() {
        let input = "38006F45291200";
        let bits = read_input(input);
        let (packet, bits_read) = parse_packet(&bits);

        assert_eq!(bits_read, 49);
        if let Packet::Operator {
            version,
            id,
            subpackets,
        } = packet
        {
            assert_eq!(version, 1);
            assert_eq!(id, 6);
            assert_eq!(subpackets.len(), 2);
            if let Packet::Literal { version, id, value } = subpackets[1] {
                assert_eq!(value, 20);
            }
        }
    }

    #[test]
    fn test_part1() {
        let input = "A0016C880162017C3686B18A3D4780";
        let result = part1(input);
        assert_eq!(result, 31);
    }

    #[test]
    fn test_part2() {
        let input = "";
        let result = part2(input);
        assert_eq!(result, 315);
    }
}

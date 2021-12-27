const INPUT_FILE: &str = include_str!("../../inputs/day16.txt");

enum Operator {
    Sum(usize),
    Product(usize),
    Minimum(Vec<usize>),
    Maximum(Vec<usize>),
    GreaterThan(Vec<usize>),
    LessThan(Vec<usize>),
    EqualTo(Vec<usize>),
}

impl Operator {
    fn insert(&mut self, value: usize) {
        match self {
            Operator::Sum(v) => {
                *v += value;
            }
            Operator::Product(v) => {
                *v *= value;
            }
            Operator::Minimum(v)
            | Operator::Maximum(v)
            | Operator::GreaterThan(v)
            | Operator::LessThan(v)
            | Operator::EqualTo(v) => v.push(value),
        }
    }
    fn evaluate(&self) -> usize {
        match self {
            Operator::Sum(v) | Operator::Product(v) => *v,
            Operator::Minimum(v) => *v.iter().min().unwrap(),
            Operator::Maximum(v) => *v.iter().max().unwrap(),
            Operator::GreaterThan(v) => {
                if v[0] > v[1] {
                    1
                } else {
                    0
                }
            }
            Operator::LessThan(v) => {
                if v[0] < v[1] {
                    1
                } else {
                    0
                }
            }
            Operator::EqualTo(v) => {
                if v[0] == v[1] {
                    1
                } else {
                    0
                }
            }
        }
    }
}

fn parse(input: String) -> Vec<char> {
    let mut decoded = String::new();
    for c in input.trim().chars() {
        decoded += &format!("{:04b}", c.to_digit(16).unwrap());
    }
    decoded.chars().collect()
}

fn binary_chars_to_number(c: &[char]) -> usize {
    let c: String = c.iter().collect();
    usize::from_str_radix(&c, 2).unwrap()
}

fn parse_packet(index: &mut usize, decoded: &[char]) -> usize {
    let version = binary_chars_to_number(&decoded[*index..*index + 3]);
    let type_id = binary_chars_to_number(&decoded[*index + 3..*index + 6]);
    *index += 6;
    println!("New packet: version={} type_id={}", version, type_id);
    let mut version_sum = version;
    if type_id == 4 {
        let mut continues = true;
        let mut val = vec![];
        while continues {
            continues = decoded[*index] == '1';
            let mut decoded: Vec<char> = decoded[*index + 1..*index + 5].iter().copied().collect();
            val.append(&mut decoded);
            *index += 5
        }
    } else {
        let length_type_id = decoded[*index];
        *index += 1;
        if length_type_id == '0' {
            let subpackage_len = binary_chars_to_number(&decoded[*index..*index + 15]);
            *index += 15;
            let parse_until = *index + subpackage_len;

            while *index < parse_until {
                version_sum += parse_packet(index, decoded);
            }
        } else {
            let subpackage_len = binary_chars_to_number(&decoded[*index..*index + 11]);
            *index += 11;
            for _ in 0..subpackage_len {
                version_sum += parse_packet(index, decoded)
            }
        }
    }
    version_sum
}

fn parse_packet_2(index: &mut usize, decoded: &[char]) -> usize {
    // Headers
    let version = binary_chars_to_number(&decoded[*index..*index + 3]);
    let type_id = binary_chars_to_number(&decoded[*index + 3..*index + 6]);
    *index += 6;

    if type_id == 4 {
        // Literal
        let mut continues = true;
        let mut val = vec![];
        while continues {
            continues = decoded[*index] == '1';
            let mut decoded: Vec<char> = decoded[*index + 1..*index + 5].iter().copied().collect();
            val.append(&mut decoded);
            *index += 5
        }
        let literal_value = binary_chars_to_number(&val);
        println!("literal: {}", literal_value);
        literal_value
    } else {
        // Operator
        let mut operator = match type_id {
            0 => Operator::Sum(0),
            1 => Operator::Product(1),
            2 => Operator::Minimum(vec![]),
            3 => Operator::Maximum(vec![]),
            5 => Operator::GreaterThan(vec![]),
            6 => Operator::LessThan(vec![]),
            7 => Operator::EqualTo(vec![]),
            _ => panic!("unrecognized operator"),
        };

        let length_type_id = decoded[*index];
        *index += 1;

        if length_type_id == '0' {
            let subpackage_len = binary_chars_to_number(&decoded[*index..*index + 15]);
            *index += 15;
            let parse_until = *index + subpackage_len;

            while *index < parse_until {
                operator.insert(parse_packet_2(index, decoded));
            }
        } else {
            let subpackage_len = binary_chars_to_number(&decoded[*index..*index + 11]);
            *index += 11;
            for _ in 0..subpackage_len {
                operator.insert(parse_packet_2(index, decoded));
            }
        }
        operator.evaluate()
    }
}

fn part_1(input: String) -> usize {
    let decoded = parse(input);
    let mut index = 0;
    parse_packet(&mut index, &decoded)
}

fn part_2(input: String) -> usize {
    let decoded = parse(input);
    let mut index = 0;
    parse_packet_2(&mut index, &decoded)
}
fn main() {
    let res1 = part_1(INPUT_FILE.to_string());
    println!("part1: {}", res1);
    let res2 = part_2(INPUT_FILE.to_string());
    println!("part2: {}", res2);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn solves_part1_example() {
        assert_eq!(part_1("D2FE28".to_string()), 6);
        assert_eq!(part_1("38006F45291200".to_string()), 9);
        assert_eq!(part_1("EE00D40C823060".to_string()), 14);
        assert_eq!(part_1("8A004A801A8002F478".to_string()), 16);
        assert_eq!(part_1("620080001611562C8802118E34".to_string()), 12);
        assert_eq!(part_1("C0015000016115A2E0802F182340".to_string()), 23);
        assert_eq!(part_1("A0016C880162017C3686B18A3D4780".to_string()), 31);
    }

    #[test]
    fn solves_part1_input() {
        assert_eq!(part_1(INPUT_FILE.to_string()), 965);
    }

    #[test]
    fn solves_part2_example() {
        assert_eq!(part_2("C200B40A82".to_string()), 3);
        assert_eq!(part_2("04005AC33890".to_string()), 54);
        assert_eq!(part_2("880086C3E88112".to_string()), 7);
        assert_eq!(part_2("CE00C43D881120".to_string()), 9);
        assert_eq!(part_2("F600BC2D8F".to_string()), 0);
        assert_eq!(part_2("D8005AC2A8F0".to_string()), 1);
        assert_eq!(part_2("9C0141080250320F1802104A08".to_string()), 1);
    }

    #[test]
    fn solves_part2_input() {
        assert_eq!(part_2(INPUT_FILE.to_string()), 116672213160);
    }
}

const INPUT_FILE: &str = include_str!("../../inputs/day16.txt");

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

fn part_1(input: String) -> usize {
    let decoded = parse(input);
    let mut index = 0;
    println!("part1: {:?}", decoded);
    parse_packet(&mut index, &decoded)
}

fn main() {
    let res1 = part_1(INPUT_FILE.to_string());
    println!("part1: {}", res1);
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
}

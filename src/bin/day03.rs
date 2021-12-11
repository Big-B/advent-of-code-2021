use advent_of_code_2021::*;

fn main() {
    let input = get_input().unwrap();
    let input: Vec<&str> = input.split_whitespace().collect();
    println!("{}", calc_gxe(&input));
    println!("{}", calc_life_support(&input));
}

fn calc_gxe(input: &[&str]) -> u64 {
    let mut mask = 0;
    let mut string = String::new();
    for i in 0..input[0].len() {
        string.push(most_common_val(input, i));
        mask |= 1 << i;
    }

    let bin = u64::from_str_radix(&string, 2).unwrap();
    bin * (!bin & mask)
}

fn calc_life_support(input: &[&str]) -> u64 {
    let mut pos = 0;
    let mut tmp: Vec<&str> = input.to_vec();
    while tmp.len() > 1 {
        let mcv = most_common_val(&tmp, pos);
        tmp = tmp
            .iter()
            .filter(|s| s.chars().nth(pos) == Some(mcv))
            .copied()
            .collect();
        pos += 1;
    }

    let o2 = u64::from_str_radix(tmp[0], 2).unwrap();

    tmp = input.to_vec();
    pos = 0;
    while tmp.len() > 1 {
        let lcv = least_common_val(&tmp, pos);
        tmp = tmp
            .iter()
            .filter(|s| s.chars().nth(pos) == Some(lcv))
            .copied()
            .collect();
        pos += 1;
    }

    let co2 = u64::from_str_radix(tmp[0], 2).unwrap();
    o2 * co2
}

fn least_common_val(input: &[&str], idx: usize) -> char {
    match most_common_val(input, idx) {
        '0' => '1',
        _ => '0',
    }
}

fn most_common_val(input: &[&str], idx: usize) -> char {
    let count = input
        .iter()
        .filter(|s| s.chars().nth(idx).unwrap() == '0')
        .count();
    if count > input.len() / 2 {
        '0'
    } else {
        '1'
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gxe() {
        let vec = vec![
            "00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100", "10000",
            "11001", "00010", "01010",
        ];

        assert_eq!(calc_gxe(&vec), 198);
    }

    #[test]
    fn test_life_support() {
        let vec = vec![
            "00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100", "10000",
            "11001", "00010", "01010",
        ];
        assert_eq!(calc_life_support(&vec), 230);
    }
}

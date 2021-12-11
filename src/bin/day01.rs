use advent_of_code_2021::*;

fn main() {
    let input: Vec<i64> = get_input()
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse::<i64>().unwrap())
        .collect();
    println!("1. {}", num_increases(&input));
    println!("2. {}", num_triplet_increases(&input));
}

fn num_increases(input: &[i64]) -> u64 {
    let (first, rest) = input.split_at(1);
    let mut prev = first[0];
    let mut sum = 0;
    for i in rest {
        if i > &prev {
            sum += 1;
        }
        prev = *i;
    }
    sum
}

fn num_triplet_increases(input: &[i64]) -> u64 {
    let triplets: Vec<i64> = input.windows(3).map(|i| i.iter().sum()).collect();
    num_increases(&triplets)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_increases() {
        let vec = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        assert_eq!(num_increases(&vec), 7);
    }

    #[test]
    fn test_triplet_increases() {
        let vec = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        assert_eq!(num_triplet_increases(&vec), 5);
    }
}

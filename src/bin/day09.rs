use advent_of_code_2021::*;
use std::cmp::Reverse;
use std::collections::BTreeSet;

fn main() {
    let input = get_input().unwrap();
    println!("Sum of risk levels: {}", sum_of_risk(&input));
    println!("Product of basins: {}", product_of_basin(&input));
}

fn sum_of_risk(input: &str) -> usize {
    let input: Vec<Vec<u32>> = input
        .lines()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();
    calc_risk(&input)
}

fn calc_risk(input: &[Vec<u32>]) -> usize {
    let len = input[0].len();
    let mut sum = 0;
    for i in 0..input.len() {
        for j in 0..len {
            let val = input[i][j];
            let up = if i > 0 { input[i - 1][j] } else { 9 };
            let left = if j > 0 { input[i][j - 1] } else { 9 };
            let down = if i + 1 < input.len() {
                input[i + 1][j]
            } else {
                9
            };
            let right = if j + 1 < len { input[i][j + 1] } else { 9 };
            if val < up && val < down && val < left && val < right {
                sum += val + 1;
            }
        }
    }
    sum.try_into().unwrap()
}

fn product_of_basin(input: &str) -> usize {
    let input: Vec<Vec<u32>> = input
        .lines()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();

    let len = input[0].len();
    let mut sets = Vec::new();
    for i in 0..input.len() {
        for j in 0..len {
            let mut set = BTreeSet::new();
            let val = input[i][j];
            if val == 9 {
                continue;
            } else {
                set.insert((i, j));
            }
            let up = if i > 0 { input[i - 1][j] } else { 9 };
            let left = if j > 0 { input[i][j - 1] } else { 9 };
            let down = if i + 1 < input.len() {
                input[i + 1][j]
            } else {
                9
            };
            let right = if j + 1 < len { input[i][j + 1] } else { 9 };

            if up != 9 {
                set.insert((i - 1, j));
            }

            if left != 9 {
                set.insert((i, j - 1));
            }

            if down != 9 {
                set.insert((i + 1, j));
            }

            if right != 9 {
                set.insert((i, j + 1));
            }
            sets.push(set);
        }
    }
    let mut reduced = reduce_to_disjoint_sets(sets);
    reduced.sort_unstable_by_key(|a| Reverse(a.len()));
    reduced[0].len() * reduced[1].len() * reduced[2].len()
}

fn reduce_to_disjoint_sets(
    mut sets: Vec<BTreeSet<(usize, usize)>>,
) -> Vec<BTreeSet<(usize, usize)>> {
    let mut vec = Vec::new();
    while !sets.is_empty() {
        let mut set = sets.pop().unwrap();
        loop {
            let mut intersects: Vec<_> = sets
                .iter()
                .filter(|f| !set.is_disjoint(f))
                .cloned()
                .collect();
            sets = sets
                .iter()
                .filter(|f| set.is_disjoint(f))
                .cloned()
                .collect();
            if intersects.is_empty() {
                break;
            } else {
                for s in intersects.iter_mut() {
                    set.append(s);
                }
            }
        }
        vec.push(set);
    }
    vec
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_unique_numbers() {
        let input = "2199943210
3987894921
9856789892
8767896789
9899965678";
        assert_eq!(sum_of_risk(input), 15);
    }

    #[test]
    fn test_basin_size() {
        let input = "2199943210
3987894921
9856789892
8767896789
9899965678";
        assert_eq!(product_of_basin(input), 1134);
    }
}

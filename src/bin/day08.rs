use advent_of_code_2021::*;
use std::collections::{BTreeSet, HashMap};

fn main() {
    let input = get_input().unwrap();
    println!("Num Unique Numbers: {}", count_unique_numbers(&input));
    println!("Sum of outputs: {}", sum_output_values(&input));
}

fn count_unique_numbers(input: &str) -> usize {
    input
        .lines()
        .map(|l| l.split('|').collect::<Vec<&str>>()) // Split on |
        .map(|v| v[1]) // Grab second portion
        .map(|s| {
            let vec: Vec<&str> = s.split_whitespace().collect();
            vec.iter()
                .map(|w| w.trim()) // get rid of whitespace chars
                .map(|w| w.chars().count()) // count chars
                .filter(|&c| c == 2 || c == 3 || c == 4 || c == 7) // keep known values
                .count()
        })
        .sum()
}

fn sum_output_values(input: &str) -> usize {
    let entries: Vec<Vec<&str>> = input
        .lines()
        .map(|l| l.split('|').collect::<Vec<&str>>())
        .collect();

    let mut sum = 0;
    for e in entries {
        let seq: Vec<&str> = e[0].trim().split_whitespace().collect();
        let output: Vec<BTreeSet<char>> = e[1]
            .split_whitespace()
            .map(|s| s.chars().collect())
            .collect();
        let map = map_str_to_num(&seq);

        let mut num = 0;
        for v in output {
            num *= 10;
            num += map.get(&v).unwrap();
        }
        sum += num;
    }
    sum.try_into().unwrap()
}

// Known by size: 1, 4, 7, 8
// size 1 = 2
// size 4 = 4
// size 7 = 3
// size 8 = 7

// Needs Determining: 0, 2, 3, 5, 6, 9
// size 0 = 6
// size 2 = 5
// size 3 = 5
// size 5 = 5
// size 6 = 6
// size 9 = 6
// 9 contains 4
// 0 contains 1
// 6 contains neither
// 3 contains 7
// 6 contains 5
// 2 is neither
fn map_str_to_num(input: &[&str]) -> HashMap<BTreeSet<char>, u64> {
    let sorted: Vec<BTreeSet<char>> = input.iter().map(|s| s.chars().collect()).collect();
    let mut map = HashMap::new();
    for set in &sorted {
        match set.len() {
            2 => map.insert(1, set.clone()),
            4 => map.insert(4, set.clone()),
            3 => map.insert(7, set.clone()),
            7 => map.insert(8, set.clone()),
            _ => continue,
        };
    }

    let sixes: Vec<BTreeSet<char>> = sorted.iter().filter(|x| x.len() == 6).cloned().collect();
    for set in sixes {
        if set.is_superset(map.get(&4).unwrap()) {
            map.insert(9, set);
        } else if set.is_superset(map.get(&1).unwrap()) {
            map.insert(0, set);
        } else {
            map.insert(6, set);
        }
    }

    let fives: Vec<BTreeSet<char>> = sorted.iter().filter(|x| x.len() == 5).cloned().collect();
    for set in fives {
        if set.is_superset(map.get(&7).unwrap()) {
            map.insert(3, set);
        } else if map.get(&6).unwrap().is_superset(&set) {
            map.insert(5, set);
        } else {
            map.insert(2, set);
        }
    }
    let mut retmap = HashMap::new();
    for (k, v) in map {
        retmap.insert(v, k);
    }

    retmap
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_unique_numbers() {
        let input =
            "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";
        assert_eq!(count_unique_numbers(input), 26);
    }

    #[test]
    fn test_sum_output_values() {
        let input =
            "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";
        assert_eq!(sum_output_values(input), 61229);
    }
}

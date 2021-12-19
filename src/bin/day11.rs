use advent_of_code_2021::*;

fn main() {
    let input = get_input().unwrap();
    println!("Num Flashes: {}", num_flashes(&input, 100));
    println!("All Flashes: {}", all_flashes(&input));
}

fn num_flashes(input: &str, steps: usize) -> usize {
    let mut sum = 0;
    let mut input: Vec<Vec<usize>> = input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect()
        })
        .collect();

    for _ in 0..steps {
        sum += take_step(&mut input);
    }
    sum
}

fn all_flashes(input: &str) -> usize {
    let mut input: Vec<Vec<usize>> = input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect()
        })
        .collect();

    let total_count = input.iter().map(|v| v.len()).sum();
    let mut ret = 0;
    for i in 0.. {
        if take_step(&mut input) == total_count {
            ret = i;
            break;
        }
    }
    ret + 1
}

fn take_step(input: &mut [Vec<usize>]) -> usize {
    for v in input.iter_mut() {
        for e in v.iter_mut() {
            *e += 1;
        }
    }

    let mut vec = Vec::new();
    for (i, v) in input.iter().enumerate() {
        for (j, e) in v.iter().enumerate() {
            if *e == 10 {
                vec.push((i, j));
            }
        }
    }

    for (i, j) in vec {
        flash(input, i, j);
    }

    let sum = input
        .iter()
        .map(|v| v.iter().filter(|&&x| x > 9).count())
        .sum();

    for v in input.iter_mut() {
        for e in v.iter_mut() {
            if *e > 9 {
                *e = 0;
            }
        }
    }

    sum
}

fn flash(input: &mut [Vec<usize>], i: usize, j: usize) {
    input[i][j] += 1;
    if i > 0 {
        input[i - 1][j] += 1;
        if input[i - 1][j] == 10 {
            flash(input, i - 1, j);
        }
        if j > 0 {
            input[i - 1][j - 1] += 1;
            if input[i - 1][j - 1] == 10 {
                flash(input, i - 1, j - 1);
            }
        }
        if j < input[i].len() - 1 {
            input[i - 1][j + 1] += 1;
            if input[i - 1][j + 1] == 10 {
                flash(input, i - 1, j + 1);
            }
        }
    }
    if i < input.len() - 1 {
        input[i + 1][j] += 1;
        if input[i + 1][j] == 10 {
            flash(input, i + 1, j);
        }
        if j > 0 {
            input[i + 1][j - 1] += 1;
            if input[i + 1][j - 1] == 10 {
                flash(input, i + 1, j - 1);
            }
        }
        if j < input[i].len() - 1 {
            input[i + 1][j + 1] += 1;
            if input[i + 1][j + 1] == 10 {
                flash(input, i + 1, j + 1);
            }
        }
    }
    if j > 0 {
        input[i][j - 1] += 1;
        if input[i][j - 1] == 10 {
            flash(input, i, j - 1);
        }
    }
    if j < input[i].len() - 1 {
        input[i][j + 1] += 1;
        if input[i][j + 1] == 10 {
            flash(input, i, j + 1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_num_flashes() {
        let input = "11111
19991
19191
19991
11111";
        assert_eq!(num_flashes(input, 1), 9);
    }

    #[test]
    fn test_num_flashes() {
        let input = "5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526";
        assert_eq!(num_flashes(input, 100), 1656);
    }

    #[test]
    fn test_all_flashes() {
        let input = "5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526";
        assert_eq!(all_flashes(input), 195);
    }
}

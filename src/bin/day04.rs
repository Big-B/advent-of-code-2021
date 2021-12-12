use advent_of_code_2021::*;
use std::collections::{HashMap, VecDeque};

fn main() {
    let input = get_input().unwrap();
    let input: Vec<&str> = input.lines().filter(|s| !s.is_empty()).collect();
    let mut bingo = Bingo::new(&input);

    while !bingo.is_done() {
        bingo.draw();
    }

    println!("1. {}", bingo.first().calc_score());
    println!("2. {}", bingo.last().calc_score());
}

fn is_bingo(board: &[[bool; 5]; 5], row: usize, col: usize) -> bool {
    let mut row_solved = true;
    let mut col_solved = true;
    for i in 0..5 {
        if !board[i][col] {
            col_solved = false;
        }

        if !board[row][i] {
            row_solved = false;
        }
    }

    row_solved || col_solved
}

struct Index {
    pub board: usize,
    pub row: usize,
    pub col: usize,
}

impl Index {
    pub fn new(board: usize, row: usize, col: usize) -> Self {
        Index { board, row, col }
    }
}

#[derive(Clone, Copy)]
struct Solution {
    pub board: [[u64; 5]; 5],
    pub pieces: [[bool; 5]; 5],
    pub val: u64,
}

impl Solution {
    pub fn new(board: [[u64; 5]; 5], pieces: [[bool; 5]; 5], val: u64) -> Self {
        Solution { board, pieces, val }
    }

    pub fn calc_score(&self) -> u64 {
        let mut res = 0;
        for i in 0..5 {
            for j in 0..5 {
                if !self.pieces[i][j] {
                    res += self.board[i][j];
                }
            }
        }
        self.val * res
    }
}

struct Bingo {
    draw: VecDeque<u64>,
    boards: Vec<Option<[[u64; 5]; 5]>>,
    pieces: Vec<Option<[[bool; 5]; 5]>>,
    num_map: HashMap<u64, Vec<Index>>,
    solved: Vec<Solution>,
}

impl Bingo {
    pub fn new(input: &[&str]) -> Self {
        let mut num_map = HashMap::new();
        let draw = input[0]
            .split_terminator(',')
            .map(|x| x.parse::<u64>().unwrap())
            .collect();

        let mut boards = Vec::new();
        for (idx, line) in input[1..].iter().enumerate() {
            if idx % 5 == 0 {
                boards.push(Some([[0; 5]; 5]));
            }

            let vals: Vec<u64> = line
                .split_whitespace()
                .map(|x| x.parse::<u64>().unwrap())
                .collect();

            for (i, val) in vals.iter().enumerate().take(5) {
                boards[idx / 5].as_mut().unwrap()[idx % 5][i] = *val;
                let vec = num_map.entry(*val).or_insert_with(Vec::new);
                vec.push(Index::new(idx / 5, idx % 5, i));
            }
        }

        let mut pieces = Vec::new();
        pieces.resize(boards.len(), Some([[false; 5]; 5]));

        Bingo {
            draw,
            boards,
            pieces,
            num_map,
            solved: Vec::new(),
        }
    }

    pub fn draw(&mut self) {
        if let Some(val) = self.draw.pop_front() {
            for index in self.num_map.get(&val).unwrap() {
                if let Some(pieces) = self.pieces[index.board].as_mut() {
                    pieces[index.row][index.col] = true;
                    if is_bingo(pieces, index.row, index.col) {
                        self.solved.push(Solution::new(
                            self.boards[index.board].unwrap(),
                            *pieces,
                            val,
                        ));
                        self.boards[index.board] = None;
                        self.pieces[index.board] = None;
                    }
                }
            }
        }
    }

    pub fn is_done(&self) -> bool {
        self.draw.is_empty()
    }

    pub fn first(&self) -> Solution {
        *self.solved.first().unwrap()
    }

    pub fn last(&self) -> Solution {
        *self.solved.last().unwrap()
    }
}

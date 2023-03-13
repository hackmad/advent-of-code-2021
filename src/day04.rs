use regex::Regex;
use std::fs;

#[derive(Copy, Clone, Default)]
struct BoardCell {
    number: u32,
    marked: bool,
}

type BoardCells = [[BoardCell; 5]; 5];

#[derive(Clone, Default)]
struct Board {
    cells: BoardCells,
}

impl Board {
    fn mark(&mut self, number: u32) {
        // Let caller verify board is incomplete.
        for r in 0..5 {
            for c in 0..5 {
                if self.cells[r][c].number == number {
                    self.cells[r][c].marked = true;
                }
            }
        }
    }

    fn score(&self, last_number: u32) -> u32 {
        let mut score = 0_u32;
        for r in 0..5 {
            for c in 0..5 {
                if !self.cells[r][c].marked {
                    score += self.cells[r][c].number;
                }
            }
        }
        score * last_number
    }

    fn is_complete(&self) -> bool {
        for r in 0..5 {
            let mut complete = true;
            for c in 0..5 {
                complete = complete && self.cells[r][c].marked
            }
            if complete {
                return true;
            }
        }

        for c in 0..5 {
            let mut complete = true;
            for r in 0..5 {
                complete = complete && self.cells[r][c].marked
            }
            if complete {
                return true;
            }
        }

        false
    }
}

#[derive(Clone)]
struct Bingo {
    draws: Vec<u32>,
    boards: Vec<Board>,
}

impl Bingo {
    fn new(input_file: &str) -> Self {
        let contents =
            fs::read_to_string(input_file).expect("Something went wrong reading the file");
        let contents: Vec<&str> = contents.lines().collect();

        let draws: Vec<u32> = contents[0]
            .split(",")
            .map(|s| {
                s.parse::<u32>()
                    .expect(&format!("invalid non-numeric input {}", s))
            })
            .collect();

        let seperator = Regex::new(r"( +)").expect("invalid regex");
        let mut boards: Vec<Board> = vec![];
        let mut i = 2; // skip blank line
        while i < contents.len() {
            let mut board = Board::default();

            for r in 0..5 {
                let row: Vec<u32> = seperator
                    .split(contents[i].trim())
                    .into_iter()
                    .map(|s| {
                        s.parse::<u32>()
                            .expect(&format!("invalid non-numeric input {}", s))
                    })
                    .collect();
                for c in 0..5 {
                    board.cells[r][c].number = row[c];
                }
                i += 1;
            }
            boards.push(board);

            i += 1; // blank line
        }

        Self { draws, boards }
    }
}

pub fn part1(input_file: &str) {
    let mut bingo = Bingo::new(input_file);
    let n = bingo.draws.len();
    let b = bingo.boards.len();
    let mut completed_board: Option<usize> = None;
    let mut last_number: u32 = 0;

    for i in 0..n {
        let number = bingo.draws[i];
        for j in 0..b {
            if bingo.boards[j].is_complete() {
                continue;
            }

            bingo.boards[j].mark(number);

            if bingo.boards[j].is_complete() {
                completed_board = Some(j);
                last_number = number;
                break;
            }
        }

        if completed_board.is_some() {
            break;
        }
    }

    let first = completed_board.unwrap();
    let score = bingo.boards[first].score(last_number);
    println!("day 04: part 1 = {}", score);
}

pub fn part2(input_file: &str) {
    let mut bingo = Bingo::new(input_file);
    let n = bingo.draws.len();
    let b = bingo.boards.len();
    let mut completed_board: Option<usize> = None;
    let mut last_number: u32 = 0;

    for i in 0..n {
        let number = bingo.draws[i];
        for j in 0..b {
            if bingo.boards[j].is_complete() {
                continue;
            }

            bingo.boards[j].mark(number);

            if bingo.boards[j].is_complete() {
                completed_board = Some(j);
                last_number = number;
            }
        }
    }
    let last = completed_board.unwrap();
    let score = bingo.boards[last].score(last_number);
    println!("day 04: part 2 = {}", score);
}

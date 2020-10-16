// Based on https://github.com/jamesmcm/scala-sudoku-solver
//
// rust_sudoku_solver is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// rust_sudoku_solver is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with rust_sudoku_solver.  If not, see <http://www.gnu.org/licenses/>.

use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::process;

type Board = Vec<Vec<u8>>;

fn main() {
    let mut args = env::args();

    if args.len() < 2 {
        eprintln!("Missing filename argument");
        process::exit(1);
    }

    let _program_name = args.next().unwrap();
    let filename = args.next().unwrap();

    let board = load_sudoku(filename.as_str());

    println!("{}", solve_sudoku(board));
}

fn solve_sudoku(board: Board) -> String {
    match recurse(board, 0, 0) {
        None => "No valid solution".to_string(),
        Some(solution) => format_solution(solution),
    }
}

fn get_box_bounds(row: usize, col: usize) -> (usize, usize) {
    ((row / 3) * 3, (col / 3) * 3)
}

fn is_valid_move(board: &Board, row: usize, col: usize, candidate: u8) -> bool {
    // check row
    for i in 0..9 {
        if board[i][col] == candidate {
            return false;
        }
    }

    // check column
    for i in 0..9 {
        if board[row][i] == candidate {
            return false;
        }
    }

    // check box
    let (row, col) = get_box_bounds(row, col);
    for i in row..(row + 3) {
        for j in col..(col + 3) {
            if board[i][j] == candidate {
                return false;
            }
        }
    }

    true
}

fn bound(row: usize, col: usize) -> (usize, usize) {
    match (row, col) {
        (_, 8) => (row + 1, 0),
        _ => (row, col + 1),
    }
}

fn move_position(board: Board, row: usize, col: usize) -> Option<Board> {
    let (row, col) = bound(row, col);

    match (row, col) {
        (9, _) => Some(board),
        _ => recurse(board, row, col),
    }
}

fn substitute(board: &Board, row: usize, col: usize, candidate: u8) -> Option<Board> {
    let mut board = board.clone();

    board[row][col] = candidate;

    recurse(board, row, col)
}

fn recurse(board: Board, row: usize, col: usize) -> Option<Board> {
    match board[row][col] {
        0 => (1..10 as u8)
            .filter(|i| is_valid_move(&board, row, col, *i))
            .map(|i| substitute(&board, row, col, i))
            .find(|i| i.is_some())
            .flatten(),
        _ => move_position(board, row, col),
    }
}

fn load_sudoku(filename: &str) -> Board {
    let file = match File::open(filename) {
        Ok(file) => file,
        Err(error) => {
            eprintln!("Error opening file `{}`: {:?}", filename, error);
            process::exit(1);
        }
    };

    let mut lines = io::BufReader::new(file).lines();
    let mut board: Board = vec![];

    for i in 0..9 {
        let line = match lines.next() {
            Some(line) => line,
            None => {
                eprintln!("Error reading file: line `{}` missing", i + 1);
                process::exit(1);
            }
        };

        let line = match line {
            Ok(line) => line,
            Err(error) => {
                eprintln!(
                    "Error reading file: line `{}` `{}`",
                    i + 1,
                    error.to_string()
                );
                process::exit(1);
            }
        };

        if line.len() < 9 {
            eprintln!(
                "Error reading file: line `{}` expected 9 column values, found {}",
                i + 1,
                line.len()
            );
            process::exit(1);
        }

        let mut vec: Vec<u8> = vec![];
        let line: Vec<char> = line.chars().collect();

        for j in 0..9 {
            let x: u8 = match line[j].to_digit(10) {
                Some(x) if x >= 1 && x <= 9 => x as u8,
                _ => 0,
            };

            vec.push(x);
        }

        board.push(vec);
    }

    board
}

fn format_solution(solution: Board) -> String {
    let mut formatted = String::new();

    for row in solution.iter() {
        for col in row.iter() {
            formatted = format!("{}{}", formatted, col);
        }

        formatted.push('\n');
    }

    formatted
}

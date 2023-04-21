use std::env;
use std::fs::File;
use std::io;
use std::io::prelude::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut board = vec![0; 81];
    if let Some(path) = args.get(1) {
        println!("{}", path);
        board = read_board_file(path).expect("Failed to read board file");
    } else {
        println!("You must specify a input file.");
        return;
    }
    solve(&mut board);
    print_board(&board);
}

fn read_board_file(path: &String) -> Result<Vec<i32>, io::Error> {
    let mut f = File::open(path)?;
    let mut contents = String::new();
    f.read_to_string(&mut contents)?;
    let mut board = vec![0; 81];
    for (i, c) in contents.chars().enumerate() {
        if i >= 89 {
            break;
        }
        if i % 10 == 9 {
            continue;
        }
        let num = c as i32 - 48;
        if 0 <= num && num <= 9 {
            let index = (i / 10) * 9 + (i % 10);
            board[index] = num;
        }
    }
    return Ok(board);
}

fn solve(board: &mut Vec<i32>) {
    for cell in board.iter_mut() {
        *cell = 1;
    }
}

fn print_board(board: &Vec<i32>) {
    for i in 0..9 {
        for j in 0..9 {
            print!("{}", board[9 * i + j]);
        }
        println!();
    }
}

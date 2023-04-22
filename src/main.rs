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
    println!("Board is valid: {}", is_valid_board(&board));
    if let Some(result) = solve(board) {
        print_board(&result);
    } else {
        println!("failed to fill board");
    }
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

fn solve(board: Vec<i32>) -> Option<Vec<i32>> {
    if is_filled(&board) {
        return Some(board);
    }
    for i in 0..81 {
        if board[i] == 0 {
            for j in 1..=9 {
                let mut _board = board.clone();
                _board[i] = j;
                if !is_valid_board(&_board) {
                    continue;
                }
                match solve(_board) {
                    None => continue,
                    Some(result) => return Some(result)
                }
            }
            return None;
        }
    }
    return None;
}

//TODO: test this function
fn is_valid_board(board: &Vec<i32>) -> bool {
    // check each row
    for i in 0..9 {
        let mut check_v = vec![false; 9];
        for j in 0..9 {
            let index = i * 9 + j;
            if board[index] == 0 {
                continue;
            };
            if check_v[board[index] as usize - 1] {
                return false;
            }
            check_v[board[index] as usize - 1] = true;
        }
    }

    //check each column
    for i in 0..9 {
        let mut check_v = vec![false; 9];
        for j in 0..9 {
            let index = j * 9 + i;
            if board[index] == 0 {
                continue;
            }
            if check_v[board[index] as usize - 1] {
                return false;
            }
            check_v[board[index] as usize - 1] = true;
        }
    }

    //check each 3x3 box
    for i in 0..3 {
        for j in 0..3 {
            let mut check_v = vec![false; 9];
            for k in 0..3 {
                for l in 0..3 {
                    let index = 27 * i + 3 * j + 9 * k + l;
                    if board[index] == 0 {
                        continue;
                    }
                    if check_v[board[index] as usize - 1] {
                        return false;
                    }
                    check_v[board[index] as usize - 1] = true;
                }
            }
        }
    }
    return true;
}

fn is_filled(board: &Vec<i32>) -> bool{
    for cell in board {
        if *cell == 0 {
            return false
        }
    }
    return true;
}

fn print_board(board: &Vec<i32>) {
    for i in 0..9 {
        for j in 0..9 {
            print!("{}", board[9 * i + j]);
        }
        println!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn check_valid_board() {
        for i in 1..=5 {
            let path = format!("test_cases/valid_board{}.txt", i);
            let board = read_board_file(&path).expect("failed to read a board file.");
            assert!(is_valid_board(&board));
        }
    }

    #[test]
    fn check_wrong_board() {
        for i in 1..=5 {
            let path = format!("test_cases/wrong_board{}.txt", i);
            let board = read_board_file(&path).expect("failed to read a board file.");
            assert_eq!(is_valid_board(&board), false);
        }
    }

    #[test]
    fn check_is_filled() {
        let path = String::from("test_cases/valid_full_board1.txt");
        let board = read_board_file(&path).expect("failed to read a board file.");
        assert!(is_filled(&board));

        let path = String::from("test_cases/valid_board1.txt");
        let board = read_board_file(&path).expect("failed to read a board file.");
        assert_eq!(is_filled(&board), false);
    }
}

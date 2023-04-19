fn main() {
    let mut board = vec![0; 81];
    solve(&mut board);
    print_board(&board);
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

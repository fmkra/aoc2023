use std::fs;

fn calc_board(board: Vec<String>) -> i32 {
    let rows = board.len() as i32;
    for i in 1..rows {
        let mut no_wrong = 0;
        for j in 0..rows {
            let top = i-1-j;
            let bottom = i+j;
            if top < 0 || bottom >=rows {
                break;
            }
            for c in  board[top as usize].chars().zip(board[bottom as usize].chars()) {
                if c.0 != c.1 {
                    no_wrong += 1;
                }
            }
        }
        if no_wrong == 1 {
            return 100 * i;
        }
    }
    let cols = board[0].len() as i32;
    for i in 1..cols {
        let mut no_wrong = 0;
        for j in 0..cols {
            let left = i-1-j;
            let right = i+j;
            if left < 0 || right >=cols {
                break;
            }
            for c in  board.iter().map(|row| row.chars().nth(left as usize).unwrap()).zip(board.iter().map(|row| row.chars().nth(right as usize).unwrap())) {
                if c.0 != c.1 {
                    no_wrong += 1;
                }
            }
        }
        if no_wrong == 1 {
            return i as i32;
        }
    }
    0
}

fn main() {
    let file = fs::read_to_string("data.txt").expect("Unable to read file");

    let ans: i32 = file
        .split("\n\n")
        .map(|board| board.split("\n").map(|line| line.to_string()).collect::<Vec<_>>())
        .map(calc_board)
        .sum();

    println!("{}", ans);
}

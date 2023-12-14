use std::fs;

fn get_tilted(board: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let height = board.len();
    let width = board[0].len();
    let mut result = vec![vec![' '; width]; height];
    for col in 0..width {
        let mut first_o = 0;
        let mut o_count = vec![0; height];
        for row in 0..height {
            match board[row][col] {
                '#' => {
                    first_o = row + 1;
                },
                'O' => {
                    o_count[first_o] += 1;
                },
                _ => (),
            };
        }
        let mut len = 0;
        for row in 0..height {
            len += o_count[row];
            result[row][col] = match board[row][col] {
                '#' => '#',
                _ => {
                    if len > 0 {
                        len -= 1;
                        'O'
                    } else {
                        '.'
                    }
                },
            };
        }
    }
    result
}

fn get_score(board: &Vec<Vec<char>>) -> i32 {
    let mut score = 0;
    for (i,row) in board.iter().rev().enumerate() {
        for col in row {
            if *col == 'O' {
                score += (i + 1) as i32;
            }
        }
    }
    score
}

fn get_rotated(board: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let height = board.len();
    let width = board[0].len();
    let mut result = vec![vec![' '; height]; width];
    for row in 0..height {
        for col in 0..width {
            result[col][row] = board[height - row - 1][col];
        }
    }
    result
}

fn get_cycle(board: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let board = get_rotated(&get_tilted(&board));
    let board = get_rotated(&get_tilted(&board));
    let board = get_rotated(&get_tilted(&board));
    get_rotated(&get_tilted(&board))
}

fn print_board(board: &Vec<Vec<char>>) {
    for row in board {
        for col in row {
            print!("{}", col);
        }
        println!();
    }
}

fn main() {
    let mut board = fs::read_to_string("data.txt")
        .expect("Error reading file")
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    for _ in 0..1000 {
        board = get_cycle(&board);
        println!("{}", get_score(&board));
    }

}

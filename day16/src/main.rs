use std::fs;

#[derive(Copy, Clone)]
enum Direction {
    Up = 0,
    Right = 1,
    Down = 2,
    Left = 3,
}

impl Direction {
    fn get_row_shift(&self) -> i32 {
        match self {
            Direction::Up => -1,
            Direction::Down => 1,
            _ => 0,
        }
    }

    fn get_col_shift(&self) -> i32 {
        match self {
            Direction::Left => -1,
            Direction::Right => 1,
            _ => 0,
        }
    }

    fn reflect_left_mirror(&self) -> Direction {
        match self {
            Direction::Up => Direction::Left,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Up,
        }
    }

    fn reflect_right_mirror(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Up,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Down,
        }
    }

    fn is_vertical(&self) -> bool {
        match self {
            Direction::Up => true,
            Direction::Down => true,
            _ => false,
        }
    }
}

#[derive(Clone, PartialEq)]
struct Tile {
    type_: char,
    visited: [bool; 4],
}

fn dfs(board: &mut Vec<Vec<Tile>>, _r: i32, _c: i32, dir: Direction) {
    if _r < 0 || _r >= board.len() as i32 || _c < 0 || _c >= board[0].len() as i32 {
        return;
    }
    let (r,c) = (_r as usize, _c as usize);
    let before = board[r][c].clone();
    match board[r][c].type_ {
        '.' => {
            board[r][c].visited[dir as usize] = true;
            board[r][c].visited[dir as usize^2] = true;
            if before != board[r][c] {
                dfs(board, _r + dir.get_row_shift(), _c + dir.get_col_shift(), dir);
            }
        },
        '\\' => {
            board[r][c].visited[dir as usize^2] = true;
            board[r][c].visited[dir as usize^3] = true;
            if before != board[r][c] {
                dfs(board, _r + dir.reflect_left_mirror().get_row_shift(), _c + dir.reflect_left_mirror().get_col_shift(), dir.reflect_left_mirror());
            }
        },
        '/' => {
            board[r][c].visited[dir as usize^2] = true;
            board[r][c].visited[dir as usize^1] = true;
            if before != board[r][c] {
                dfs(board, _r + dir.reflect_right_mirror().get_row_shift(), _c + dir.reflect_right_mirror().get_col_shift(), dir.reflect_right_mirror());
            }
        },
        '|' | '-' => {
            if dir.is_vertical() == (board[r][c].type_ == '|') {
                board[r][c].visited[dir as usize] = true;
                board[r][c].visited[dir as usize^2] = true;
                if before != board[r][c] {
                    dfs(board, _r + dir.get_row_shift(), _c + dir.get_col_shift(), dir);
                }
            } else {
                board[r][c].visited[dir as usize^2] = true;
                board[r][c].visited[dir as usize^3] = true;
                board[r][c].visited[dir as usize^1] = true;
                if before != board[r][c] {
                    dfs(board, _r + dir.reflect_left_mirror().get_row_shift(), _c + dir.reflect_left_mirror().get_col_shift(), dir.reflect_left_mirror());
                    dfs(board, _r + dir.reflect_right_mirror().get_row_shift(), _c + dir.reflect_right_mirror().get_col_shift(), dir.reflect_right_mirror());
                }
            }
        }
        _ => {},
    };
}

fn get_ans(board: &Vec<Vec<Tile>>, r: i32, c: i32, dir: Direction) -> u64 {
    let mut new_board = board.clone();
    dfs(&mut new_board, r, c, dir);

    let mut count: u64 = 0;
    for row in new_board.iter() {
        for tile in row.iter() {
            if tile.visited[0] || tile.visited[1] || tile.visited[2] || tile.visited[3] {
                count += 1;
            }
        }
    }
    count
}

fn main() {
    let mut board: Vec<Vec<Tile>> = Vec::new();

    let file = fs::read_to_string("data.txt").expect("Unable to read file");

    for line in file.lines() {
        let mut row: Vec<Tile> = Vec::new();
        for c in line.chars() {
            row.push(Tile{type_: c, visited: [false; 4]});
        }
        board.push(row);
    }

    let last_row = board.len() - 1;
    let last_col = board[0].len() - 1;
    
    let mut vec: Vec<u64> = Vec::new();
    for i in 0..=last_col {
        vec.push(get_ans(&board, 0, i as i32, Direction::Down));
        vec.push(get_ans(&board, last_row as i32, i as i32, Direction::Up));
    }
    for i in 0..=last_row {
        vec.push(get_ans(&board, i as i32, 0, Direction::Right));
        vec.push(get_ans(&board, i as i32, last_col as i32, Direction::Left));
    }

    let ans = vec.iter().max().unwrap();

    println!("{}", ans);
}

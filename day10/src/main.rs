use std::fs;
use std::thread;


#[derive(Clone, Copy)]
struct Connection {
    top: bool,
    bottom: bool,
    left: bool,
    right: bool,
}

impl Connection {
    fn new() -> Connection {
        Connection {
            top: false,
            bottom: false,
            left: false,
            right: false,
        }
    }

    fn is_empty(&self) -> bool {
        !self.top && !self.bottom && !self.left && !self.right
    }

    fn from_char(c: char) -> Connection {
        match c {
            '.' => Connection::new(),
            '-' => Connection {
                top: false,
                bottom: false,
                left: true,
                right: true,
            },
            '|' => Connection {
                top: true,
                bottom: true,
                left: false,
                right: false,
            },
            'L' => Connection {
                top: true,
                bottom: false,
                left: false,
                right: true,
            },
            'J' => Connection {
                top: true,
                bottom: false,
                left: true,
                right: false,
            },
            '7' => Connection {
                top: false,
                bottom: true,
                left: true,
                right: false,
            },
            'F' => Connection {
                top: false,
                bottom: true,
                left: false,
                right: true,
            },
            'S' => Connection {
                top: true,
                bottom: true,
                left: true,
                right: true,
            },
            _ => panic!("Unknown character"),
        }
    }
    fn to_char(&self) -> char {
        match (self.top, self.bottom, self.left, self.right) {
            (false, false, false, false) => '.',
            (false, false, true, true) => '-',
            (true, true, false, false) => '|',
            (true, false, false, true) => 'L',
            (true, false, true, false) => 'J',
            (false, true, true, false) => '7',
            (false, true, false, true) => 'F',
            (true, true, true, true) => 'S',
            _ => panic!("Unknown connection"),
        }
    }
}

fn dfs(grid: &Vec<Vec<Connection>>, pos: (usize, usize), prev: Option<(usize, usize)>, end: (usize, usize), depth: usize, board: &mut Vec<Vec<Connection>>) -> Option<usize> {
    if pos == end && prev.is_some() {
        return Option::Some(depth);
    }
    board[pos.0][pos.1] = grid[pos.0][pos.1];
    if pos.0 > 0 && prev != Option::Some((pos.0 - 1, pos.1)) && grid[pos.0][pos.1].top && grid[pos.0 - 1][pos.1].bottom {
        match dfs(grid, (pos.0 - 1, pos.1), Option::Some(pos), end, depth + 1, board) {
            Option::Some(depth) => return Option::Some(depth),
            Option::None => (),
        }
    }
    if pos.0 < grid.len() - 1 && prev != Option::Some((pos.0 + 1, pos.1)) && grid[pos.0][pos.1].bottom && grid[pos.0 + 1][pos.1].top {
        match dfs(grid, (pos.0 + 1, pos.1), Option::Some(pos), end, depth + 1, board) {
            Option::Some(depth) => return Option::Some(depth),
            Option::None => (),
        }
    }
    if pos.1 > 0 && prev != Option::Some((pos.0, pos.1 - 1)) && grid[pos.0][pos.1].left && grid[pos.0][pos.1 - 1].right {
        match dfs(grid, (pos.0, pos.1 - 1), Option::Some(pos), end, depth + 1, board) {
            Option::Some(depth) => return Option::Some(depth),
            Option::None => (),
        }
    }
    if pos.1 < grid[0].len() - 1 && prev != Option::Some((pos.0, pos.1 + 1)) && grid[pos.0][pos.1].right && grid[pos.0][pos.1 + 1].left {
        match dfs(grid, (pos.0, pos.1 + 1), Option::Some(pos), end, depth + 1, board) {
            Option::Some(depth) => return Option::Some(depth),
            Option::None => (),
        }
    }
    Option::None
}

fn main() {
    let builder = thread::Builder::new().stack_size(256 * 1024 * 1024);
    let handler = builder.spawn(|| {
        let file = fs::read_to_string("data.txt").expect("Unable to read file");
        
        let mut start: Option<(usize, usize)> = Option::None;
        let grid = file.lines().enumerate().map(|(row, line)| {
            line.chars().enumerate().map(|(col, char)| {
                if char == 'S' {
                    start = Option::Some((row, col));
                }
                Connection::from_char(char)
            }).collect::<Vec<Connection>>()
        }).collect::<Vec<Vec<Connection>>>();

        let start = start.expect("No start found");
        let mut board = vec![vec![Connection::new(); grid[0].len()]; grid.len()];
        let res = dfs(&grid, start, Option::None, start, 0, &mut board);
        println!("{:?}", res.unwrap()/2);

        if grid[start.0+1][start.1].top == false {
            board[start.0][start.1].bottom = false;
        } 
        if grid[start.0-1][start.1].bottom == false {
            board[start.0][start.1].top = false;
        }
        if grid[start.0][start.1+1].left == false {
            board[start.0][start.1].right = false;
        }
        if grid[start.0][start.1-1].right == false {
            board[start.0][start.1].left = false;
        }
        
        // println!("{}", res.unwrap()/2);
        // for row in board.iter() {
        //     for col in row.iter() {
        //         print!("{}", col.to_char());
        //     }
        //     println!();
        // }

        let mut dots = 0;
        for row in board.iter() {
            let mut sum_top = 0;
            let mut sum_bot = 0;
            let mut is_open = false;
            for col in row.iter() {
                if !col.is_empty() {
                    if col.top {
                        sum_top += 1;
                    }
                    if col.bottom {
                        sum_bot += 1;
                    }
                    if sum_top == 1 && sum_bot == 1 {
                        sum_top = 0;
                        sum_bot = 0;
                        is_open = !is_open;
                    } else if sum_top == 2 {
                        sum_top = 0;
                    } else if sum_bot == 2 {
                        sum_bot = 0;
                    } else if sum_top + sum_bot > 1 {
                        panic!("Invalid board {} {}", sum_top, sum_bot);
                    }
                    print!("{}", col.to_char());
                } else {
                    if is_open {
                        dots += 1;
                        print!(".");
                    } else {
                        print!(" ");
                    }
                }
            }
            println!();
        }
        println!("{}", dots);
    }).unwrap();

    handler.join().unwrap();
}

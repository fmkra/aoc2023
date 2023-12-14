use std::fs;

fn f(board: Vec<char>, start: usize, pattern: Vec<usize>, acc: &mut u64) {
    // let board_str = board.clone().into_iter().collect::<String>();
    // println!("f({}, {}, {:?})", board_str, start, pattern);

    if start >= board.len() {
        if pattern.len() == 0 {
            *acc += 1;
            // println!("  ans = {}", board_str);
        } else {
            // println!("no ans {}, because pattern is left {:?}", board_str, pattern);
        }
        return;
    }
    if pattern.len() == 0 {
        let mut has_hash = false;
        for i in start..board.len() {
            if board[i] == '#' {
                has_hash = true;
                break;
            }
        }
        if !has_hash {
            *acc += 1;
            // println!("  ans = {}", board_str);
            return;
        }
    }
    if board[start] == '.' {
        return f(board, start + 1, pattern, acc);
    }
    if pattern.len() > 0 {
        if board.len() < start + pattern[0] {
            return;
        }

        let at_most_on_right = board[start..].iter().fold(0, |acc, x| if *x != '.' { acc + 1 } else { acc });
        if at_most_on_right < pattern.iter().sum::<usize>() {
            return;
        }
        // let at_least_on_right = board[start..].iter().fold(0, |acc, x| if *x == '#' { acc + 1 } else { acc });
        // if at_least_on_right > pattern.iter().sum::<usize>() {
        //     return;
        // }

        // println!("f({}, {}, {:?}) loop length is {}", board.clone().into_iter().collect::<String>(), start, pattern, (board.len() - start - pattern[0] + 1));
        for left_offset in 0..(board.len() - start - pattern[0] + 1) {
            let mut matches = true;
            for i in 0..left_offset {
                if board[start + i] == '#' {
                    matches = false;
                    break;
                }
            }
            for i in 0..pattern[0] {
                if board[start + left_offset + i] == '.' {
                    matches = false;
                    break;
                }
            }
            if board.get(start + left_offset + pattern[0]) == Option::Some(&'#') {
                matches = false;
            }
            // println!("f({}, {}, {:?}) {} {:?}", board.clone().into_iter().collect::<String>(), start, pattern, left_offset, matches);
            if matches {
                let mut new_board = board.clone();
                for i in 0..left_offset {
                    new_board[start + i] = '.';
                }
                for i in 0..pattern[0] {
                    new_board[start + left_offset + i] = '#';
                }
                if start + left_offset + pattern[0] < new_board.len() {
                    new_board[start + left_offset + pattern[0]] = '.';
                }
                f(new_board, start + pattern[0] + left_offset, pattern[1..].to_vec(), acc);
            }
        }
    }
}

fn get_times_5(input: String, join: String) -> String {
    let mut acc = String::new();
    for i in 0..5 {
        if i != 0 {
            acc.push_str(&join);
        }
        acc.push_str(&input);
    }
    acc
}

fn count_patterns(input: String) -> u64 {
    let mut splited = input.split(" ");
    let board = splited.next().unwrap();
    let pattern_str = splited.next().unwrap();
    
    let board = get_times_5(board.to_string(), "?".to_string());
    let pattern_str = get_times_5(pattern_str.to_string(), ",".to_string());
    
    let pattern = pattern_str.split(",").map(|x| x.parse::<usize>().unwrap()).collect::<Vec<usize>>();
    let mut acc = 0;
    f(board.chars().collect(), 0, pattern, &mut acc);
    acc
}

fn main() {
    let file = fs::read_to_string("data.txt").expect("Unable to read file");

    let mut sum: u64 = 0;
    file.lines().map(|line| count_patterns(line.to_string())).enumerate().for_each(|(i,x)| {
        println!("{} {}", i, x);
        sum += x;
    });

    println!("sum = {}", sum);
}

use std::fs;
use std::io::Write;
use rayon::prelude::*;
use std::collections::HashMap;
use cached::proc_macro::cached;

#[cached]
fn f(board: Vec<char>, pattern: Vec<usize>) -> u64 {
    // let board_str = board.clone().into_iter().collect::<String>();
    // println!("f({}, {}, {:?})", board_str, start, pattern);

    if 0 >= board.len() {
        if pattern.len() == 0 {
            return 1;
            // println!("  ans = {}", board_str);
        } else {
            // println!("no ans {}, because pattern is left {:?}", board_str, pattern);
            return 0;
        }
    }
    if pattern.len() == 0 {
        let mut has_hash = false;
        for i in 0..board.len() {
            if board[i] == '#' {
                has_hash = true;
                break;
            }
        }
        if !has_hash {
            // println!("  ans = {}", board_str);
            return 1;
        }
    }
    if board[0] == '.' {
        return f(board[1..].to_vec(), pattern);
    }
    let mut acc = 0;
    if pattern.len() > 0 {
        if board.len() < pattern[0] {
            return 0;
        }

        // let at_most_on_right = board[start..].iter().fold(0, |acc, x| if *x != '.' { acc + 1 } else { acc });
        // if at_most_on_right < pattern.iter().sum::<usize>() {
        //     return;
        // }
        // let at_least_on_right = board[start..].iter().fold(0, |acc, x| if *x == '#' { acc + 1 } else { acc });
        // if at_least_on_right > pattern.iter().sum::<usize>() {
        //     return;
        // }

        // println!("f({}, {}, {:?}) loop length is {}", board.clone().into_iter().collect::<String>(), start, pattern, (board.len() - start - pattern[0] + 1));
        for left_offset in 0..(board.len() - pattern[0] + 1) {
            let mut matches = true;
            for i in 0..left_offset {
                if board[i] == '#' {
                    matches = false;
                    break;
                }
            }
            for i in 0..pattern[0] {
                if board[left_offset + i] == '.' {
                    matches = false;
                    break;
                }
            }
            if board.get(left_offset + pattern[0]) == Option::Some(&'#') {
                matches = false;
            }
            // println!("f({}, {}, {:?}) {} {:?}", board.clone().into_iter().collect::<String>(), start, pattern, left_offset, matches);
            if matches {
                let mut new_board = board.clone();
                for i in 0..left_offset {
                    new_board[i] = '.';
                }
                for i in 0..pattern[0] {
                    new_board[left_offset + i] = '#';
                }
                if left_offset + pattern[0] < new_board.len() {
                    new_board[left_offset + pattern[0]] = '.';
                }
                acc += f(new_board[pattern[0] + left_offset..].to_vec(), pattern[1..].to_vec());
            }
        }
    }
    acc
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
    let acc = f(board.chars().collect(), pattern);
    println!("{} = {}", input, acc);
    acc
}

fn main() {
    main1();
}

fn main1() {
    let file = fs::read_to_string("data.txt").expect("Unable to read file");

    let sum: u64 = file.lines() // .filter(|line| (&line).find("=").is_none()).collect::<Vec<&str>>().par_iter()
        .map(|line| count_patterns(line.to_string()))
        .sum();

    println!("sum = {}", sum);
}

fn main2() {
    let partial_result = fs::read_to_string("partial results.txt").expect("Unable to read partial results file");

    let results = fs::read_to_string("results.txt").expect("Unable to read results file");

    let mut map_results = HashMap::new();
    partial_result.lines().for_each(|line| {
        let mut splited = line.split(" = ");
        let key = splited.next().unwrap();
        let value = splited.next().unwrap();
        map_results.insert(key.to_string(), value.to_string());
    });

    let results = results.lines().map(|line| {
        let mapped = map_results.get(line);
        if mapped.is_some() {
            line.to_string() + " = " + mapped.unwrap()
        } else {
            line.to_string()
        }
    });

    // write to file

    let mut file = fs::File::create("results.txt").expect("Unable to create file");
    let mut x = false;
    for result in results {
        if x {
            file.write_all("\n".as_bytes()).expect("Unable to write data");
        } else {
            x = true;
        }
        file.write_all(result.as_bytes()).expect("Unable to write data");
    }
}
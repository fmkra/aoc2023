use std::fs;


fn main() {
    let board: Vec<String> = fs::read_to_string("data.txt")
        .expect("Unable to read file")
        .lines()
        .map(|line| line.to_string())
        .collect();

    let mut numbers: Vec<(usize, usize, usize, u64)> = Vec::new(); // (y, x1, x2, acc)
    let rows = board.len();
    let cols = board[0].len();
    numbers.push((rows, 0, 0, 0));
    let mut adj: Vec<Vec<u64>> = vec![Vec::new(); cols*rows];

    for r in 0..rows {
        for c in 0..cols {
            let v = board[r].chars().nth(c).unwrap();
            if !v.is_digit(10) {
                continue;
            }
            let last = numbers.last().unwrap().clone();
            if last.0 == r && last.2 == c-1 {
                numbers.pop();
                numbers.push((r, last.1, c, last.3 * 10 + v.to_digit(10).unwrap() as u64));
            } else {
                numbers.push((r, c, c, v.to_digit(10).unwrap() as u64));
            }
        }
    }
    let part1ans: u64 = numbers[1..].iter().map(|(r, c1, c2, number)| {
        let mut part1res = 0;
        for cc in (if *c1 == 0 {0} else {*c1-1})..=(*c2 + 1) {
            for cr in (if *r == 0 {0} else {*r-1})..=(*r + 1) {
                if cr == *r && cc >= *c1 && cc <= *c2 || cc == cols || cr == rows {
                    continue;
                }
                let c = board[cr].chars().nth(cc).unwrap();
                if c != '.' {
                    part1res = *number;
                }
                if c == '*' {
                    adj[cr*cols+cc].push(*number);
                }
            }
        }
        return part1res;
    }).sum();

    let mut part2sum: u64 = 0;
    for x in adj {
        if x.len() == 2 {
            part2sum += x[0] * x[1];
        }
    }
    
    println!("{}", part1ans);

    println!("{}", part2sum);
}

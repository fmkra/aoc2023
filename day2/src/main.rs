use std::fs;
use std::collections::HashMap;

fn part1(str: String) -> u32 {
    str.lines()
        .map(|line| line.replace(",", "").replace(";","").replace(":",""))
        .map(|line| {
            let cols: Vec<_> = line.split_whitespace().collect();
            let game_num = cols[1].parse::<u32>().unwrap();
            if cols[2..].chunks(2).all(|col| {
                let (count, color) = (col[0], col[1]);
                match color {
                    "red" => count.parse::<u32>().unwrap() <= 12,
                    "green" => count.parse::<u32>().unwrap() <= 13,
                    "blue" => count.parse::<u32>().unwrap() <= 14,
                    _ => false
                }
            }) { game_num } else { 0 }
        })
        .sum::<u32>()
}

fn part2(str: String) -> u64 {
    str.lines()
        .map(|line| line.replace(",", "").replace(";","").replace(":",""))
        .map(|line| {
            let cols: Vec<_> = line.split_whitespace().collect();
            let game_num = cols[1].parse::<u64>().unwrap();
            let mut max: HashMap<&str, u64> = HashMap::new();
            cols[2..].chunks(2).for_each(|col| {
                let (count, color) = (col[0].parse::<u64>().unwrap(), col[1]);
                if let Some(&val) = max.get(color) {
                    if count > val {
                        max.insert(color, count);
                    }
                } else {
                    max.insert(color, count);
                }
            });
            max.values().fold(1, |acc, &x|acc * x)
        })
        .sum::<u64>()
}

fn main() {
    let file = fs::read_to_string("data.txt").expect("Couldn't open foo.txt");
    
    println!("{}", part1(file));
    println!("{}", part2(file));
}

use std::fs;
use std::collections::HashSet;

fn main() {
    let file = fs::read_to_string("data.txt").expect("Unable to read file");

    let mut no_matches: Vec<u64> = Vec::new();
    let mut no_cards: Vec<u64> = Vec::new();
    let mut sum = 0;
    let mut sum_part2 = 0;
    for line in file.lines() {
        let data = line
            .split(":")
            .last()
            .unwrap()
            .split('|')
            .map(|word| word
                .split_whitespace()
                .map(|x| x.parse::<u32>().unwrap())
                .collect::<Vec<_>>()
            )
            .collect::<Vec<_>>();
        
        let mut winning = HashSet::new();
        for i in 0..data[0].len() {
            winning.insert(data[0][i]);
        }
        let mut count: u32 = 0;
        for i in 0..data[1].len() {
            if winning.contains(&data[1][i]) {
                count += 1;
            }
        }
        let points = if count == 0 { 0 } else { 2_u64.pow(count - 1) };
        no_matches.push(count.into());
        no_cards.push(1);
        sum += points;
    }
    println!("{}", sum);

    for i in 0..no_cards.len() {
        let nc = no_cards[i];
        let nm = no_matches[i];
        for j in 1..=nm {
            no_cards[(j as usize)+i] += nc; 
        }
        sum_part2 += no_cards[i];
    }
    println!("{}", sum_part2);
}

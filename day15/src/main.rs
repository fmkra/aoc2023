use std::fs;

fn hash(input: &String) -> u64 {
    let mut hash: u8 = 0;
    for c in input.chars() {
        hash = hash.wrapping_add( c as u8).wrapping_mul(17);
    }
    hash as u64
}

fn main() {
    let file = fs::read_to_string("data.txt").expect("Unable to read file");
    let str = file.lines().collect::<Vec<_>>().join(",");

    let sum = str.split(",").map(|x| hash(&x.to_string())).sum::<u64>();
    println!("part1: {}", sum);


    let mut boxes: Vec<Vec<String>> = std::iter::repeat(vec![]).take(256).collect();
    file.split(",").for_each(|x| {
        let s = x.to_string();
        if s.ends_with("-") {
            let label = s[0..(s.len() - 1)].to_string();
            let hash = hash(&label);
            boxes[hash as usize].retain(|e| e[..(e.len() - 2)] != label);
        } else {
            let label = s[0..(s.len() - 2)].to_string();
            let hash = hash(&label);
            let number = s.chars().last().unwrap();
            let mut was_replaced = false;
            boxes[hash as usize].iter_mut().for_each(|e| {
                if e[..(e.len() - 2)] == label {
                    *e = s.clone();
                    was_replaced = true;
                }
            });
            if !was_replaced {
                boxes[hash as usize].push(s.clone());
            }
        }
    });
    let ans = boxes.iter().enumerate().map(|(i, box_)| {
        (i+1) * box_.iter().enumerate().map(|(slot,fl)| (slot+1) * (*fl).chars().last().unwrap().to_digit(10).unwrap() as usize).sum::<usize>()
    }).sum::<usize>();

    println!("part2: {}", ans);
}
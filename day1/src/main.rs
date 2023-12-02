use std::fs;
use std::cmp;


fn main() {
    let numbers = vec![
        "one".to_string(),
        "two".to_string(),
        "three".to_string(),
        "four".to_string(),
        "five".to_string(),
        "six".to_string(),
        "seven".to_string(),
        "eight".to_string(),
        "nine".to_string(),
    ];

    let file = fs::read_to_string("data.txt")
        .expect("Unable to read file");

    let mut sum: usize = 0;
    for line in file.lines() {
        let first = numbers
            .iter()
            .enumerate()
            .map(|(i, n)| cmp::min(
                line.find(n).unwrap_or(100),
                line.find(&(i+1).to_string()).unwrap_or(100)
            ))
            .enumerate()
            .min_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
            .map(|(index, _)| index).unwrap() + 1;

        let last = numbers
            .iter()
            .enumerate()
            .map(|(i, n)| cmp::max(
                line.rfind(n).map(|x| x+1).unwrap_or(0), 
                line.rfind(&(i+1).to_string()).map(|x| x+1).unwrap_or(0)
            ))
            .enumerate()
            .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
            .map(|(index, _)| index).unwrap() + 1;
        sum += 10 * first + last;
    }
    println!("{}", sum);
}

use std::fs;
use std::collections::HashMap;


fn parse_input(input: &str) -> (&str, &str, &str) {
    let parts: Vec<&str> = input.split('=').collect();

    let first_part = parts[0].trim();
    let second_part = parts[1].trim().trim_start_matches('(').trim_end_matches(')');
    
    let values: Vec<&str> = second_part.split(',').map(str::trim).collect();

    return (first_part, values[0], values[1]);
}

fn main() {
    let file = fs::read_to_string("data.txt").expect("Unable to read file");
    let mut lines = file.lines();

    let instructions = lines.next().unwrap().chars().cycle();

    let graph = lines
        .skip(1)
        .map(parse_input)
        .fold(HashMap::new(), |mut acc, (key, value1, value2)| {
            acc.insert(key, (value1, value2));
            acc
        });
    
    let mut current_position = "AAA";
    let mut count = 0;
    for m in instructions {
        if m == 'L' {
            current_position = graph.get(current_position).unwrap().0;
        }
        if m == 'R' {
            current_position = graph.get(current_position).unwrap().1;
        }
        count += 1;
        if current_position == "ZZZ" {
            break;
        }
    }
    println!("{}", count);
}

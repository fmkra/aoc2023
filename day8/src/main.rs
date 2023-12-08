use std::fs;
use std::collections::HashMap;
use num::integer::lcm;


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
            acc.insert(key.to_string(), (value1.to_string(), value2.to_string()));
            acc
        });
    
    let mut positions = graph.keys().filter(|x| x.ends_with("A")).collect::<Vec<&String>>();
    let mut steps = (0..positions.len()).map(|_| 0).collect::<Vec<u64>>();
    let mut counter = 0;
    for m in instructions {
        counter += 1;
        if counter % 1000000 == 0 {
            println!("{}", counter);
        }
        let mut no_finished = 0;
        for i in 0..positions.len() {
            if positions[i].ends_with("Z") {
                no_finished += 1;
                continue;
            }
            steps[i] += 1;
            if m == 'L' {
                positions[i] = &graph.get(positions[i]).unwrap().0;
            }
            if m == 'R' {
                positions[i] = &graph.get(positions[i]).unwrap().1;
            }
        }
        if no_finished == positions.len() {
            break;
        }
    }
    println!("{}", steps.iter().fold(1, |acc, x| lcm(acc, *x)));
}

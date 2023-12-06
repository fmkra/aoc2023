use std::fs;

fn main() {
    let file = fs::read_to_string("part2.txt").expect("Unable to read file");
    let mut iter = file.lines();

    let time = iter.next().unwrap().split_whitespace().skip(1).map(|x| x.parse::<i64>().unwrap());
    let dist = iter.next().unwrap().split_whitespace().skip(1).map(|x| x.parse::<i64>().unwrap());

    let mut product = 1;
    for (t, d) in time.zip(dist) {
        let mut count = 0;
        for charge_time in 1..t {
            let distance = charge_time * (t - charge_time);
            if distance > d {
                count += 1;
            }
        }
        product *= count;
    }
    println!("{}", product);
}

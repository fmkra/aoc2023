use std::fs;
pub mod range;

fn main() {
    let input = fs::read_to_string("data.txt").expect("Unable to read file");

    let mut lines = input.split("\n\n");

    let init: Vec<_> = lines.next().unwrap().split_whitespace().skip(1).map(|x| x.parse::<i64>().unwrap()).collect();

    let mut rng = range::Range::new();
    init.chunks(2).for_each(|chunk| {
        let (start, len) = (chunk[0], chunk[1]);
        rng.add((start as i64, (start + len) as i64));
    });

    let maps = lines.map(|m|
        m.split("\n")
            .skip(1)
            .map(|l| l.split_whitespace().map(|x| x.parse::<i64>().unwrap()).collect::<Vec<_>>())
            .map(|x| (x[0], x[1], x[2]))
            .collect::<Vec<_>>()
    );

    maps.for_each(|mapping| {
        rng.shift_many(mapping);
    });

    println!("{}", rng.intervals[0].0);
}

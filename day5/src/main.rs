use std::fs;

fn applay_first(val: u64, mapping: Vec<Vec<u64>>) -> u64 {
    for m in mapping {
        let (dst, src, len) = (m[0], m[1], m[2]);
        if val >= src && val < src + len {
            return dst + (val - src);
        }
    }
    val
}

fn main() {
    let input = fs::read_to_string("data.txt").expect("Unable to read file");

    let mut lines = input.split("\n\n");

    let mut init: Vec<_> = lines.next().unwrap().split_whitespace().skip(1).map(|x| x.parse::<u64>().unwrap()).collect();

    let maps = lines.map(|m|
        m.split("\n")
            .skip(1)
            .map(|l| l.split_whitespace().map(|x| x.parse::<u64>().unwrap()).collect::<Vec<_>>())
            .collect::<Vec<_>>()
    );

    maps.for_each(|mapping| {
        init = init.clone().into_iter().map(|val| applay_first(val, mapping.clone())).collect();
    });
    println!("{}", init.into_iter().min().unwrap());
}

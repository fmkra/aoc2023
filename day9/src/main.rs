use std::fs;


fn get_prediction(seq: Vec<i64>) -> Vec<i64> {
    if seq.iter().all(|x| *x == 0) {
        let mut ret = seq.clone();
        ret.push(0);
        return ret;
    }
    let diffs = seq.windows(2).map(|w| w[1] - w[0]).collect::<Vec<i64>>();
    let diffs_prediction = get_prediction(diffs.clone());
    let mut new_seq = seq.clone();
    let extr = diffs_prediction.last().unwrap() + seq.last().unwrap();
    new_seq.push(extr);
    return new_seq;
}

fn main() {
    let file = fs::read_to_string("data.txt").expect("Unable to read file");

    let sum: i64 = file.lines().map(|line| line.split_whitespace().map(|x| x.parse::<i64>().unwrap()).collect::<Vec<i64>>()).map(|x| {
        let mut x = x.clone();
        x.reverse();
        let pred = get_prediction(x);
        pred.last().unwrap().clone()
    }).sum();

    println!("Sum: {}", sum);
}

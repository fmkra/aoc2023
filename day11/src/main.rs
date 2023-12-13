use std::fs;
use std::cmp::{min, max};

fn main() {
    let factor = 1000000;

    let file = fs::read_to_string("data.txt").expect("Unable to read file");

    let mut map: Vec<Vec<bool>> = vec![];
    let mut col_has_galaxy: Vec<bool> = vec![];
    let mut row_has_galaxy: Vec<bool> = vec![];
    for row in file.lines() {
        let mut map_row = vec![];
        let mut current_row_has_galaxy = false;
        for (col,elem) in row.chars().enumerate() {
            if map.len() == 0 { // first row
                col_has_galaxy.push(false);
            }
            if elem == '#' {
                map_row.push(true);
                col_has_galaxy[col] = true;
                current_row_has_galaxy = true;
            } else {
                map_row.push(false);
            }
        }
        map.push(map_row);
        row_has_galaxy.push(current_row_has_galaxy);
    }

    let mut galaxies: Vec<(i64, i64)> = vec![];
    for (row,elems) in map.iter().enumerate() {
        for (col,elem) in elems.iter().enumerate() {
            if *elem {
                galaxies.push((row as i64, col as i64));
            }
        }
    }

    println!("{:?}\n{:?}", row_has_galaxy, col_has_galaxy);

    let mut sum: i64 = 0;
    for i in 0..galaxies.len() {
        for j in (i+1)..galaxies.len() {
            let g1 = galaxies[i];
            let g2 = galaxies[j];
            
            let mut empty_rows = 0;
            let mut empty_cols = 0;
            for row in min(g1.0,g2.0)..=max(g1.0,g2.0) {
                if !row_has_galaxy[row as usize] {
                    empty_rows += 1;
                }
            }
            for col in min(g1.1,g2.1)..=max(g1.1,g2.1) {
                if !col_has_galaxy[col as usize] {
                    empty_cols += 1;
                }
            }

            sum += (g1.0 - g2.0).abs() + (g1.1 - g2.1).abs() + (empty_rows + empty_cols) * (factor - 1);
        }
    }
    println!("{}", sum);
}

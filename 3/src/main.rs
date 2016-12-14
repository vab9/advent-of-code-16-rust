use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

fn main() {
    let f = File::open("input.txt").unwrap();
    let reader = BufReader::new(f);
    let n = reader.lines()
        .map(|l| {
            l.unwrap()
                .split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect()
        })
        .fold(0, |count, t: Vec<u32>| {
            if t[0] + t[1] > t[2] && t[0] + t[2] > t[1] && t[1] + t[2] > t[0] {
                count + 1
            } else {
                count
            }
        });
    println!("{:?}", n);
}

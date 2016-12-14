use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

fn main() {
    let fu = File::open("input.txt").unwrap();
    let reader = BufReader::new(fu);
    let myvec: Vec<Vec<u32>> = reader.lines()
        .map(|l| {
            l.unwrap()
                .split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect()
        })
        .collect();

    let mut triangles = Vec::new();
    for chunk in myvec.as_slice().chunks(3) {
        triangles.push(vec![chunk[0][0], chunk[1][0], chunk[2][0]]);
        triangles.push(vec![chunk[0][1], chunk[1][1], chunk[2][1]]);
        triangles.push(vec![chunk[0][2], chunk[1][2], chunk[2][2]]);
    }
    let c = triangles.iter().fold(0, |count, t| {
        if t[0] + t[1] > t[2] && t[0] + t[2] > t[1] && t[1] + t[2] > t[0] {
            count + 1
        } else {
            count
        }
    });
    println!("{:?}", c);
}

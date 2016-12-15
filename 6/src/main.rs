use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::collections::BTreeMap;

fn main() {
    let f = File::open("input.txt").unwrap();
    let reader = BufReader::new(f);
    let mut result: Vec<BTreeMap<char, u32>> = vec![BTreeMap::new(); 8];
    for line in reader.lines() {
        let line = line.unwrap();
        for tuple in line.chars().enumerate() {
            let index = tuple.0;
            let c = tuple.1;
            *result[index].entry(c).or_insert(0) += 1;
        }
    }
    // println!("{:?}", result);
    for map in result {
        let mut v: Vec<(char, u32)> = map.into_iter().collect();
        v.sort_by_key(|t| t.1);
        v.reverse();
        print!("{}", &v[0].0);
    }

}

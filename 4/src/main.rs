extern crate regex;

use regex::Regex;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

fn main() {
    let re_words = Regex::new(r"(?P<word>[a-z]+)-").unwrap();
    let re_rest = Regex::new(r"-(?P<id>\d+)\[(?P<checksum>[a-z]+)\]$").unwrap();

    let f = File::open("input.txt").unwrap();
    let reader = BufReader::new(f);


    let mut sum = 0;
    for line in reader.lines() {
        let room = line.unwrap();
        let words: Vec<&str> = re_words.captures_iter(&room)
            .map(|capture| capture.name("word").unwrap())
            .collect();
        let rest = re_rest.captures(&room).unwrap();
        let id: u32 = rest.name("id").unwrap().parse().unwrap();
        let checksum: &str = rest.name("checksum").unwrap();
        // println!("id: {}, checksum: {:?}", id, checksum);
        let most_common: String = five_most_common_chars(&words).into_iter().collect();
        // println!("{:?}", most_common);
        if most_common == checksum {
            sum = sum + id;
        }
    }
    println!("THE SUM OF IDs IS {}", sum);
}

fn five_most_common_chars(arr: &[&str]) -> Vec<char> {
    use std::collections::BTreeMap;
    let mytree: BTreeMap<char, u32> = arr.iter()
        .flat_map(|s| s.chars())
        .filter(|x| !x.is_whitespace())
        .fold(BTreeMap::new(), |mut tree, c| {
            if tree.contains_key(&c) {
                if let Some(entry) = tree.get_mut(&c) {
                    *entry = *entry + 1;
                }
            } else {
                tree.insert(c, 1);
            }
            tree
        });
    let mut vec = Vec::new();
    for (character, count) in mytree {
        vec.push((character, count));
    }
    vec.sort_by(|b, a| {
        if a.1 == b.1 {
            b.0.cmp(&a.0)
        } else {
            a.1.cmp(&b.1)
        }
    });
    vec.iter().map(|t| t.0).take(5).collect()
}

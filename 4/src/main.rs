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

    let mut real_rooms = Vec::new();
    // let mut sum = 0;
    for line in reader.lines() {
        let room = line.unwrap();
        let words: Vec<&str> = re_words.captures_iter(&room)
            .map(|capture| capture.name("word").unwrap())
            .collect();
        let rest = re_rest.captures(&room).unwrap();
        let id: u32 = rest.name("id").unwrap().parse().unwrap();
        let checksum: &str = rest.name("checksum").unwrap();
        let most_common: String = five_most_common_chars(&words).into_iter().collect();
        let offset = (id % 26) as usize;
        if most_common == checksum {
            // sum = sum + id;
            real_rooms.push((caesar(&words, offset), id));

            if caesar(&words, offset).contains("north") {
                println!("{:?}, ID: {}", caesar(&words, offset), id);
            }
        }

        // println!("{:?}", most_common);
        // if most_common == checksum {
        //     sum = sum + id;
        // }
    }
    // println!("THE SUM OF IDs IS {}", sum);
}

fn caesar(input: &[&str], offset: usize) -> String {
    input.iter()
        .map(|s| rotate(s, offset))
        .fold(String::new(), |mut new, s| {
            new.push_str(&s);
            new.push_str(" ");
            new
        })
}

fn rotate(s: &str, offset: usize) -> String {
    let lower = "abcdefghijklmnopqrstuvwxyz";
    s.chars()
        .map(|c| {
            match () {
                _ if lower.contains(c) => {
                    lower.chars()
                        .nth((lower.chars()
                            .position(|x| x == c)
                            .unwrap() + offset) % 26)
                        .unwrap()
                }
                _ => c,
            }
        })
        .collect()
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

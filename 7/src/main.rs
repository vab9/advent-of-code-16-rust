extern crate regex;

use regex::Regex;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

#[cfg(not(test))]
fn main() {
    let re = Regex::new(r"(\[?[a-z]+\]?)").unwrap();
    let f = File::open("input.txt").unwrap();
    let reader = BufReader::new(f);
    let mut count = 0;
    for line in reader.lines() {
        let line = line.unwrap();

        let (reg_seqs, hyper_seqs) = re.find_iter(&line)
            .map(|(begin, end)| &line[begin..end])
            .collect::<Vec<&str>>()
            .iter()
            .partition::<Vec<&str>, _>(|s| !s.starts_with('['));

        if reg_seqs.iter().any(|s| has_abba(s)) && hyper_seqs.iter().all(|s| !has_abba(s)) {
            count += 1;
        }

    }
    println!("Count: {}", count);

}

pub fn has_abba(s: &str) -> bool {
    s.as_bytes()
        .windows(4)
        .any(|w| is_abba(String::from_utf8(w.to_vec()).unwrap().as_str()))
}

pub fn is_abba(s: &str) -> bool {
    s.bytes().nth(0) != s.bytes().nth(1) &&
    s.chars()
        .zip(s.chars().rev())
        .take(s.len() / 2)
        .all(|(a, b)| a == b)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn abba_test() {
        assert!(is_abba("yaay"));
        assert!(is_abba("abba"));
        assert!(!is_abba("xyxy"));
        assert!(!is_abba("aaaa"));
    }

    #[test]
    fn has_abba_test() {
        assert!(has_abba("rhamabbaaeovmbheijj"));
        assert!(has_abba("abbaaaeovmbheijj"));
        assert!(has_abba("rhamabbaaeovmbheabba"));
        assert!(!has_abba("asdfghjkl"));
        assert!(!has_abba("asdfjklpqweruip"));

    }

}

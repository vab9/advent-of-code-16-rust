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

        let (super_seqs, hyper_seqs) = re.find_iter(&line)
            .map(|(begin, end)| &line[begin..end])
            .collect::<Vec<&str>>()
            .iter()
            .partition::<Vec<&str>, _>(|s| !s.starts_with('['));

        let super_abas: Vec<&str> = super_seqs.iter()
            .flat_map(|seq| get_abas(seq))
            .collect();

        let hyper_abas: Vec<&str> = hyper_seqs.iter()
            .flat_map(|seq| get_abas(seq))
            .collect();

        if corresponding_bab_exists(super_abas, hyper_abas) {
            count += 1;
        }

    }
    println!("Count: {}", count);

}

pub fn get_abas(s: &str) -> Vec<&str> {
    s.as_bytes()
        .windows(3)
        .filter_map(|bytes| std::str::from_utf8(bytes).ok())
        .filter(|s| is_aba(s))
        .collect()
}

pub fn is_aba(s: &str) -> bool {
    s.len() == 3 && s.bytes().nth(0) == s.bytes().nth(2) && s.bytes().nth(0) != s.bytes().nth(1)
}

pub fn corresponding_bab_exists(super_seq: Vec<&str>, hyper_seq: Vec<&str>) -> bool {
    super_seq.iter().any(|aba| hyper_seq.iter().any(|s| is_bab(s, aba)))
}

pub fn is_bab(s: &str, aba: &str) -> bool {
    is_aba(aba) && is_aba(s) && s.bytes().nth(0) == aba.bytes().nth(1) &&
    s.bytes().nth(1) == aba.bytes().nth(0)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aba_test() {
        assert!(is_aba("yay"));
        assert!(is_aba("aba"));
        assert!(!is_aba("xxy"));
        assert!(!is_aba("aaa"));
    }

    #[test]
    fn get_abas_test() {
        assert_eq!(get_abas("zazbzxdsfd"), vec!["zaz", "zbz"]);
    }

    #[test]
    fn is_bab_test() {
        assert!(is_bab("bab", "aba"));
        assert!(is_bab("xyx", "yxy"));
        assert!(!is_bab("bbb", "bbb"));
        assert!(!is_bab("aba", "aba"));
        assert!(!is_bab("abc", "bac"));
    }

    #[test]
    fn corresponding_bab_exists_test() {
        assert!(corresponding_bab_exists(vec!["aba", "xyz"], vec!["bab"]));
        assert!(!corresponding_bab_exists(vec!["xyx", "xyx"], vec!["xyx"]));
        assert!(corresponding_bab_exists(vec!["aaa", "eke"], vec!["kek"]));
        assert!(corresponding_bab_exists(vec!["zaz", "zbz"], vec!["bzb"]));
    }
}

extern crate crypto;

use crypto::md5::Md5;
use crypto::digest::Digest;

fn main() {
    let mut hasher = Md5::new();
    let mut v: [Option<char>; 8] = [None; 8];
    let key = "cxdnnyjw";
    for i in 0..std::u64::MAX {
        hasher.input_str(&format!("{}{}", key, i.to_string()));
        let output = hasher.result_str();
        if output.starts_with("00000") {
            let c = output.chars().nth(5);
            if c.is_none() {
                panic!("WALLA");
            }
            let pos = match output.chars().nth(5).unwrap().to_digit(10) {
                Some(p) if p < 8 => p as usize,
                _ => {
                    hasher.reset();
                    continue;
                }
            };
            let val = output.chars().nth(6);
            if val.is_some() && v[pos].is_none() {
                v[pos] = val;
                let s: String = &v.iter()
                    .map(|o| o.unwrap_or('_'))
                    .collect();
                println!("{}", s);
                if v.iter().all(|o| o.is_some()) {
                    break;
                }
            }
        }
        hasher.reset();
    }
}

extern crate crypto;

use crypto::md5::Md5;
use crypto::digest::Digest;

fn main() {
    let mut hasher = Md5::new();
    let mut count = 0;
    let key = "cxdnnyjw";
    for i in 0..std::u64::MAX {
        hasher.input_str(&format!("{}{}", key, i.to_string()));

        let output = hasher.result_str();
        if output.starts_with("00000") {
            if let Some(x) = output.chars().nth(5) {
                print!("{}", x);
            }
            count += 1;
            if count >= 8 {
                break;
            }
        }
        hasher.reset();
    }
}

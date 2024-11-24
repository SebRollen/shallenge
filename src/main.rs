use itertools::Itertools;
use sha2::{Digest, Sha256};

const GEN_ASCII_STR_CHARSET: &[u8; 64] = b"+/\
                0123456789\
                ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                abcdefghijklmnopqrstuvwxyz";

fn main() {
    let hasher = Sha256::new_with_prefix("sebrollen/");
    let mut min_hash = hasher.finalize();
    let hasher = Sha256::new_with_prefix("sebrollen/");
    let now = std::time::Instant::now();
    let iter = GEN_ASCII_STR_CHARSET.into_iter().copied().powerset();
    for cs in iter {
        let mut hasher = hasher.clone();
        let cs_string = std::str::from_utf8(&cs).unwrap();
        hasher.update(cs_string);
        let hash = hasher.finalize();
        if hash < min_hash {
            let hash_str = format!("{:x}", hash);
            println!(
                "{} {} - ({}s)",
                hash_str,
                cs_string,
                now.elapsed().as_secs()
            );
            min_hash = hash;
        }
    }
}

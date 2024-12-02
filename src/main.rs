use rand::Rng;
use rand::distributions::Alphanumeric;
use sha2::{Sha256, Digest};
use sha3::Sha3_384;
use std::env;


fn generate_password(length: usize) -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(length)
        .map(char::from)
        .collect()
}

fn hash_password(password: &str, algorithm: &str) -> String {
    match algorithm {
        "SHA256" => {
            let mut hasher = Sha256::new();
            hasher.update(password);
            format!("{:x}", hasher.finalize())
        },
        "SHA3-384" => {
            let mut hasher = Sha3_384::new();
            hasher.update(password);
            format!("{:x}", hasher.finalize())
        },
        _ => panic!("Unsupported algorithm"),
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: {} <length> <algorithm>", args[0]);
        return;
    }

    let length: usize = args[1].parse().expect("Invalid length");
    let algorithm = &args[2];

    let password = generate_password(length);
    let hashed_password = hash_password(&password, algorithm);

    println!("Generated password: {}", password);
    println!("Hashed password: {}", hashed_password);
}
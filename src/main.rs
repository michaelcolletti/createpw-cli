use rand::Rng;
use rand::distributions::Alphanumeric;
use rand::rngs::OsRng;
use sha2::{Sha256, Digest};
use sha3::Sha3_384;
use std::env;
use std::process;

fn generate_password(length: usize) -> String {
    OsRng
        .sample_iter(&Alphanumeric)
        .take(length)
        .map(char::from)
        .collect()
}

fn hash_password(password: &str, algorithm: &str) -> Result<String, &'static str> {
    match algorithm {
        "SHA256" => {
            let mut hasher = Sha256::new();
            hasher.update(password);
            Ok(format!("{:x}", hasher.finalize()))
        },
        "SHA3-384" => {
            let mut hasher = Sha3_384::new();
            hasher.update(password);
            Ok(format!("{:x}", hasher.finalize()))
        },
        _ => Err("Unsupported algorithm"),
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: {} <length> <algorithm>", args[0]);
        process::exit(1);
    }

    let length: usize = match args[1].parse() {
        Ok(n) if n > 0 => n,
        _ => {
            eprintln!("Invalid length: must be a positive integer");
            process::exit(1);
        }
    };

    let algorithm = &args[2];

    let password = generate_password(length);
    let hashed_password = match hash_password(&password, algorithm) {
        Ok(hash) => hash,
        Err(err) => {
            eprintln!("Error: {}", err);
            process::exit(1);
        }
    };

    println!("Generated password: {}", password);
    println!("Hashed password: {}", hashed_password);
}

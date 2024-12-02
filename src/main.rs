/// This program generates a random password and hashes it using a specified algorithm.
/// 
/// # Usage
/// 
/// ```sh
/// createpw-cli <length> <algorithm>
/// ```
/// 
/// - `<length>`: The length of the password to generate (must be a positive integer).
/// - `<algorithm>`: The hashing algorithm to use (either "SHA256" or "SHA3-384").
/// 
/// # Example
/// 
/// ```sh
/// createpw-cli 16 SHA256
/// ```
/// 
/// This will generate a random password of length 16 and hash it using the SHA256 algorithm.
/// 
/// # Functions
/// 
/// - `generate_password(length: usize) -> String`: Generates a random password of the specified length using alphanumeric characters.
/// - `hash_password(password: &str, algorithm: &str) -> String`: Hashes the given password using the specified algorithm ("SHA256" or "SHA3-384").
/// 
/// # Panics
/// 
/// The program will panic if:
/// - The length argument is not a valid positive integer.
/// - The specified algorithm is not supported.
/// 
/// # Errors
/// 
/// The program will print an error message and exit if the number of arguments is incorrect.
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